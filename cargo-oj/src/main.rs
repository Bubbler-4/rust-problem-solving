use std::fs;
use std::io::Write;
use std::process::{Command, Stdio};
use std::collections::{HashSet, VecDeque};
use serde_json::Value;
use syn::visit_mut::{self, VisitMut};
use syn::spanned::Spanned;
use proc_macro2::Span;
use tokio::runtime;
use tokio::io::AsyncWriteExt;

fn main() {
    // Open the root module `src/lib.rs` and its children, and merge into one string
    let file = load_recursive("src/lib");
    let source = prettyplease::unparse(&file);

    // Run rustc to get unused warnings
    let unused = rustc_check_unused(&source);
    // Parse the source into a `syn` AST and remove unused codes
    let mut ast = syn::parse_file(&source).expect("Failed to parse as Rust source code.");
    remove_unused(&mut ast, &unused, &source);

    // Bruteforce removing items one by one
    let source = prettyplease::unparse(&ast);
    let bleached = try_remove_one_item(&source);

    // Remove unused code again
    let unused = rustc_check_unused(&bleached);
    let mut ast = reparse(&bleached);
    remove_unused(&mut ast, &unused, &source);
    let source = prettyplease::unparse(&ast);

    // Replace leading indents with tabs
    let mut final_source = String::new();
    for line in source.lines() {
        let leading_spaces = line.bytes().take_while(|&b| b == b' ').count();
        final_source.push_str(&"\t".repeat(leading_spaces / 4));
        final_source.push_str(&line[leading_spaces / 4 * 4..]);
        final_source.push('\n');
    }

    // Write out the final result
    let _ = fs::create_dir("src/bin"); // Create directory if not exists, do nothing otherwise
    fs::write("src/bin/main.rs", &final_source).expect("Failed to write to the file src/bin/main.rs.");
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
        // let rt = Runtime::new()?;
        let rt = runtime::Builder::new_current_thread().enable_io().build()?;
        rt.block_on(async {
            let char_indices = src.char_indices().map(|(i, _)| i).collect::<Vec<_>>();
            let mut src2 = Src::new(src);
            let syn_file = syn::parse_file(src)?;
            let positions = item_positions(&syn_file);
            let mut spans = positions.iter().map(|&(_, (x, y))| (char_indices[x], char_indices[y])).collect::<VecDeque<_>>();
            let mut futures = VecDeque::new();
            let mut failed = VecDeque::new();
            // Test one-item deletions sequentially and cyclically until a whole cycle fails
            let mut after_success_counter = 0usize;
            loop {
                if futures.len() < 4 && !spans.is_empty() {
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
                        after_success_counter = 3;
                    } else if after_success_counter > 0 {
                        spans.push_back(span);
                        after_success_counter -= 1;
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

fn mod_is_cfg_test(module: &syn::ItemMod) -> bool {
    module.attrs.iter().any(|attr| {
        let meta = &attr.meta;
        if let syn::Meta::List(metalist) = meta {
            if !metalist.path.is_ident("cfg") { return false; }
            if let Ok(ident) = attr.parse_args::<syn::Ident>() {
                return format!("{}", ident) == "test";
            }
            false
        } else {
            false
        }
    })
}

fn item_positions(root: &syn::File) -> Vec<(Vec<usize>, (usize, usize))> {
    // Extract positions of:
    //   - mod-level items that are trait defs, and impl blocks
    //   - impl blocks
    //   - attributes on structs and enums (for #[derive(...)])
    //   - macros
    // No mods because it will be removed at next dead_code pass
    let root_span = span_to_bytes(root.span());
    let mut positions = vec![];
    let mut pos_items = Vec::new();
    for (i, item) in root.items.iter().enumerate() {
        match item {
            syn::Item::Mod(_) | syn::Item::Trait(_) | syn::Item::Impl(_) | syn::Item::Macro(_) => {
                pos_items.push((vec![i], item));
            }
            syn::Item::Struct(syn::ItemStruct { attrs, .. }) | syn::Item::Enum(syn::ItemEnum { attrs, .. }) => {
                for attr in attrs {
                    let span = offset(span_to_bytes(attr.span()), root_span.0);
                    positions.push((vec![], span));
                }
            }
            _ => {}
        }
    }
    while let Some((pos, item)) = pos_items.pop() {
        match item {
            syn::Item::Mod(module) if mod_is_cfg_test(module) => {
                let span = offset(span_to_bytes(item.span()), root_span.0);
                positions.push((pos, span));
            }
            syn::Item::Mod(syn::ItemMod{content: Some((_, items)), ..}) => {
                for (i, item) in items.iter().enumerate() {
                    match item {
                        syn::Item::Mod(_) | syn::Item::Trait(_) | syn::Item::Impl(_) | syn::Item::Macro(_) => {
                            let mut next_pos = pos.clone();
                            next_pos.push(i);
                            pos_items.push((next_pos, item));
                        }
                        syn::Item::Struct(syn::ItemStruct { attrs, .. }) | syn::Item::Enum(syn::ItemEnum { attrs, .. }) => {
                            for attr in attrs {
                                let span = offset(span_to_bytes(attr.span()), root_span.0);
                                positions.push((pos.clone(), span));
                            }
                        }
                        _ => {}
                    }
                }
            }
            syn::Item::Trait(_) | syn::Item::Impl(_) | syn::Item::Macro(_) => {
                let span = offset(span_to_bytes(item.span()), root_span.0);
                positions.push((pos, span));
            }
            _ => {}
        }
    }
    positions
}

// async fn rustc_check_success_async0(source: &str) -> bool {
//     async fn inner(source: &str) -> Result<bool, Box<dyn std::error::Error>> {
//         let mut rustc_check = tokio::process::Command::new("rustc");
//         rustc_check
//             .args(["--emit=mir", "--edition=2021", "--out-dir=/tmp/ramdisk", "-"])
//             .stdin(Stdio::piped())
//             .stdout(Stdio::null())
//             .stderr(Stdio::null());
//         let mut child = rustc_check.spawn().expect("Failed to run rustc.");
//         let mut stdin = child.stdin.take().expect("Failed to get stdin handle of rustc.");
//         stdin.write_all(source.as_bytes()).await.expect("Failed to write to rustc's stdin.");
//         drop(stdin); // Signal EOF
//         let status = child.wait().await?;
//         Ok(status.success())
//     }
//     inner(source).await.unwrap()
// }

async fn rustc_check_success_async(source: &str) -> bool {
    async fn inner(source: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let mut rustc_check = tokio::process::Command::new("rustup");
        rustc_check
            .args(["run", "nightly", "cargo-oj-internal-typeck"])
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

// byte offset start, end
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct RawSpan(usize, usize);

impl<T> From<T> for RawSpan where T: syn::spanned::Spanned {
    fn from(value: T) -> Self {
        let span = span_to_bytes(value.span());
        Self(span.0, span.1)
    }
}

// A mut visitor on syn AST to remove indicated items
struct UnusedRemover {
    items: HashSet<RawSpan>,
    start: usize,
    char_indices: Vec<usize>,
}

impl UnusedRemover {
    fn new(file: &syn::File, deadcodes: &HashSet<RawSpan>, source: &str) -> Self {
        Self {
            items: deadcodes.clone(),
            start: RawSpan::from(file).0,
            char_indices: source.char_indices().map(|(i, _)| i).collect()
        }
    }
}

fn remove_unused(file: &mut syn::File, unused: &HashSet<RawSpan>, source: &str) {
    let mut deleter = UnusedRemover::new(file, unused, source);
    deleter.visit_file_mut(file);
}

// is current tree a single path (path* (name | rename | glob))?
fn is_single_use_path(tree: &syn::UseTree) -> bool {
    match tree {
        syn::UseTree::Path(syn::UsePath{tree, ..}) => is_single_use_path(tree),
        syn::UseTree::Name(_) => true,
        syn::UseTree::Rename(_) => true,
        syn::UseTree::Glob(_) => true,
        syn::UseTree::Group(_) => false,
    }
}

impl UnusedRemover {
    fn node_span<T>(&self, node: &T) -> RawSpan where T: syn::spanned::Spanned {
        let span = offset(span_to_bytes(node.span()), self.start);
        RawSpan(self.char_indices[span.0], self.char_indices[span.1])
    }

    // if current tree is single path, test on entire path directly
    // otherwise recurse
    // if current tree is group, recurse over all children and remove deleted ones.
    // the node survives if it is still nonempty
    fn use_should_survive(&mut self, tree: &mut syn::UseTree) -> bool {
        if is_single_use_path(tree) {
            let span = self.node_span(tree);
            return !self.items.remove(&span);
        }
        match tree {
            syn::UseTree::Path(syn::UsePath{tree, ..}) => self.use_should_survive(tree),
            syn::UseTree::Group(syn::UseGroup{items, ..}) => {
                let mut v = vec![];
                while let Some(pair) = items.pop() {
                    let mut subtree = pair.into_value();
                    if self.use_should_survive(&mut subtree) {
                        v.push(subtree);
                    }
                }
                while let Some(subtree) = v.pop() {
                    items.push(subtree);
                }
                !items.is_empty()
            },
            _ => unreachable!()
        }
    }
    
    // if cur item is use, recursively remove individual imports and empty groups,
    // and return true if the use item itself should survive, false otherwise
    // else if fn/const/type/struct/enum, return true if it should survive based on its ident
    // do not touch other kinds of items (which means they always survive)
    fn should_survive(&mut self, item: &mut syn::Item) -> bool {
        match item {
            | syn::Item::Const(syn::ItemConst{ident, ..})
            | syn::Item::Fn(syn::ItemFn{sig: syn::Signature{ident, ..}, ..})
            | syn::Item::Type(syn::ItemType{ident, ..})
            | syn::Item::Struct(syn::ItemStruct{ident, ..})
            | syn::Item::Enum(syn::ItemEnum{ident, ..}) => !self.items.remove(&self.node_span(ident)),
            syn::Item::Use(syn::ItemUse{tree, ..}) => self.use_should_survive(tree),
            _ => true,
        }
    }
    
    fn remove_items2(&mut self, items: &mut Vec<syn::Item>) {
        items.retain_mut(|item| {
            self.should_survive(item)
        });
    }
}

fn remove_empty_items(items: &mut Vec<syn::Item>) {
    items.retain(|item| {
        if let syn::Item::Mod(module) = item {
            if let Some((_, ref items)) = module.content {
                !items.is_empty()
            } else { true }
        } else if let syn::Item::Impl(syn::ItemImpl{items, trait_, ..}) = item {
            trait_.is_some() || !items.is_empty()
        } else { true }
    });
}

impl VisitMut for UnusedRemover {
    fn visit_file_mut(&mut self, i: &mut syn::File) {
        self.remove_items2(&mut i.items);
        visit_mut::visit_file_mut(self, i);
        remove_empty_items(&mut i.items);
    }
    fn visit_item_mod_mut(&mut self, i: &mut syn::ItemMod) {
        // At each module, scan over its child items and remove matching items
        if let Some((_, ref mut items)) = i.content {
            self.remove_items2(items);
        }
        visit_mut::visit_item_mod_mut(self, i);
        // After all children (including child modules) are visited, check for empty modules
        if let Some((_, ref mut items)) = i.content {
            remove_empty_items(items);
        }
    }
    fn visit_item_impl_mut(&mut self, i: &mut syn::ItemImpl) {
        // Remove associated functions and constants detected as dead code
        i.items.retain(|item| {
            match item {
                | syn::ImplItem::Fn(syn::ImplItemFn{sig: syn::Signature{ident, ..}, ..})
                | syn::ImplItem::Const(syn::ImplItemConst{ident, ..}) => !self.items.remove(&self.node_span(ident)),
                _ => true
            }
        });
        visit_mut::visit_item_impl_mut(self, i);
    }
}

// fn rustc_check_unused0(source: &str) -> HashSet<RawSpan> {
//     let mut rustc_check = Command::new("rustc");
//     rustc_check
//         .args(["--emit=mir", "--edition", "2021", "--error-format", "json", "--out-dir=/tmp/ramdisk", "-"])
//         .stdin(Stdio::piped())
//         .stdout(Stdio::null())
//         .stderr(Stdio::piped());
//     let child = rustc_check.spawn().expect("Failed to run rustc.");
//     let mut stdin = child.stdin.as_ref().expect("Failed to get stdin handle of rustc.");
//     stdin.write_all(source.as_bytes()).expect("Failed to write to rustc's stdin.");
//     let rustc_check_stdout = child.wait_with_output().expect("Failed to open rustc's stdout.");
//     let rustc_check_output = String::from_utf8(rustc_check_stdout.stderr).unwrap();
//     let mut unused = HashSet::new();
//     for line in rustc_check_output.lines() {
//         let Ok(obj) = line.parse::<Value>() else { continue; };
//         let warning = obj.pointer("/code/code");
//         if warning == Some(&Value::from("dead_code")) || warning == Some(&Value::from("unused_imports")) {
//             let Some(spans) = obj.pointer("/spans") else { continue; };
//             let Some(spans) = spans.as_array() else { continue; };
//             for span in spans {
//                 let is_primary = span.pointer("/is_primary").unwrap().as_bool().unwrap();
//                 if !is_primary { continue; }
//                 let byte_start = span.pointer("/byte_start").unwrap().as_u64().unwrap() as usize;
//                 let byte_end = span.pointer("/byte_end").unwrap().as_u64().unwrap() as usize;
//                 unused.insert(RawSpan(byte_start, byte_end));
//             }
//         }
//     }
//     unused
// }

fn rustc_check_unused(source: &str) -> HashSet<RawSpan> {
    let mut rustc_check = Command::new("rustup");
    rustc_check
        .args(["run", "nightly", "cargo-oj-internal-unused"])
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::piped());
    let child = rustc_check.spawn().expect("Failed to run rustc.");
    let mut stdin = child.stdin.as_ref().expect("Failed to get stdin handle of rustc.");
    stdin.write_all(source.as_bytes()).expect("Failed to write to rustc's stdin.");
    let rustc_check_stdout = child.wait_with_output().expect("Failed to open rustc's stdout.");
    let rustc_check_output = String::from_utf8(rustc_check_stdout.stderr).unwrap();
    let mut unused = HashSet::new();
    for line in rustc_check_output.lines() {
        let Ok(obj) = line.parse::<Value>() else { continue; };
        let warning = obj.pointer("/code/code");
        if warning == Some(&Value::from("dead_code")) || warning == Some(&Value::from("unused_imports")) {
            let Some(spans) = obj.pointer("/spans") else { continue; };
            let Some(spans) = spans.as_array() else { continue; };
            for span in spans {
                let is_primary = span.pointer("/is_primary").unwrap().as_bool().unwrap();
                if !is_primary { continue; }
                let byte_start = span.pointer("/byte_start").unwrap().as_u64().unwrap() as usize;
                let byte_end = span.pointer("/byte_end").unwrap().as_u64().unwrap() as usize;
                unused.insert(RawSpan(byte_start, byte_end));
            }
        }
    }
    unused
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
        if !attr.path().is_ident("allow") { return true; }
        if let Ok(ident) = attr.parse_args::<syn::Ident>() {
            if ident == "dead_code" || ident == "unused_imports" { return false; }
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

fn child_path(path: &str, module: &str) -> String {
    if path == "src/lib" { "src/".to_owned() + module }
    else { path.to_owned() + "/" + module }
}
