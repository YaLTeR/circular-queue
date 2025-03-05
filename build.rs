extern crate version_check;

fn main() {
    println!("cargo:rustc-check-cfg=cfg(has_extern_crate_alloc)");
    if version_check::is_min_version("1.36.0").unwrap_or(false) {
        println!("cargo:rustc-cfg=has_extern_crate_alloc");
    }

    println!("cargo:rustc-check-cfg=cfg(has_relaxed_orphan_rule)");
    if version_check::is_min_version("1.41.0").unwrap_or(false) {
        println!("cargo:rustc-cfg=has_relaxed_orphan_rule");
    }
}
