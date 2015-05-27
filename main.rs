use std::fs::File;

use cmderror::CmdError;
use rscrate::Crate;

mod cmderror;
mod rscrate;


fn main() {
    let builddir = "target";

    let args: Vec<_> = std::env::args().collect();
    if args.len() < 3 {
        println!["Usage: {} <target OS> <triple>", args[0]];
        println![""];
        println!["Target OS:"];
        println!["  android, bitrig, dragonfly, freebsd, ios"];
        println!["  linux, macos, nacl or openbsd"];
        println![""];
        println!["Triple examples:"];
        println!["  x86_64-unknown-freebsd"];
        return;
    }

    let target_os = &args[1];
    let triple = &args[2];

    //
    // Dependencies of the Cargo crate, listed in build order.
    //
    let dependencies = vec![
        // Build libc with Cargo-like options rather than bootstrap options
        // (so it can be used with `extern crate libc`).
        Crate::new("libc").owner("rust-lang")
              .feature("cargo-build")
              .filename("rust/src/liblibc/lib.rs")
              .cfg("unix")
              .target_os(target_os),

        Crate::new("bitflags").owner("rust-lang"),
        Crate::new("glob").owner("rust-lang"),
        Crate::new("log").owner("rust-lang").extern_lib("libc", builddir),
        Crate::new("rand").owner("rust-lang").extern_lib("libc", builddir),
        Crate::new("regex").owner("rust-lang"),
        Crate::new("rustc_serialize").repo("rustc-serialize").owner("rust-lang"),
        Crate::new("semver").owner("rust-lang"),
        Crate::new("tempdir").owner("rust-lang").extern_lib("rand", builddir),
        Crate::new("term").owner("rust-lang"),
        Crate::new("threadpool").owner("rust-lang"),
        Crate::new("time").owner("rust-lang").extern_lib("libc", builddir),

        Crate::new("strsim").repo("strsim-rs").owner("dguo"),
        Crate::new("docopt").repo("docopt.rs").owner("docopt").kind("dylib"),

        Crate::new("matches").repo("rust-std-candidates").owner("SimonSapin")
              .filename("matches/lib.rs"),
        Crate::new("url").repo("rust-url").owner("servo"),
        Crate::new("libz_sys").repo("libz-sys").owner("alexcrichton")
              .extern_lib("libc", builddir),
        Crate::new("openssl_sys").repo("rust-openssl").owner("sfackler")
              .filename("openssl-sys/src/lib.rs")
              .extern_lib("libc", builddir),
        Crate::new("curl_sys").repo("curl-rust").owner("carllerche")
              .filename("curl-sys/lib.rs")
              .extern_lib("libc", builddir),
        Crate::new("curl").repo("curl-rust").owner("carllerche")
              .extern_lib("libc", builddir)
              .extern_lib("log", builddir),

        Crate::new("encoding_index_japanese")
              .repo("rust-encoding").owner("lifthrasiir")
              .filename("src/index/japanese/lib.rs"),
        Crate::new("encoding_index_korean")
              .repo("rust-encoding").owner("lifthrasiir")
              .filename("src/index/korean/lib.rs"),
        Crate::new("encoding_index_simpchinese")
              .repo("rust-encoding").owner("lifthrasiir")
              .filename("src/index/simpchinese/lib.rs"),
        Crate::new("encoding_index_singlebyte")
              .repo("rust-encoding").owner("lifthrasiir")
              .filename("src/index/singlebyte/lib.rs"),
        Crate::new("encoding_index_tradchinese")
              .repo("rust-encoding").owner("lifthrasiir")
              .filename("src/index/tradchinese/lib.rs"),
        Crate::new("encoding").repo("rust-encoding").owner("lifthrasiir"),

        Crate::new("miniz_sys").repo("flate2-rs").owner("alexchichton")
              .filename("miniz-sys/lib.rs")
              .extern_lib("libc", builddir),
        Crate::new("flate2").repo("flate2-rs").owner("alexcrichton")
              .extern_lib("libc", builddir),

        Crate::new("hamcrest").repo("hamcrest-rust").owner("carllerche"),
        Crate::new("tar").repo("tar-rs").owner("alexcrichton")
              .feature("nightly")
              .extern_lib("libc", builddir),
        Crate::new("ssh2").repo("libssh2-static-sys").owner("alexcrichton"),
        Crate::new("openssl_sys").repo("openssl-static-sys")
              .owner("alexcrichton"),
        Crate::new("toml").repo("toml-rs").owner("alexcrichton")
              .feature("rustc-serialize"),

        Crate::new("conduit").owner("conduit-rust"),
        Crate::new("civet_sys").repo("rust-civet").owner("wycats")
              .filename("civet-sys/lib.rs"),
        Crate::new("civet").repo("rust-civet").owner("wycats")
              .extern_lib("libc", builddir),

        Crate::new("libssh2_sys").repo("ssh2-rs").owner("alexcrichton")
              .filename("libssh2-sys/lib.rs")
              .extern_lib("libc", builddir),
        Crate::new("libgit2_sys").repo("git2-rs").owner("alexcrichton")
              .filename("libgit2-sys/lib.rs")
              .extern_lib("libc", builddir),
        Crate::new("git2").repo("git2-rs").owner("alexcrichton")
              .extern_lib("libc", builddir),

        Crate::new("num_cpus").owner("seanmonstar")
              .extern_lib("libc", builddir),

        // Cargo itself includes libraries and a separate command-line binary:
        Crate::new("registry").repo("cargo").owner("rust-lang")
              .filename("src/registry/lib.rs"),
        Crate::new("cargo").owner("rust-lang")
              .filename("src/cargo/lib.rs")
              .extern_lib("log", builddir)
              .extern_lib("term", builddir),
    ];

    let cargo = Crate::new("cargo").owner("rust-lang");

//"""
//OUT_DIR=../../../target/ gmake -C deps/flate2-rs/build/
//deps/git2-rs/src/lib.rs --extern url=target/liburl.rlib
//deps/curl-rust/src/lib.rs --extern url=target/liburl.rlib
//"""

    match fetch_and_compile(&dependencies, "src", builddir)
        .and_then(|s:String| {
            println!["{}", s];
            cargo.fetch(".")
        })
        .and_then(|s:String| {
            println!["{}", s];
            cargo.compile(".", builddir)
        }) {

        Ok(result) => println!["{}", result],
        Err(e) => println!["Error: {}", e],
    }
}


fn fetch_and_compile(crates: &[Crate], subdir: &str, builddir: &str)
    -> Result<String, CmdError> {

    let donedir = format!["{}/.done", builddir];
    try![std::fs::create_dir_all(&donedir)];

    for c in crates {
        println!["==> {}", c];

        // Use a file to indicate whether we've already built this crate.
        let marker = format!["{}/{}", &donedir, c.name];

        match File::open(&marker) {
            Ok(_) => {},
            Err(_) => {
                println!["Fetching..."];
                try![c.fetch(subdir)];

                println!["Compiling..."];
                try![c.compile(subdir, builddir)];

                println![""];
                try![std::fs::File::create(&marker)];
            }
        }
    };

    Ok(format!["Fetched and compiled {} crates.", crates.len()])
}
