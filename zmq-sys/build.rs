extern crate pkg_config;

use std::env;

fn main() {
    let lib_path = env::var("LIBZMQ_LIB_DIR").ok().or_else(|| {
        env::var("LIBZMQ_PREFIX").ok()
            .map(|prefix| format!("{}/lib", prefix))
    });

    let include = env::var("LIBZMQ_INCLUDE_DIR").ok().or_else(|| {
        env::var("LIBZMQ_PREFIX").ok()
            .map(|prefix| format!("{}/include", prefix))
    });

    match (lib_path, include) {
        (Some(lib_path), Some(include)) => {
            println!("cargo:rustc-link-search=native={}", lib_path);
            println!("cargo:include={}", include);
        }
        (Some(_), None) => {
            panic!("Unable to locate libzmq include directory.")
        }
        (None, Some(_)) => {
            panic!("Unable to locate libzmq library directory.")
        }
        (None, None) => {
            match pkg_config::probe_library("libzmq") {
                Ok(pkg) => println!("{:?}", pkg),
                Err(e) => panic!("Unable to locate libzmq, err={:?}", e),
            }
        }
    }
}
