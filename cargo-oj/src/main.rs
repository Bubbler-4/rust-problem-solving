use std::fs;
use std::io;
use std::io::Write;
use std::process::{Command, Stdio};
use std::collections::{HashSet, VecDeque, HashMap};
use serde_json::Value;
use syn::visit_mut::{self, VisitMut};
use syn::spanned::Spanned;
use proc_macro2::Span;
use rayon::prelude::*;
use tokio::runtime::Runtime;
use tokio::io::AsyncWriteExt;
use futures::stream::FuturesUnordered;
use futures::StreamExt;
use regex::Regex;
use once_cell::sync::Lazy;

fn main() {
    // Open and parse the root file `src/lib.rs` from the current crate
    let root_syntax = load_recursive_simple("src/lib");
    // Prettyprint it into `src/bin/tmp.rs`
    let _ = fs::create_dir("src/bin"); // Create directory if not exists, do nothing otherwise
    // let unparsed = prettyplease::unparse(&root_syntax);
    let unparsed = root_syntax;
    // fs::write("src/bin/tmp.rs", &unparsed).expect("Failed to write to the file src/bin/tmp.rs.");
    // Run cargo check on the resulting tmp
    // let deadcodes = cargo_check_deadcode();
    let deadcodes = rustc_check_deadcode(&unparsed);
    // Reparsing is necessary to produce correct spans for items
    let mut reparsed = reparse(&unparsed);
    // fs::remove_file("src/bin/tmp.rs").expect("Failed to remove file src/bin/tmp.rs.");
    remove_deadcodes(&mut reparsed, &deadcodes, false);

    // Bruteforce removing items one by one
    // try_remove_one_item(&mut reparsed);
    let unparsed = prettyplease::unparse(&reparsed);
    let bleached = try_remove_one_item6(&unparsed);
    // Remove dead code again, allowing structs and enums to be removed this time
    let deadcodes = rustc_check_deadcode(&bleached);
    let mut reparsed = reparse(&bleached);
    remove_deadcodes(&mut reparsed, &deadcodes, true);
    let unparsed = prettyplease::unparse(&reparsed);
    // Write out the final result
    fs::write("src/bin/main.rs", &unparsed).expect("Failed to write to the file src/bin/main.rs.");
}

fn main2() {
    // Open and parse the root file `src/lib.rs` from the current crate
    let root_syntax = load_recursive_simple("src/lib");
    // Prettyprint it into `src/bin/tmp.rs`
    let _ = fs::create_dir("src/bin"); // Create directory if not exists, do nothing otherwise
    // let unparsed = prettyplease::unparse(&root_syntax);
    let unparsed = root_syntax;
    // fs::write("src/bin/tmp.rs", &unparsed).expect("Failed to write to the file src/bin/tmp.rs.");
    // Run cargo check on the resulting tmp
    // let deadcodes = cargo_check_deadcode();
    let deadcodes = rustc_check_deadcode(&unparsed);
    // Reparsing is necessary to produce correct spans for items
    let mut reparsed = reparse(&unparsed);
    // fs::remove_file("src/bin/tmp.rs").expect("Failed to remove file src/bin/tmp.rs.");
    remove_deadcodes(&mut reparsed, &deadcodes, false);

    // Bruteforce removing items one by one
    // try_remove_one_item(&mut reparsed);
    let unparsed = prettyplease::unparse(&reparsed);
    let bleached = try_remove_one_item4(&unparsed);
    let reparsed = reparse(&bleached);
    let unparsed = prettyplease::unparse(&reparsed);
    // Write out the final result
    fs::write("src/bin/main.rs", &unparsed).expect("Failed to write to the file src/bin/main.rs.");
}

fn _main() {
    // Open and parse the root file `src/lib.rs` from the current crate
    let root_syntax = load_recursive("src/lib");
    // Prettyprint it into `src/bin/tmp.rs`
    // let unparsed = prettyplease::unparse(&root_syntax);
    // fs::write("src/bin/tmp.rs", &unparsed).expect("Failed to write to the file src/bin/tmp.rs.");
    // Run cargo check on the resulting tmp
    // let deadcodes = cargo_check_deadcode();
    // Reparsing is necessary to produce correct spans for items
    let mut reparsed = root_syntax;
    // fs::remove_file("src/bin/tmp.rs").expect("Failed to remove file src/bin/tmp.rs.");
    // remove_deadcodes(&mut reparsed, &deadcodes);

    // Bruteforce removing items one by one
    try_remove_one_item(&mut reparsed);
    let unparsed = prettyplease::unparse(&reparsed);
    // Write out the final result
    let _ = fs::create_dir("src/bin"); // Create directory if not exists, do nothing otherwise
    fs::write("src/bin/main.rs", &unparsed).expect("Failed to write to the file src/bin/main.rs.");
}

// utility to avoid repeated unparse calls
#[derive(Clone)]
struct Src {
    src: Vec<u8>,
    syn_file: syn::File,
}

impl Src {
    fn new(src: &str) -> Self {
        let src_bytes = src.as_bytes().to_vec();
        let syn_file = syn::parse_file(src).unwrap();
        Self {
            src: src_bytes,
            syn_file,
        }
    }

    fn bleach(&mut self, span: (usize, usize)) {
        self.src[span.0..span.1].fill(32);
    }

    fn src_str(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(&self.src) }
    }
}

fn try_remove_one_item(root: &mut syn::File) {
    loop {
        let mut removed_prefixes: Vec<Vec<usize>> = vec![];
        let positions = item_positions(root);
        for pos in positions {
            if removed_prefixes.iter().any(|prefix| pos.starts_with(prefix)) {
                continue;
            }
            println!("{:?}", pos);
            let mut modified_root = root.clone();
            remove_position(&mut modified_root, &pos);
            let unparsed = prettyplease::unparse(&modified_root);
            if rustc_check_success2(&unparsed) {
                std::mem::swap(root, &mut modified_root);
                // println!("{}", unparsed);
                removed_prefixes.push(pos);
            }
        }
        if removed_prefixes.is_empty() { break; }
    }
}

fn try_remove_one_item2(src: &str) -> String {
    // println!("{}", src);
    let mut src = Src::new(src);
    let mut positions = VecDeque::from(item_positions2(&src.syn_file));
    // println!("{:?}", positions);
    let mut removed_prefixes: Vec<Vec<usize>> = vec![];
    let mut processed = VecDeque::new();
    while let Some((pos, span)) = positions.pop_front() {
        if removed_prefixes.iter().any(|prefix| pos.starts_with(prefix)) {
            continue;
        }
        println!("{:?}", pos);
        let mut modified_src = src.clone();
        modified_src.bleach(span);
        if rustc_check_success(modified_src.src_str()) {
            std::mem::swap(&mut src, &mut modified_src);
            removed_prefixes.push(pos);
            positions.append(&mut processed);
        } else {
            processed.push_back((pos, span));
        }
    }
    src.src_str().to_owned()
}

// utility to avoid repeated unparse calls
#[derive(Clone)]
struct Src2 {
    src: Vec<u8>,
}

impl Src2 {
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

fn try_remove_one_item3(src: &str) -> String {
    // println!("{}", src);
    let mut src2 = Src2::new(src);
    let syn_file = syn::parse_file(src).unwrap();
    let mut positions = item_positions2(&syn_file).into_iter().map(|x| x.1).collect::<Vec<_>>();
    loop {
        let (can_be_removed, rest): (Vec<_>, Vec<_>) = positions.into_par_iter().partition(|&span| {
            let mut modified_src = src2.clone();
            modified_src.bleach(span);
            rustc_check_success(modified_src.src_str())
        });
        positions = rest;
        if can_be_removed.is_empty() { break; }
        for span in can_be_removed {
            src2.bleach(span);
        }
    }
    src2.src_str().to_owned()
}

fn try_remove_one_item4(src: &str) -> String {
    fn inner(src: &str) -> Result<String, Box<dyn std::error::Error>> {
        let rt = Runtime::new()?;
        rt.block_on(async {
            let mut src2 = Src2::new(src);
            let syn_file = syn::parse_file(src)?;
            let positions = item_positions2(&syn_file);
            let mut pos_hash = positions.iter().map(|x| (x.1, &x.0)).collect::<HashMap<_,_>>();
            let mut futures = FuturesUnordered::new();
            let empty = vec![];
            let mut modified = true;
            while modified {
                modified = false;
                futures.push(tokio::spawn(async {
                    ((0usize, 0usize), false)
                }));
                while let Some(Ok((span, can_delete))) = futures.next().await {
                    let pos = pos_hash.get(&span).copied().unwrap_or(&empty);
                    if can_delete {
                        src2.bleach(span);
                        modified = true;
                        pos_hash.remove(&span); // effectively removes the subtree
                    } else {
                        // for each child of pos, spawn a tokio task that checks for compilability
                        for (next_span, next_pos) in &pos_hash {
                            if next_pos.starts_with(&pos) && next_pos.len() == pos.len() + 1 {
                                let next_span = *next_span;
                                let mut modified_src = src2.clone();
                                modified_src.bleach(next_span);
                                futures.push(tokio::spawn(async move {
                                    let success = rustc_check_success_async(modified_src.src_str()).await;
                                    (next_span, success)
                                }));
                            }
                        }
                    }
                }
            }
            Ok(src2.src_str().to_owned())
        })
    }
    inner(src).unwrap()
}

fn try_remove_one_item5(src: &str) -> String {
    // println!("{}", src);
    let mut src2 = Src2::new(src);
    let syn_file = syn::parse_file(src).unwrap();
    let positions = item_positions2(&syn_file);
    let empty = vec![];
    let mut queue = VecDeque::from(vec![(&empty, &(0, 0), false)]);
    while let Some((pos, span, can_delete)) = queue.pop_front() {
        if can_delete {
            src2.bleach(*span);
        } else {
            // for each child of pos, spawn a tokio task that checks for compilability
            for (next_pos, next_span) in &positions {
                if next_pos.starts_with(&pos) && next_pos.len() == pos.len() + 1 {
                    let mut modified_src = src2.clone();
                    modified_src.bleach(*next_span);
                    let success = rustc_check_success(modified_src.src_str());
                    queue.push_back((next_pos, next_span, success));
                }
            }
        }
    }
    src2.src_str().to_owned()
}

fn try_remove_one_item6(src: &str) -> String {
    fn inner(src: &str) -> Result<String, Box<dyn std::error::Error>> {
        let rt = Runtime::new()?;
        rt.block_on(async {
            let mut src2 = Src2::new(src);
            let syn_file = syn::parse_file(src)?;
            let positions = item_positions3(&syn_file);
            let mut positions = positions.iter().map(|x| x.1).collect::<Vec<_>>();
            // let mut pos_hash = positions.iter().map(|x| (x.1, &x.0)).collect::<HashMap<_,_>>();
            let mut futures = FuturesUnordered::new();
            let mut modified = true;
            while modified {
                modified = false;
                for span in &positions {
                    let span = *span;
                    let mut modified_src = src2.clone();
                    modified_src.bleach(span);
                    println!("{:?}", span);
                    futures.push(tokio::spawn(async move {
                        let success = rustc_check_success_async(modified_src.src_str()).await;
                        (span, success)
                    }));
                }
                let mut remaining = vec![];
                while let Some(Ok((span, can_delete))) = futures.next().await {
                    if can_delete {
                        src2.bleach(span);
                        modified = true;
                    } else {
                        remaining.push(span);
                    }
                }
                positions = remaining;
            }
            Ok(src2.src_str().to_owned())
        })
    }
    inner(src).unwrap()
}

enum Item<'a> {
    Item(&'a syn::Item),
    ImplItem(&'a syn::ImplItem),
}

fn item_positions(root: &syn::File) -> Vec<Vec<usize>> {
    let mut positions = vec![];
    let mut pos_items = Vec::new();
    for (i, item) in root.items.iter().enumerate() {
        if let syn::Item::Fn(syn::ItemFn{sig: syn::Signature{ident, ..}, ..}) = item {
            if &format!("{}", ident) == "main" {
                continue;
            }
        }
        pos_items.push((vec![i], Item::Item(item)));
    }
    while let Some((pos, item)) = pos_items.pop() {
        if let Item::Item(item) = item {
            if let syn::Item::Mod(syn::ItemMod{content: Some((_, items)), ..}) = item {
                for (i, item) in items.iter().enumerate() {
                    let mut next_pos = pos.clone();
                    next_pos.push(i);
                    pos_items.push((next_pos, Item::Item(item)));
                }
            }
            if let syn::Item::Impl(syn::ItemImpl{items, ..}) = item {
                for (i, item) in items.iter().enumerate() {
                    let mut next_pos = pos.clone();
                    next_pos.push(i);
                    pos_items.push((next_pos, Item::ImplItem(item)));
                }
            }
        }
        positions.push(pos);
    }
    positions
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

fn item_positions2(root: &syn::File) -> Vec<(Vec<usize>, (usize, usize))> {
    // extract positions for all mod-level items and items inside impl blocks,
    // except the function named main at root
    let root_span = span_to_bytes(root.span());
    // println!("{:?}", root_span);
    let mut positions = vec![];
    let mut pos_items = Vec::new();
    for (i, item) in root.items.iter().enumerate() {
        if let syn::Item::Fn(syn::ItemFn{sig: syn::Signature{ident, ..}, ..}) = item {
            if &format!("{}", ident) == "main" {
                continue;
            }
        }
        pos_items.push((vec![i], Item::Item(item)));
    }
    while let Some((pos, item)) = pos_items.pop() {
        match item {
            Item::Item(item) => {
                if let syn::Item::Mod(syn::ItemMod{content: Some((_, items)), ..}) = item {
                    for (i, item) in items.iter().enumerate() {
                        let mut next_pos = pos.clone();
                        next_pos.push(i);
                        pos_items.push((next_pos, Item::Item(item)));
                    }
                }
                if let syn::Item::Impl(syn::ItemImpl{items, ..}) = item {
                    for (i, item) in items.iter().enumerate() {
                        let mut next_pos = pos.clone();
                        next_pos.push(i);
                        pos_items.push((next_pos, Item::ImplItem(item)));
                    }
                }
                let span = offset(span_to_bytes(item.span()), root_span.0);
                positions.push((pos, span));
            }
            Item::ImplItem(item) => {
                let span = offset(span_to_bytes(item.span()), root_span.0);
                positions.push((pos, span));
            }
        }
    }
    positions
}
fn item_positions3(root: &syn::File) -> Vec<(Vec<usize>, (usize, usize))> {
    // extract positions of mod-level items that are trait defs, and impl blocks
    // no mods because it will be removed at next dead_code pass
    let root_span = span_to_bytes(root.span());
    // println!("{:?}", root_span);
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
                // let span = offset(span_to_bytes(item.span()), root_span.0);
                // positions.push((pos, span));
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

fn remove_position(root: &mut syn::File, pos: &[usize]) {
    if pos.len() == 1 {
        root.items.remove(pos[0]);
    } else {
        let mut cur_items = &mut root.items;
        let mut rest = pos;
        while rest.len() > 1 {
            let cur_mod = &mut cur_items[rest[0]];
            if let syn::Item::Mod(syn::ItemMod{content: Some((_, items)), ..}) = cur_mod {
                cur_items = items;
                rest = &rest[1..];
            } else if let syn::Item::Impl(syn::ItemImpl{items, ..}) = cur_mod {
                items.remove(rest[1]);
                return;
            } else {
                panic!("Unexpected digging into a non-module-or-impl or a separate-file module.");
            }
        }
        cur_items.remove(rest[0]);
    }
}

fn rustc_check_success(source: &str) -> bool {
    let mut rustc_check = Command::new("rustc");
    rustc_check
        .args(["--emit=mir", "--edition=2021", "--out-dir=/tmp/ramdisk", "-"])
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    let mut child = rustc_check.spawn().expect("Failed to run rustc.");
    let mut stdin = child.stdin.as_ref().expect("Failed to get stdin handle of rustc.");
    stdin.write_all(source.as_bytes()).expect("Failed to write to rustc's stdin.");
    let status = child.wait().expect("Failed to wait for rustc to exit.");
    status.success()
}

fn rustc_check_success2(source: &str) -> bool {
    let mut rustc_check = Command::new("rustc");
    rustc_check
        .args(["-C", "extra-filename=-tmp", "-C", "linker=true", "--edition=2021", "--out-dir=/tmp/ramdisk", "-"])
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    let mut child = rustc_check.spawn().expect("Failed to run rustc.");
    let mut stdin = child.stdin.as_ref().expect("Failed to get stdin handle of rustc.");
    stdin.write_all(source.as_bytes()).expect("Failed to write to rustc's stdin.");
    let status = child.wait().expect("Failed to wait for rustc to exit.");
    status.success()
}

async fn rustc_check_success2_async(source: &str) -> bool {
    async fn inner(source: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let mut rustc_check = tokio::process::Command::new("rustc");
        rustc_check
            .args(["-C", "extra-filename=-tmp", "-C", "linker=true", "--edition=2021", "--out-dir=/tmp/ramdisk", "-"])
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
        | syn::Item::Const(syn::ItemConst{ident, ..})
        | syn::Item::Fn(syn::ItemFn{sig: syn::Signature{ident, ..}, ..})
        | syn::Item::Type(syn::ItemType{ident, ..}) => Some(ident.into()),
        | syn::Item::Struct(syn::ItemStruct{ident, ..})
        | syn::Item::Enum(syn::ItemEnum{ident, ..}) => Some(ident.into()).filter(|_| remove_structs),
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
        .args(["-C", "extra-filename=-tmp", "-C", "linker=true", "--edition", "2021", "--error-format", "json", "-"])
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::piped());
    let child = rustc_check.spawn().expect("Failed to run rustc.");
    let mut stdin = child.stdin.as_ref().expect("Failed to get stdin handle of rustc.");
    stdin.write_all(source.as_bytes()).expect("Failed to write to rustc's stdin.");
    // let status = child.wait().expect("Failed to wait for rustc to exit.");
    // assert!(status.success(), "rustc returned nonzero status code.");
    let rustc_check_stdout = child.wait_with_output().expect("Failed to open rustc's stdout.");
    let rustc_check_output = String::from_utf8(rustc_check_stdout.stderr).unwrap();
    // println!("{}", rustc_check_output);
    let mut deadcodes = HashSet::new();
    for line in rustc_check_output.lines() {
        let Ok(obj) = line.parse::<Value>() else { continue; };
        // only consider obj.code != null
        // and obj.code.code == "dead_code" (warning types may be added)
        // if obj.pointer("/reason") != Some(&Value::from("compiler-message")) { continue; }
        if obj.pointer("/code/code") != Some(&Value::from("dead_code")) { continue; }
        if let (Some(span), Some(Value::String(message))) = (obj.pointer("/spans/0"), obj.pointer("/message")) {
            let filename = span.pointer("/file_name").unwrap().as_str().unwrap();
            let line_start = span.pointer("/line_start").unwrap().as_u64().unwrap();
            let line_end = span.pointer("/line_end").unwrap().as_u64().unwrap();
            let column_start = span.pointer("/column_start").unwrap().as_u64().unwrap();
            let column_end = span.pointer("/column_end").unwrap().as_u64().unwrap();
            let byte_start = span.pointer("/byte_start").unwrap().as_u64().unwrap();
            let byte_end = span.pointer("/byte_end").unwrap().as_u64().unwrap();
            let mut split = message.split('`');
            let item_kind = split.next().unwrap().trim_end();
            let item_name = split.next().unwrap();
            // println!("Found a dead code: file `{}`, item `{}` of kind `{}` (at {}:{} .. {}:{}, bytes {}..{})",
            //     filename, item_name, item_kind, line_start, column_start, line_end, column_end, byte_start, byte_end);
            deadcodes.insert(RawIdent::new(item_name, line_start, column_start-1, line_end, column_end-1));
        }
    }
    // println!("{}", cargo_check_output);
    deadcodes
}

fn cargo_check_deadcode() -> HashSet<RawIdent> {
    let cargo_check = Command::new("cargo")
        .args(["check", "--bin", "tmp", "--message-format", "json"])
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to run cargo check.");
    let cargo_check_stdout = cargo_check.stdout.expect("Failed to open cargo check's stdout.");
    let cargo_check_output = io::read_to_string(cargo_check_stdout).expect("Failed to read cargo check's stdout.");
    let mut deadcodes = HashSet::new();
    for line in cargo_check_output.lines() {
        let Ok(obj) = line.parse::<Value>() else { continue; };
        // only consider obj.reason == "compiler-message" and obj.message.code != null
        // and obj.message.code.code == "dead_code" (warning types may be added)
        if obj.pointer("/reason") != Some(&Value::from("compiler-message")) { continue; }
        if obj.pointer("/message/code/code") != Some(&Value::from("dead_code")) { continue; }
        if let (Some(span), Some(Value::String(message))) = (obj.pointer("/message/spans/0"), obj.pointer("/message/message")) {
            let filename = span.pointer("/file_name").unwrap().as_str().unwrap();
            let line_start = span.pointer("/line_start").unwrap().as_u64().unwrap();
            let line_end = span.pointer("/line_end").unwrap().as_u64().unwrap();
            let column_start = span.pointer("/column_start").unwrap().as_u64().unwrap();
            let column_end = span.pointer("/column_end").unwrap().as_u64().unwrap();
            let byte_start = span.pointer("/byte_start").unwrap().as_u64().unwrap();
            let byte_end = span.pointer("/byte_end").unwrap().as_u64().unwrap();
            let mut split = message.split('`');
            let item_kind = split.next().unwrap().trim_end();
            let item_name = split.next().unwrap();
            // println!("Found a dead code: file `{}`, item `{}` of kind `{}` (at {}:{} .. {}:{}, bytes {}..{})",
            //     filename, item_name, item_kind, line_start, column_start, line_end, column_end, byte_start, byte_end);
            deadcodes.insert(RawIdent::new(item_name, line_start, column_start-1, line_end, column_end-1));
        }
    }
    // println!("{}", cargo_check_output);
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
    // Remove all indentations
    static WS_REGEX: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"\n[ \t]+").unwrap()
    });
    let modified_src = WS_REGEX.replace_all(&modified_src, "\n");
    modified_src.replacen("#![allow(dead_code)]", "", if path == "src/lib" { 1 } else { 0 })
}

fn child_path(path: &str, module: &str) -> String {
    if path == "src/lib" { "src/".to_owned() + module }
    else { path.to_owned() + "/" + module }
}
