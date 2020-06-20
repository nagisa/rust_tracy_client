use std::io::Write;

fn link_libraries(target_os: &str) {
    match target_os {
        "linux" | "android" => println!("cargo:rustc-link-lib=dl"),
        "freebsd" | "dragonfly" => println!("cargo:rustc-link-lib=c"),
        _ => {}
    }
}

fn main() {
    cc::Build::new()
        .define("TRACY_ENABLE", None)
        .file("tracy/TracyClient.cpp")
        .warnings(false)
        .cpp(true)
        .compile("libtracy-client.a");
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
