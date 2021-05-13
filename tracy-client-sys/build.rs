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
    if std::env::var_os("CARGO_FEATURE_DELAYED_INIT").is_some() {
        c.define("TRACY_DELAYED_INIT", None);
    }
    if std::env::var_os("CARGO_FEATURE_LOWRES_TIMER").is_some() {
        c.define("TRACY_TIMER_QPC", None);
    }
    if std::env::var_os("CARGO_FEATURE_NOEXIT").is_some() {
        c.define("TRACY_NO_EXIT", None);
    }
    if std::env::var_os("CARGO_FEATURE_ONDEMAND").is_some() {
        c.define("TRACY_ON_DEMAND", None);
    }
    c
}


fn main() {
    if std::env::var_os("CARGO_FEATURE_ENABLE").is_some() {
        set_feature_defines(cc::Build::new())
            .file("tracy/TracyClient.cpp")
            .warnings(false)
            .cpp(true)
            .flag_if_supported("-std=gnu++17")
            .compile("libtracy-client.a");
    }

    match std::env::var("CARGO_CFG_TARGET_OS") {
        Ok(target_os) => {
            link_libraries(&target_os);
        }
        Err(e) => {
            writeln!(::std::io::stderr(),
                     "Unable to get target_os=`{}`!", e).expect("could not report the error");
            ::std::process::exit(0xfd);
        }
    }
}
