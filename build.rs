use std::env;
use std::path::Path;

fn main() {
    let toolchain = install_packages();
    let out_dir = env::var("OUT_DIR").unwrap();
    println!("Building with {:#?}", out_dir);

    let tailwind_file = out_dir + "/tailwind.css";
    std::process::Command::new(toolchain)
        .args([
            "tailwind",
            "-i",
            "src/index.css",
            "-c",
            "tailwind.config.js",
            "-o",
            tailwind_file.as_str(),
            "--minify",
        ])
        .env("NODE_ENV", "production")
        .spawn()
        .unwrap();
}

fn install_packages() -> &'static str {
    let npm = if_windows("npm.cmd", "npm");
    let npx = if_windows("npx.cmd", "npx");

    match std::process::Command::new(npm).arg("install").spawn() {
        Ok(_) => npx,
        Err(e) => panic!("ERROR: Npm or Yarn installation is needed.\n{e}"),
    }
}

const fn if_windows(windows: &'static str, unix: &'static str) -> &'static str {
    #[cfg(windows)]
    {
        windows
    }
    #[cfg(unix)]
    {
        unix
    }
}
