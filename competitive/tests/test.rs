use std::process::{Command, Stdio};
use std::io::Write;

use console::Style;
use similar::{ChangeTag, TextDiff};
use reqwest::blocking::get;
use scraper::{Html, Selector};

fn bin() -> Command {
    test_bin::get_test_bin("main")
}

fn io(input: &str) -> String {
    let mut handle = bin().stdin(Stdio::piped()).stdout(Stdio::piped()).spawn().unwrap();
    let stdin = handle.stdin.as_mut().unwrap();
    write!(stdin, "{}", input).unwrap();
    //stdin.flush().unwrap();
    let stdout = handle.wait_with_output().unwrap();
    String::from_utf8_lossy(&stdout.stdout).to_string()
}

//#[test]
#[allow(dead_code)]
fn custom() {
    // write test code
}
#[test]
fn is_solved() {
    let url = format!("https://www.acmicpc.net/problem/{}", std::env::var("BOJ").unwrap());
    let res = get(url).unwrap().text().unwrap();
    let html = Html::parse_document(&res);
    let spj_selector = Selector::parse("span.problem-label-spj").unwrap();
    let mut it = html.select(&spj_selector);
    let spj = it.next().is_some();
    let selector = Selector::parse("pre.sampledata").unwrap();
    let mut it = html.select(&selector);
    while let Some(inel) = it.next() {
        let output = it.next().unwrap().text().collect::<String>();
        let input = inel.text().collect::<String>();
        //let mut ii = std::io::Cursor::new(input.as_bytes());
        //let mut oo = std::io::Cursor::new(Vec::<u8>::new());
        let now = std::time::Instant::now();
        //crate::solve(&mut ii, &mut oo);
        let result = io(&input);
        let elapsed = now.elapsed().as_micros();
        //let result = unsafe { String::from_utf8_unchecked(oo.into_inner()) };
        let output = output.trim_end().lines().map(|l| l.trim_end()).collect::<Vec<_>>().join("\n");
        let result = result.trim_end().lines().map(|l| l.trim_end()).collect::<Vec<_>>().join("\n");
        let diff = TextDiff::from_lines(&result, &output);
        let styles = if spj { (Style::new(), Style::new(), Style::new()) } else { (Style::new().red(), Style::new().green(), Style::new()) };
        let mut failed = false;
        for op in diff.ops() {
            for change in diff.iter_changes(op) {
                let (sign, style) = match change.tag() {
                    ChangeTag::Delete => { failed = true; ("-", &styles.0) },
                    ChangeTag::Insert => { failed = true; ("+", &styles.1) },
                    ChangeTag::Equal => (" ", &styles.2),
                };
                print!("{}{}", style.apply_to(sign), style.apply_to(change));
            }
        }
        if !spj && failed { assert!(false, "incorrect output"); }
        println!("Elapsed: {}.{:06}", elapsed / 1000000, elapsed % 1000000);
    }
}