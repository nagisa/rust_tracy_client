use std::io::Write;

fn link_libraries(target_os: &str) {
    match target_os {
        "linux" | "android" => println!("cargo:rustc-link-lib=dl"),
        "freebsd" | "dragonfly" => println!("cargo:rustc-link-lib=c"),
        "windows" => println!("cargo:rustc-link-lib=user32"),
        _ => {}
    }
}

fn set_feature_defines(mut c: cc::Build) -> cc::Build {
    if std::env::var_os("CARGO_FEATURE_ENABLE").is_some() {
        c.define("TRACY_ENABLE", None);
    }
    if std::env::var_os("CARGO_FEATURE_TIMER_FALLBACK").is_some() {
        c.define("TRACY_TIMER_FALLBACK", None);
    }
    if std::env::var_os("CARGO_FEATURE_ONDEMAND").is_some() {
        c.define("TRACY_ON_DEMAND", None);
    }
    if std::env::var_os("CARGO_FEATURE_ONLY_LOCALHOST").is_some() {
        c.define("TRACY_ONLY_LOCALHOST", None);
    }
    if std::env::var_os("CARGO_FEATURE_ONLY_IPV4").is_some() {
        c.define("TRACY_ONLY_IPV4", None);
    }
    if std::env::var_os("CARGO_FEATURE_FIBERS").is_some() {
        c.define("TRACY_FIBERS", None);
    }

    if !std::env::var_os("CARGO_FEATURE_SYSTEM_TRACING").is_some() {
        c.define("TRACY_NO_SYSTEM_TRACING", None);
    }
    if !std::env::var_os("CARGO_FEATURE_CONTEXT_SWITCH_TRACING").is_some() {
        c.define("TRACY_NO_CONTEXT_SWITCH", None);
    }
    if !std::env::var_os("CARGO_FEATURE_SAMPLING").is_some() {
        c.define("TRACY_NO_SAMPLING", None);
    }
    if !std::env::var_os("CARGO_FEATURE_CODE_TRANSFER").is_some() {
        c.define("TRACY_NO_CODE_TRANSFER", None);
    }
    if !std::env::var_os("CARGO_FEATURE_BROADCAST").is_some() {
        c.define("TRACY_NO_BROADCAST", None);
    }
    c
}

fn main() {
    if std::env::var_os("CARGO_FEATURE_ENABLE").is_some() {
        set_feature_defines(cc::Build::new())
            .file("tracy/TracyClient.cpp")
            .define("TRACY_MANUAL_LIFETIME", None)
            .define("TRACY_DELAYED_INIT", None)
            .warnings(false)
            .cpp(true)
            .flag_if_supported("-std=c++11")
            .compile("libtracy-client.a");
    }

    match std::env::var("CARGO_CFG_TARGET_OS") {
        Ok(target_os) => {
            link_libraries(&target_os);
        }
        Err(e) => {
            writeln!(::std::io::stderr(), "Unable to get target_os=`{}`!", e)
                .expect("could not report the error");
            ::std::process::exit(0xfd);
        }
    }
}
