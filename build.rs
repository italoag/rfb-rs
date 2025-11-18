// build.rs
use std::env;

fn main() {
    // Re-run build script if this file changes
    println!("cargo:rerun-if-changed=build.rs");

    // Helpful trace when building in CI
    println!(
        "cargo:warning=Running build.rs; TARGET={}",
        env::var("TARGET").unwrap_or_else(|_| "<unknown>".into())
    );

    let target = match env::var("TARGET") {
        Ok(t) => t,
        Err(_) => return,
    };

    // Link math library on non-Windows targets (fixes undefined reference to log10/pow)
    // Windows doesn't have libm (m.lib), so skip it there
    if !target.to_lowercase().contains("windows") {
        println!("cargo:rustc-link-lib=m");
        println!("cargo:warning=Added link to libm (math)");
    } else {
        println!("cargo:warning=Skipping libm link on Windows (math is in C runtime)");
    }

    // For musl targets, let the cross-compilation toolchain handle linking
    // The cross-rs Docker images have all necessary libraries configured
    if target.contains("musl") {
        println!("cargo:warning=Building for musl target - relying on cross-rs toolchain");
    } else if !target.to_lowercase().contains("windows") {
        println!(
            "cargo:warning=Non-musl target detected ({}); libm already linked",
            target
        );
    }
}
