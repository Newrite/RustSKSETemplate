fn main() {
    println!("cargo:rerun-if-changed=../");
    println!("cargo:rustc-link-arg=/DLL");

    // Встраиваем версионный ресурс в DLL
    let mut res = winresource::WindowsResource::new();
    res.set("FileDescription", "SKSE Hello Rust");
    res.set("ProductName",     "skse-hello-rust");
    res.set("FileVersion",     env!("CARGO_PKG_VERSION"));
    res.compile().unwrap();

    // Версия из git для лога (необязательно, но полезно)
    let version = std::process::Command::new("git")
        .args(&["describe", "--always", "--dirty", "--tags"])
        .output()
        .map(|o| String::from_utf8(o.stdout).unwrap_or_default())
        .unwrap_or_else(|_| "unknown".into());
    println!("cargo:rustc-env=LIBSKYRIM_PLUGIN_VC_VERSION={}", version.trim());
}
