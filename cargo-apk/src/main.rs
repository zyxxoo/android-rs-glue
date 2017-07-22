extern crate cargo;
extern crate rustc_serialize;
extern crate term;
extern crate toml;

use std::env;

use cargo::core::Workspace;
use cargo::ops::MessageFormat;
use cargo::util::Config as CargoConfig;
use cargo::util::important_paths::find_root_manifest_for_wd;

mod config;
mod ops;

fn main() {
    let cargo_config = CargoConfig::default().unwrap();

    let args = env::args().collect::<Vec<_>>();

    let err = match args.get(2).map(|a| &a[..]) {
        Some("install") => {
            cargo::call_main_without_stdin(execute_install, &cargo_config, INSTALL_USAGE, &args, false)
        },
        _ => {
            cargo::call_main_without_stdin(execute_build, &cargo_config, BUILD_USAGE, &args, false)
        }
    };

    match err {
        Ok(_) => (),
        Err(err) => cargo::exit_with_error(err, &mut *cargo_config.shell())
    }
}

pub fn execute_build(options: Options, cargo_config: &CargoConfig) -> cargo::CliResult {
    cargo_config.configure(options.flag_verbose,
                           options.flag_quiet,
                           &options.flag_color,
                           options.flag_frozen,
                           options.flag_locked)?;

    let root_manifest = find_root_manifest_for_wd(options.flag_manifest_path.clone(),
                                                  cargo_config.cwd())?;

    let workspace = Workspace::new(&root_manifest, &cargo_config)?;

    let mut android_config = config::load(workspace.current()?.manifest_path());
    android_config.release = options.flag_release;
    if !options.flag_bin.is_empty() {
        android_config.target = Some(options.flag_bin[0].clone());
    }

    ops::build(&workspace, &android_config, &options)?;
    Ok(())
}

pub fn execute_install(options: Options, cargo_config: &CargoConfig) -> cargo::CliResult {
    cargo_config.configure(options.flag_verbose,
                           options.flag_quiet,
                           &options.flag_color,
                           options.flag_frozen,
                           options.flag_locked)?;

    let root_manifest = find_root_manifest_for_wd(options.flag_manifest_path.clone(),
                                                  cargo_config.cwd())?;

    let workspace = Workspace::new(&root_manifest, &cargo_config)?;

    let mut android_config = config::load(workspace.current()?.manifest_path());
    android_config.release = options.flag_release;
    if !options.flag_bin.is_empty() {
        android_config.target = Some(options.flag_bin[0].clone());
    }

    ops::install(&workspace, &android_config, &options)?;
    Ok(())
}

#[derive(RustcDecodable)]
pub struct Options {
    flag_package: Vec<String>,
    flag_jobs: Option<u32>,
    flag_features: Vec<String>,
    flag_all_features: bool,
    flag_no_default_features: bool,
    flag_manifest_path: Option<String>,
    flag_verbose: u32,
    flag_quiet: Option<bool>,
    flag_color: Option<String>,
    flag_message_format: MessageFormat,
    flag_release: bool,
    flag_lib: bool,
    flag_bin: Vec<String>,
    flag_bins: bool,
    flag_example: Vec<String>,
    flag_examples: bool,
    flag_test: Vec<String>,
    flag_tests: bool,
    flag_bench: Vec<String>,
    flag_benches: bool,
    flag_locked: bool,
    flag_frozen: bool,
    flag_all: bool,
    flag_exclude: Vec<String>,
}

const BUILD_USAGE: &'static str = r#"
Usage:
    cargo apk [options]

Options:
    -h, --help                   Print this message
    -p SPEC, --package SPEC ...  Package to build
    --all                        Build all packages in the workspace
    --exclude SPEC ...           Exclude packages from the build
    -j N, --jobs N               Number of parallel jobs, defaults to # of CPUs
    --lib                        Build only this package's library
    --bin NAME                   Build only the specified binary
    --bins                       Build all binaries
    --example NAME               Build only the specified example
    --examples                   Build all examples
    --test NAME                  Build only the specified test target
    --tests                      Build all tests
    --bench NAME                 Build only the specified bench target
    --benches                    Build all benches
    --release                    Build artifacts in release mode, with optimizations
    --features FEATURES          Space-separated list of features to also build
    --all-features               Build all available features
    --no-default-features        Do not build the `default` feature
    --manifest-path PATH         Path to the manifest to compile
    -v, --verbose ...            Use verbose output (-vv very verbose/build.rs output)
    -q, --quiet                  No output printed to stdout
    --color WHEN                 Coloring: auto, always, never
    --message-format FMT         Error format: human, json [default: human]
    --frozen                     Require Cargo.lock and cache are up to date
    --locked                     Require Cargo.lock is up to date

Does the same as `cargo build`.
"#;

const INSTALL_USAGE: &'static str = r#"
Usage:
    cargo apk install [options]

Options:
    -h, --help                   Print this message
    -p SPEC, --package SPEC ...  Package to build
    --all                        Build all packages in the workspace
    --exclude SPEC ...           Exclude packages from the build
    -j N, --jobs N               Number of parallel jobs, defaults to # of CPUs
    --lib                        Build only this package's library
    --bin NAME                   Build only the specified binary
    --bins                       Build all binaries
    --example NAME               Build only the specified example
    --examples                   Build all examples
    --test NAME                  Build only the specified test target
    --tests                      Build all tests
    --bench NAME                 Build only the specified bench target
    --benches                    Build all benches
    --release                    Build artifacts in release mode, with optimizations
    --features FEATURES          Space-separated list of features to also build
    --all-features               Build all available features
    --no-default-features        Do not build the `default` feature
    --manifest-path PATH         Path to the manifest to compile
    -v, --verbose ...            Use verbose output (-vv very verbose/build.rs output)
    -q, --quiet                  No output printed to stdout
    --color WHEN                 Coloring: auto, always, never
    --message-format FMT         Error format: human, json [default: human]
    --frozen                     Require Cargo.lock and cache are up to date
    --locked                     Require Cargo.lock is up to date

Does the same as `cargo build`.
"#;
