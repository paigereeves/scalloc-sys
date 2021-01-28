use std::process::Command;
use std::path::PathBuf;

const VERSION: &'static str = "049857919b5fa1d539c9e4206e353daca2e87394";

fn main() {
    // There does not seem to be any consistency as to when this works and when it doesn't
    // Cloning works, just make it yourself for now.
    let out_dir = &PathBuf::from("/home/paiger/scalloc-sys");
    let scalloc_dir = &out_dir.join("scalloc");

    if scalloc_dir.exists() {
        let mut cmd = Command::new("make");
        cmd
            .current_dir(&scalloc_dir)
            .env("BUILDTYPE", "Release")
            // .env("CXX", "gcc")

            // .arg("-n")
            .status().expect("Failed to build scalloc.");
    }

    if !scalloc_dir.exists() {
        // assert!(false);
        clone_scalloc(out_dir);
        assert!(scalloc_dir.exists());
        disable_initial_exec_tls(scalloc_dir);
        make_deps(scalloc_dir);
        generate_build_environment(scalloc_dir);
        
        let mut cmd = Command::new("make");
        cmd
            .current_dir(&scalloc_dir)
            .env("BUILDTYPE", "Release")
            // .env("CXX", "gcc")
            // .arg("-n")
            .status().expect("Failed to build scalloc.");

    }

    // assert!(false, "{:?}", &scalloc_dir);
    // std::env::set_var("BUILDTYPE", "Release");
    // let mut cmd = Command::new("make");
    // cmd
    //     .current_dir(&scalloc_dir)
    //     .env("BUILDTYPE", "Release")
    //     // .env("CXX", "gcc")
    //     // .arg("-n")
    //     .status().expect("Failed to build scalloc.");
    
    
    // assert!(false);
    println!("cargo:rustc-link-lib=dylib=scalloc");
    println!("cargo:rustc-link-search=native={}/scalloc/out/Release/lib.target", out_dir.to_str().unwrap()); 
    println!("cargo:rerun-if-changed={}", out_dir.to_str().unwrap()); 
}


fn clone_scalloc(out_dir: &PathBuf) {
    let mut cmd = Command::new("git");
    cmd
        .current_dir(&out_dir)
        .args(&["clone", "-n", "https://github.com/cksystemsgroup/scalloc"])
        .status().expect("Failed to clone scalloc.");
    let scalloc_dir = out_dir.join("scalloc");
    let mut cmd = Command::new("git");
    cmd
        .current_dir(scalloc_dir)
        .args(&["checkout", VERSION])
        .status().expect("Failed to checkout.");
}

fn make_deps(scalloc_dir: &PathBuf) {
    let mut cmd = Command::new("tools/make_deps.sh");
    cmd
        .current_dir(scalloc_dir)
        .status().expect("Failed to install gyp.");
}

fn disable_initial_exec_tls(scalloc_dir: &PathBuf) {
    let mut cmd = Command::new("sed");
    cmd
        .current_dir(scalloc_dir)
        .args(&["-i", "s/initial-exec/global-dynamic/g", "common.gypi"])
        .status().expect("Failed to sed.");
}

fn generate_build_environment(scalloc_dir: &PathBuf) {
    let mut cmd = Command::new("build/gyp/gyp");
    cmd
        .current_dir(scalloc_dir)
        .args(&["--depth=.", "scalloc.gyp"])
        // .args(&["--depth=.", "-Dlog_level=kTrace", "scalloc.gyp"])
        .status().expect("Failed to generate scalloc build environment.");
}
