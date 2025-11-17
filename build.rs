// build.rs
use std::{env, path::Path};

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

    if target.contains("musl") {
        // Try to use pkg-config to find libunwind if available
        if pkg_config::probe_library("libunwind").is_ok()
            || pkg_config::probe_library("unwind").is_ok()
        {
            // pkg-config will have already emitted the necessary link flags,
            // but explicitly instruct cargo too in case
            println!("cargo:rustc-link-lib=unwind");
            println!("cargo:warning=Found libunwind via pkg-config; linking unwind");
        } else {
            // Add common musl/gcc search paths if they exist
            let common_paths = [
                "/usr/lib/x86_64-linux-musl",
                "/usr/lib/gcc/x86_64-linux-gnu",
                "/usr/lib/gcc/x86_64-linux-musl",
                "/usr/lib",
                "/usr/lib64",
            ];

            for p in &common_paths {
                if Path::new(p).exists() {
                    println!("cargo:rustc-link-search=native={}", p);
                    println!("cargo:warning=Added native search path: {}", p);
                }
            }

            // Try linking common fallback libraries that provide personality/unwind symbols.
            println!("cargo:rustc-link-lib=gcc_s"); // dynamic
            println!("cargo:rustc-link-lib=static=gcc"); // static fallback
            println!("cargo:rustc-link-lib=static=unwind");
            println!("cargo:warning=Added fallback links: gcc_s, static=gcc, static=unwind");
        }

        if let Ok(libpath) = env::var("LIBRARY_PATH") {
            println!("cargo:warning=LIBRARY_PATH={}", libpath);
        } else {
            println!(
                "cargo:warning=No LIBRARY_PATH set; if link errors persist, set LIBRARY_PATH to musl lib dir"
            );
        }
    } else if !target.to_lowercase().contains("windows") {
        println!(
            "cargo:warning=Non-musl target detected ({}); libm already linked",
            target
        );
    }
}
