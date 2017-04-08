use std::process::Command;

fn main() {
    compile_sass();
}

fn compile_sass() {
    assert!(Command::new("sass")
        .arg("--scss")
        .arg("--style")
            .arg("expanded")
        .arg("--sourcemap=none")
        .arg("scss/styles.scss:static/css/styles.css")
        .status()
        .expect("scss not compiled :(")
        .success());

    // Only rebuild `build.rs` process if these these have changed:
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=scss/styles.scss");
}