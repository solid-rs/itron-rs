//! This test checks that `itron::abi`'s contents exactly match what the target
//! kernel provides.
//!
//! Note: This test is a little bit tricky to get working. `target/debug` must
//! be ridden of other copies of `itron*.rlib` compiled with different Cargo
//! feature sets.
#![cfg(not(feature = "none"))]
use std::{collections::HashSet, env, fmt::Write, fs, path::Path};

mod symbols;

trait KernelAbi {
    fn get_symbols(&self, b: &mut SymbolsBuilder);
}

#[derive(Default)]
struct SymbolsBuilder {
    // For now we are only interested in function names
    func_names: HashSet<&'static str>,
}

impl SymbolsBuilder {
    fn insert_func(&mut self, f: symbols::Func) {
        self.func_names.insert(f.name);
    }
}

// Target kernels' ABI definitions
#[cfg(feature = "asp3")]
mod asp3;
#[cfg(feature = "asp3")]
use asp3 as os;

#[cfg(feature = "solid_asp3")]
mod solid_asp3;
#[cfg(feature = "solid_asp3")]
use solid_asp3 as os;

#[cfg(feature = "fmp3")]
mod fmp3;
#[cfg(feature = "fmp3")]
use fmp3 as os;

#[cfg(feature = "solid_fmp3")]
mod solid_fmp3;
#[cfg(feature = "solid_fmp3")]
use solid_fmp3 as os;

#[test]
fn abi_function_set() {
    let actual_abi = os::Abi;

    // Get the target kernel's provided symbols
    let mut actual_symbols = SymbolsBuilder::default();
    actual_abi.get_symbols(&mut actual_symbols);

    // Enumerate functions that are not supposed to exist
    let all_func_names: HashSet<&'static str> =
        symbols::known_funcs::ALL_NAMES.iter().cloned().collect();
    let bad_func_names = (&all_func_names) - (&actual_symbols.func_names);

    println!(
        "actual_symbols.func_names = {:?}",
        actual_symbols.func_names
    );
    println!(
        "bad_func_names (expected not to be in `itron::abi`) = {:?}",
        bad_func_names
    );

    // Generate compile tests in `$OUT_DIR`
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir);
    let pass_dir = out_dir.join("abi-test-pass");
    let fail_dir = out_dir.join("abi-test-fail");
    let _ = fs::remove_dir_all(&pass_dir);
    let _ = fs::remove_dir_all(&fail_dir);
    fs::create_dir_all(&pass_dir).unwrap();
    fs::create_dir_all(&fail_dir).unwrap();

    macro_rules! codegen {
        ($dollar:tt $($tt:tt)*) => {{
            let mut rs = String::new();
            macro_rules! wln { ($dollar($tt2:tt)*) => { writeln!(rs, $dollar($tt2)*).unwrap() }; }
            $($tt)*
            rs
        }};
    }
    let pass_test = codegen! {$
        wln!("fn main() {{");
        for &func_name in actual_symbols.func_names.iter() {
            wln!("    let _ = itron::abi::{};", func_name);
        }
        wln!("}}");
    };
    fs::write(pass_dir.join("func_names.rs"), pass_test).unwrap();
    let fail_test = codegen! {$
        wln!("fn main() {{");
        for &func_name in bad_func_names.iter() {
            wln!("    let _ = itron::abi::{0}; //~ ERROR cannot find value `{0}` in module `itron::abi`", func_name);
        }
        wln!("}}");
    };
    fs::write(fail_dir.join("func_names.rs"), fail_test).unwrap();

    // Run compile tests
    let flags = "--edition=2018 --extern itron";
    {
        let mut config = compiletest::Config::default();
        config.mode = compiletest::common::Mode::RunPass;
        config.target_rustcflags = Some(flags.to_string());
        config.src_base = pass_dir.to_owned();
        config.link_deps();
        config.clean_rmeta();
        compiletest::run_tests(&config);
    }
    {
        let mut config = compiletest::Config::default();
        config.mode = compiletest::common::Mode::CompileFail;
        config.target_rustcflags = Some(flags.to_string());
        config.src_base = fail_dir.to_owned();
        config.link_deps();
        config.clean_rmeta();
        compiletest::run_tests(&config);
    }
}
