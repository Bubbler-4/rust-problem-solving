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
use std::sync::{Arc, atomic::AtomicBool};

fn main() {
    let source = io::read_to_string(io::stdin()).unwrap();
    let config = rustc_interface::Config {
        opts: config::Options {
            maybe_sysroot: None,
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
        parse_sess_created: None,
        register_lints: None,
        override_queries: None,
        registry: registry::Registry::new(&[][..]),
        make_codegen_backend: None,
        expanded_args: vec![],
        ice_file: None,
        hash_untracked_state: None,
        using_internal_features: Arc::new(AtomicBool::from(false)),
    };
    let success = rustc_interface::run_compiler(config, |compiler| {
        compiler.enter(|queries| {
            queries.global_ctxt().unwrap().enter(|tcx| {
                // Run the analysis phase on the local crate to trigger the type error.
                for defid in tcx.hir().body_owners() {
                    if let rustc_hir::def::DefKind::Fn | rustc_hir::def::DefKind::AssocFn = tcx.def_kind(defid) {
                        let ck = tcx.diagnostic_only_typeck(defid);
                        if ck.tainted_by_errors.is_some() {
                            return false;
                        }
                    }
                }
                // let items = tcx.hir_crate_items(());
                // for defid in items.owners() {
                //     if let rustc_hir::def::DefKind::Fn = tcx.def_kind(defid) {
                //         let ck = tcx.diagnostic_only_typeck(defid);
                //         if ck.tainted_by_errors.is_some() {
                //             return false;
                //         }
                //     }
                // }
                true
            })
        })
    });
    std::process::exit(if success { 0 } else { 1 });
}
