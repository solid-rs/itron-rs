use std::env;

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
    }
}
