use std::{env, fmt::Write, fs, path::Path};

fn main() {
    // Validate the kernel selection and select `std` if none are selected
    const KERNEL_LIST: &[&str] = &["asp3", "fmp3", "solid_asp3", "solid_fmp3"];
    let selected_kernels: Vec<_> = KERNEL_LIST
        .iter()
        .filter(|name| {
            env::var_os(format!("CARGO_FEATURE_{}", name.to_ascii_uppercase())).is_some()
        })
        .collect();
    if selected_kernels.len() > 1 {
        panic!("more than one kernel is selected: {:?}", selected_kernels);
    }
    if selected_kernels.is_empty() {
        // Default to `none` if none are selected
        // (Maintainer note: Please make sure to synchronize the transitive
        // features with `Cargo.toml`)
        println!("cargo:rustc-cfg=feature=\"none\"");
        println!("cargo:rustc-cfg=feature=\"dcre\"");
        println!("cargo:rustc-cfg=feature=\"rstr_task\"");
        println!("cargo:rustc-cfg=feature=\"messagebuf\"");
        println!("cargo:rustc-cfg=feature=\"ovrhdr\"");
        println!("cargo:rustc-cfg=feature=\"subprio\"");
        println!("cargo:rustc-cfg=feature=\"pi_mutex\"");
        println!("cargo:rustc-cfg=feature=\"systim_local\"");
        println!("cargo:rustc-cfg=feature=\"exd_tsk\"");
    }

    // Generate code for `itron::macros`
    let mut macros_rs = String::new();
    macro_rules! w {
        ($($tt:tt)*) => {
            write!(macros_rs, $($tt)*).unwrap()
        };
    }

    for kernel in KERNEL_LIST.iter().cloned().chain(["none"]) {
        w!(
            r#"
            /// Get the kernel selected by a Cargo feature.
            ///
            /// # Example
            ///
            /// ```
            /// let kernel = tt_call::tt_call! {{ macro = [{{ itron::macros::tt_kernel }}] }};
            ///
            /// println!("We are running on {{}}", kernel);
            /// ```
            ///
            #[cfg(feature = "{kernel}")]
            pub macro tt_kernel($caller:tt) {{
                tt_call::tt_return! {{
                    $caller
                    output = [{{ "{kernel}" }}]
                }}
            }}
            "#,
            kernel = kernel,
        );
        w!(
            r#"
            /// Determine if this crate was compiled for the specified kernel.
            ///
            /// # Example
            ///
            /// ```
            /// tt_call::tt_if! {{
            ///     condition = [{{ itron::macros::tt_is_kernel }}]
            ///     input = [{{ "asp3" }}]
            ///     true = [{{ println!("We are on TOPPERS/ASP3, yay!"); }}]
            ///     false = [{{}}]
            /// }}
            /// ```
            ///
            #[cfg(feature = "{kernel}")]
            pub macro tt_is_kernel {{
                (
                    $caller:tt
                    input = [{{ "{kernel}" $(| $($rest:literal)|+ )? }}]
                ) => {{
                    tt_call::tt_return! {{
                        $caller
                        is = [{{ true }}]
                    }}
                }},
                (
                    $caller:tt
                    input = [{{ $other_kernel:literal $(| $($rest:literal)|+ )? }}]
                ) => {{
                    tt_is_kernel! {{
                        $caller
                        input = [{{ $( $($rest)|+ )? }}]
                    }}
                }},
                (
                    $caller:tt
                    input = [{{ }}]
                ) => {{
                    tt_call::tt_return! {{
                        $caller
                        is = [{{ false }}]
                    }}
                }},
            }}
            "#,
            kernel = kernel,
        );
    }

    let out_dir = env::var_os("OUT_DIR").unwrap();
    fs::write(&Path::new(&out_dir).join("macros.rs"), &macros_rs).unwrap();
}
