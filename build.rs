fn main() {
    if let Ok(dir) = std::env::var("PF_RING_LIB_DIR") {
        println!("cargo:rustc-link-search=native={}", dir);
    }
    println!("cargo:rustc-link-search=native=/usr/local/lib");
    println!("cargo:rustc-link-search=native=/usr/lib");
    println!("cargo:rustc-link-search=native=/usr/lib/x86_64-linux-gnu");
    println!("cargo:rustc-link-lib=dylib=pfring");

    if pkg_config::Config::new().probe("libpcap").is_ok() {
        return;
    }
    println!("cargo:rustc-link-lib=dylib=pcap");
    println!("cargo:rerun-if-env-changed=PF_RING_LIB_DIR");
}
