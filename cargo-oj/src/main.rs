use std::fs;
use std::io::Write;
use std::process::{Command, Stdio};
use std::collections::{HashSet, VecDeque};
use serde_json::Value;
use syn::visit_mut::{self, VisitMut};
use syn::spanned::Spanned;
use proc_macro2::Span;
use tokio::runtime::Runtime;
use tokio::io::AsyncWriteExt;
use regex::Regex;
use once_cell::sync::Lazy;

fn main() {
    // Open the root module `src/lib.rs` and its children, and merge into one string
    let source = load_recursive_simple("src/lib");

    // Run rustc to get `dead_code` warnings
    let deadcodes = rustc_check_deadcode(&source);
    // Parse the source into a `syn` AST and remove dead codes
    // which are module-level functions and impl items
    let mut ast = syn::parse_file(&source).expect("Failed to parse as Rust source code.");
    remove_deadcodes(&mut ast, &deadcodes, false);

    // Bruteforce removing items one by one
    let source = prettyplease::unparse(&ast);
    let bleached = try_remove_one_item(&source);

    // Remove dead code again, allowing structs and enums to be removed this time
    let deadcodes = rustc_check_deadcode(&bleached);
    let mut ast = reparse(&bleached);
    remove_deadcodes(&mut ast, &deadcodes, true);
    let source = prettyplease::unparse(&ast);
    // Write out the final result
    let _ = fs::create_dir("src/bin"); // Create directory if not exists, do nothing otherwise
    fs::write("src/bin/main.rs", &source).expect("Failed to write to the file src/bin/main.rs.");
}

// A helper to replace sections of code with spaces ("bleach")
#[derive(Clone)]
struct Src {
    src: Vec<u8>,
}

impl Src {
    fn new(src: &str) -> Self {
        let src_bytes = src.as_bytes().to_vec();
        Self {
            src: src_bytes,
        }
    }

    fn bleach(&mut self, span: (usize, usize)) {
        self.src[span.0..span.1].fill(32);
    }

    fn src_str(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(&self.src) }
    }
}

fn try_remove_one_item(src: &str) -> String {
    fn inner(src: &str) -> Result<String, Box<dyn std::error::Error>> {
        let rt = Runtime::new()?;
        rt.block_on(async {
            let mut src2 = Src::new(src);
            let syn_file = syn::parse_file(src)?;
            let positions = item_positions(&syn_file);
            let mut spans = positions.iter().map(|x| x.1).collect::<VecDeque<_>>();
            let mut futures = VecDeque::new();
            let mut failed = VecDeque::new();
            // Test one-item deletions sequentially and cyclically until a whole cycle fails
            loop {
                if futures.len() < 5 && !spans.is_empty() {
                    let span = spans.pop_front().unwrap();
                    let mut modified_src = src2.clone();
                    modified_src.bleach(span);
                    futures.push_back(tokio::spawn(async move {
                        let success = rustc_check_success_async(modified_src.src_str()).await;
                        (span, success)
                    }));
                } else if !futures.is_empty() {
                    let (span, success) = futures.pop_front().unwrap().await?;
                    if success {
                        src2.bleach(span);
                        spans.append(&mut failed);
                    } else {
                        failed.push_back(span);
                    }
                } else {
                    break;
                }
            }
            Ok(src2.src_str().to_owned())
        })
    }
    inner(src).unwrap()
}

fn span_to_bytes(span: Span) -> (usize, usize) {
    let s = format!("{:?}", span);
    let s = s.trim_matches(|c: char| !c.is_ascii_digit());
    let words = s.split("..").map(|word| word.parse().unwrap()).collect::<Vec<_>>();
    (words[0], words[1])
}

fn offset(span: (usize, usize), start: usize) -> (usize, usize) {
    (span.0 - start, span.1 - start)
}

fn item_positions(root: &syn::File) -> Vec<(Vec<usize>, (usize, usize))> {
    // Extract positions of mod-level items that are trait defs, and impl blocks
    // No mods because it will be removed at next dead_code pass
    let root_span = span_to_bytes(root.span());
    let mut positions = vec![];
    let mut pos_items = Vec::new();
    for (i, item) in root.items.iter().enumerate() {
        match item {
            syn::Item::Mod(_) | syn::Item::Trait(_) | syn::Item::Impl(_) => {
                pos_items.push((vec![i], item));
            }
            _ => {}
        }
    }
    while let Some((pos, item)) = pos_items.pop() {
        match item {
            syn::Item::Mod(syn::ItemMod{content: Some((_, items)), ..}) => {
                for (i, item) in items.iter().enumerate() {
                    match item {
                        syn::Item::Mod(_) | syn::Item::Trait(_) | syn::Item::Impl(_) => {
                            let mut next_pos = pos.clone();
                            next_pos.push(i);
                            pos_items.push((next_pos, item));
                        }
                        _ => {}
                    }
                }
            }
            syn::Item::Trait(_) | syn::Item::Impl(_) => {
                let span = offset(span_to_bytes(item.span()), root_span.0);
                positions.push((pos, span));
            }
            _ => {}
        }
    }
    positions
}

async fn rustc_check_success_async(source: &str) -> bool {
    async fn inner(source: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let mut rustc_check = tokio::process::Command::new("rustc");
        rustc_check
            .args(["--emit=mir", "--edition=2021", "--out-dir=/tmp/ramdisk", "-"])
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null());
        let mut child = rustc_check.spawn().expect("Failed to run rustc.");
        let mut stdin = child.stdin.take().expect("Failed to get stdin handle of rustc.");
        stdin.write_all(source.as_bytes()).await.expect("Failed to write to rustc's stdin.");
        drop(stdin); // Signal EOF
        let status = child.wait().await?;
        Ok(status.success())
    }
    inner(source).await.unwrap()
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct RawIdent {
    name: String,
    line_start: u64,
    column_start: u64,
    line_end: u64,
    column_end: u64,
}

impl RawIdent {
    fn new(name: &str, line_start: u64, column_start: u64, line_end: u64, column_end: u64) -> Self {
        Self {
            name: name.to_owned(),
            line_start,
            column_start,
            line_end,
            column_end,
        }
    }
}

impl From<&syn::Ident> for RawIdent {
    fn from(value: &syn::Ident) -> Self {
        Self {
            name: format!("{}", value),
            line_start: value.span().start().line as u64,
            column_start: value.span().start().column as u64,
            line_end: value.span().end().line as u64,
            column_end: value.span().end().column as u64,
        }
    }
}

fn remove_deadcodes(file: &mut syn::File, deadcodes: &HashSet<RawIdent>, remove_structs: bool) {
    let mut deleter = DeadCodeRemover::new(deadcodes, remove_structs);
    deleter.visit_file_mut(file);
}

// A mut visitor on syn AST to remove indicated items
struct DeadCodeRemover {
    items: HashSet<RawIdent>,
    remove_structs: bool,
}

impl DeadCodeRemover {
    fn new(deadcodes: &HashSet<RawIdent>, remove_structs: bool) -> Self {
        Self {
            items: deadcodes.clone(),
            remove_structs
        }
    }
}

fn item_raw_ident(item: &syn::Item, remove_structs: bool) -> Option<RawIdent> {
    match item {
        // const, fn, type: always remove
        | syn::Item::Const(syn::ItemConst{ident, ..})
        | syn::Item::Fn(syn::ItemFn{sig: syn::Signature{ident, ..}, ..})
        | syn::Item::Type(syn::ItemType{ident, ..}) => Some(ident.into()),
        // struct, enum: remove on request
        | syn::Item::Struct(syn::ItemStruct{ident, ..})
        | syn::Item::Enum(syn::ItemEnum{ident, ..}) => Some(ident.into()).filter(|_| remove_structs),
        // rest: ignore
        _ => None
    }
}

fn remove_items(items: &mut Vec<syn::Item>, to_remove: &mut HashSet<RawIdent>, remove_structs: bool) {
    items.retain(|item| {
        if let Some(raw_ident) = item_raw_ident(item, remove_structs) {
            !to_remove.remove(&raw_ident)
        } else { true }
    });
}

fn remove_empty_submodules(items: &mut Vec<syn::Item>) {
    items.retain(|item| {
        if let syn::Item::Mod(module) = item {
            if let Some((_, ref items)) = module.content {
                !items.is_empty()
            } else { true }
        } else { true }
    });
}

impl VisitMut for DeadCodeRemover {
    fn visit_file_mut(&mut self, i: &mut syn::File) {
        remove_items(&mut i.items, &mut self.items, self.remove_structs);
        visit_mut::visit_file_mut(self, i);
        remove_empty_submodules(&mut i.items);
    }
    fn visit_item_mod_mut(&mut self, i: &mut syn::ItemMod) {
        // At each module, scan over its child items and remove matching items
        if let Some((_, ref mut items)) = i.content {
            remove_items(items, &mut self.items, self.remove_structs);
        }
        visit_mut::visit_item_mod_mut(self, i);
        // After all children (including child modules) are visited, check for empty modules
        if let Some((_, ref mut items)) = i.content {
            remove_empty_submodules(items);
        }
    }
    fn visit_item_impl_mut(&mut self, i: &mut syn::ItemImpl) {
        // Remove associated functions detected as dead code
        i.items.retain(|item| {
            if let syn::ImplItem::Method(syn::ImplItemMethod{sig: syn::Signature{ident, ..}, ..}) = item {
                let raw_ident = RawIdent::from(ident);
                !self.items.remove(&raw_ident)
            } else { true }
        });
        visit_mut::visit_item_impl_mut(self, i);
    }
}

fn rustc_check_deadcode(source: &str) -> HashSet<RawIdent> {
    let mut rustc_check = Command::new("rustc");
    rustc_check
        .args(["--emit=mir", "--edition", "2021", "--error-format", "json", "--out-dir=/tmp/ramdisk", "-"])
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::piped());
    let child = rustc_check.spawn().expect("Failed to run rustc.");
    let mut stdin = child.stdin.as_ref().expect("Failed to get stdin handle of rustc.");
    stdin.write_all(source.as_bytes()).expect("Failed to write to rustc's stdin.");
    let rustc_check_stdout = child.wait_with_output().expect("Failed to open rustc's stdout.");
    let rustc_check_output = String::from_utf8(rustc_check_stdout.stderr).unwrap();
    let mut deadcodes = HashSet::new();
    for line in rustc_check_output.lines() {
        let Ok(obj) = line.parse::<Value>() else { continue; };
        if obj.pointer("/code/code") != Some(&Value::from("dead_code")) { continue; }
        if let (Some(span), Some(Value::String(message))) = (obj.pointer("/spans/0"), obj.pointer("/message")) {
            let line_start = span.pointer("/line_start").unwrap().as_u64().unwrap();
            let line_end = span.pointer("/line_end").unwrap().as_u64().unwrap();
            let column_start = span.pointer("/column_start").unwrap().as_u64().unwrap();
            let column_end = span.pointer("/column_end").unwrap().as_u64().unwrap();
            let mut split = message.split('`');
            let _item_kind = split.next().unwrap().trim_end();
            let item_name = split.next().unwrap();
            deadcodes.insert(RawIdent::new(item_name, line_start, column_start-1, line_end, column_end-1));
        }
    }
    deadcodes
}

fn reparse(src: &str) -> syn::File {
    let syntax = syn::parse_file(src);
    syntax.expect("Unexpected error during reparsing prettified file.")
}

fn load_recursive(path: &str) -> syn::File {
    // Supports only standard module path structure (lib.rs, m1.rs, m1/m2.rs).
    // Not supported:
    // - mod.rs style module path
    // - custom path attributes
    // - non-inline modules inside inline modules

    // Try <path>.rs unless the current path is the src root ("src/lib")
    let file_path = if path == "src/lib" { "src/lib.rs".to_owned() } else { format!("{}.rs", path) };
    let file = fs::read_to_string(&file_path);
    let src = file.unwrap_or_else(|_| {
        if path == "src/lib" {
            eprintln!("Failed to read the file {}.", file_path);
            eprintln!("Please make sure to run at the crate root of a lib crate, and the crate builds correctly.");
        } else {
            eprintln!("Failed to read the file {}.", file_path);
            eprintln!("Please make sure that the crate builds correctly.");
        }
        panic!()
    });
    // Parse the current file
    let syntax = syn::parse_file(&src);
    let mut syntax = syntax.unwrap_or_else(|_| {
        eprintln!("Failed to parse the file {}.", file_path);
        eprintln!("Please make sure the crate builds correctly.");
        panic!()
    });

    // If the current file is root, remove #![allow(dead_code)]
    syntax.attrs.retain(|attr| {
        if !attr.path.is_ident("allow") { return true; }
        if let Ok(ident) = attr.parse_args::<syn::Ident>() {
            if ident == "dead_code" { return false; }
        }
        true
    });

    // Extract `mod` items without content, and try to recursively fill in the content
    for root_item in &mut syntax.items {
        match root_item {
            syn::Item::Mod(itemmod@syn::ItemMod{content: None, ..}) => {
                let modname = format!("{}", itemmod.ident);
                let inner_file = load_recursive(&child_path(path, &modname));
                let syn::File { attrs, items, .. } = inner_file;
                itemmod.content = Some((syn::token::Brace(proc_macro2::Span::call_site()), items));
                itemmod.attrs.extend_from_slice(&attrs);
            }
            _ => {},
        }
    }
    syntax
}

fn load_recursive_simple(path: &str) -> String {
    // Supports only standard module path structure (lib.rs, m1.rs, m1/m2.rs).
    // Not supported:
    // - mod.rs style module path
    // - custom path attributes
    // - non-inline modules inside inline modules
    // - mod declaration in comments

    // Try <path>.rs unless the current path is the src root ("src/lib")
    let file_path = if path == "src/lib" { "src/lib.rs".to_owned() } else { format!("{}.rs", path) };
    let file = fs::read_to_string(&file_path);
    let src = file.unwrap_or_else(|_| {
        if path == "src/lib" {
            eprintln!("Failed to read the file {}.", file_path);
            eprintln!("Please make sure to run at the crate root of a lib crate, and the crate builds correctly.");
        } else {
            eprintln!("Failed to read the file {}.", file_path);
            eprintln!("Please make sure that the crate builds correctly.");
        }
        panic!()
    });
    // Search the current file for `mod<whitespace><ident><whitespace>;`
    static MOD_REGEX: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"mod[ \n\t]+([a-z][a-z0-9_]*)[ \n\t]*;").unwrap()
    });
    let modified_src = MOD_REGEX.replace_all(&src, |cap: &regex::Captures| {
        let mod_statement = &cap[0usize];
        let mod_name = &cap[1usize];
        let inner_file = load_recursive_simple(&child_path(path, mod_name));
        format!("{}{{{}}}", mod_statement.trim_end_matches(";"), inner_file)
    });
    modified_src.replacen("#![allow(dead_code)]", "", if path == "src/lib" { 1 } else { 0 })
}

fn child_path(path: &str, module: &str) -> String {
    if path == "src/lib" { "src/".to_owned() + module }
    else { path.to_owned() + "/" + module }
}
