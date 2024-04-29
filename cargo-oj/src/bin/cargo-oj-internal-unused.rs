#![feature(rustc_private)]

extern crate rustc_driver;
extern crate rustc_error_codes;
extern crate rustc_errors;
extern crate rustc_hash;
extern crate rustc_hir;
extern crate rustc_interface;
extern crate rustc_session;
extern crate rustc_span;

use rustc_errors::registry;
use rustc_session::config;
use std::io;
use std::path;
use std::process;
use std::str;
use std::sync::{Arc, atomic::AtomicBool};

fn main() {
    let source = io::read_to_string(io::stdin()).unwrap();
    let out = process::Command::new("rustc")
        .arg("--print=sysroot")
        .current_dir(".")
        .output()
        .unwrap();
    let sysroot = str::from_utf8(&out.stdout).unwrap().trim();
    let config = rustc_interface::Config {
        opts: config::Options {
            maybe_sysroot: Some(path::PathBuf::from(sysroot)),
            // Configure the compiler to emit diagnostics in compact JSON format.
            error_format: config::ErrorOutputType::Json {
                pretty: false,
                json_rendered: rustc_errors::emitter::HumanReadableErrorType::Default(
                    rustc_errors::emitter::ColorConfig::Never,
                ),
            },
            edition: rustc_span::edition::LATEST_STABLE_EDITION,
            ..config::Options::default()
        },
        // This program contains a type error.
        input: config::Input::Str {
            name: rustc_span::FileName::Custom("main.rs".into()),
            input: source,
        },
        crate_cfg: vec![],
        crate_check_cfg: vec![],
        output_dir: None,
        output_file: None,
        file_loader: None,
        locale_resources: rustc_driver::DEFAULT_LOCALE_RESOURCES,
        lint_caps: rustc_hash::FxHashMap::default(),
        psess_created: None,
        register_lints: None,
        override_queries: None,
        registry: registry::Registry::new(&[][..]),
        make_codegen_backend: None,
        expanded_args: vec![],
        ice_file: None,
        hash_untracked_state: None,
        using_internal_features: Arc::new(AtomicBool::from(false)),
    };
    rustc_interface::run_compiler(config, |compiler| {
        compiler.enter(|queries| {
            queries.global_ctxt().unwrap().enter(|tcx| {
                // Run the analysis phase on the local crate to trigger the type error.
                let _ = tcx.analysis(());
            })
        })
    });
}
