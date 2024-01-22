use std::process::Command;

extern crate winres;

fn main() {
    if cfg!(target_os = "windows") {
        let name = env!("CARGO_PKG_NAME");
        let version = env!("CARGO_PKG_VERSION");
        let description = env!("CARGO_PKG_DESCRIPTION");

        let mut res = winres::WindowsResource::new();
        res.set_icon("assets/tosho-logo.ico");
        res.set("ProductName", name);
        res.set("ProductVersion", version);
        res.set("FileVersion", version);
        res.set("FileDescription", description);
        res.set("OriginalFilename", &format!("{}.exe", name).to_uppercase());
        res.set("LegalCopyright", "Copyright © 2023 noaione");
        res.compile().unwrap();
    }

    // check for RELEASE environment variable is set to true
    // if it is, we will not set VERSION_WITH_HASH
    if option_env!("RELEASE").is_none() {
        let commit = Command::new("git")
            .args(&["rev-parse", "--short", "HEAD"])
            .output();
        match commit {
            Ok(commit) => {
                let commit = String::from_utf8_lossy(&commit.stdout);
                println!(
                    "cargo:rustc-env=VERSION_WITH_HASH={}-{}",
                    env!("CARGO_PKG_VERSION"),
                    commit
                );
            }
            Err(_) => {
                println!("cargo:rustc-env=VERSION_WITH_HASH=unknown");
            }
        }
    }
}
