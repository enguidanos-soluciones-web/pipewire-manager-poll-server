use semver::{Version, VersionReq};

fn main() {
    #[cfg(not(target_os = "linux"))]
    compile_error!("This project only supports Linux");

    let lib = pkg_config::probe_library("libpipewire-0.3").expect("PipeWire not found");

    let version = Version::parse(&lib.version).expect("Failed to parse PipeWire 0.3 version");

    if VersionReq::parse(">=1.0.0, <2.0.0")
        .expect("Failed to parse Version Requirement")
        .matches(&version)
    {
        println!("cargo:rustc-cfg=pw_v1_0")
    } else {
        println!(
            "cargo:warning=PipeWire version {} is not supported. Required: >=1.0.0, <2.0.0",
            version
        );
        std::process::exit(1);
    }

    println!("cargo:rerun-if-env-changed=PKG_CONFIG_PATH");
}
