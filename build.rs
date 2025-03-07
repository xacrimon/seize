fn main() {
    #[cfg(all(target_os = "macos", feature = "fast-barrier", not(miri)))]
    {
        println!("cargo:rerun-if-changed=mach-bridge.c");
        cc::Build::new()
            .file("mach-bridge.c")
            .compile("mach-bridge");
    }
}
