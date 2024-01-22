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
}
