fn main() {
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo:rustc-link-arg=/DLL");

    let mut res = winresource::WindowsResource::new();
    res.set(
        "FileDescription",
        "Rust SKSE plugin template powered by libskyrim",
    );
    res.set("ProductName", "RustSKSETemplate");
    res.set("FileVersion", env!("CARGO_PKG_VERSION"));
    res.compile().unwrap();

    let version = std::process::Command::new("git")
        .args(["describe", "--always", "--dirty", "--tags"])
        .output()
        .map(|o| String::from_utf8(o.stdout).unwrap_or_default())
        .unwrap_or_else(|_| "unknown".into());
    println!(
        "cargo:rustc-env=LIBSKYRIM_PLUGIN_VC_VERSION={}",
        version.trim()
    );
}
