use color_print::cformat;
use self_update::cargo_crate_version;

pub(crate) async fn perform_update(console: &crate::term::Terminal) -> anyhow::Result<()> {
    console.info("Checking for update...");

    let status = self_update::backends::github::Update::configure()
        .repo_owner("noaione")
        .repo_name("tosho-mango")
        .bin_name("tosho")
        .show_download_progress(true)
        .current_version(cargo_crate_version!())
        .show_output(console.is_debug())
        .build()?
        .update()?;

    match status {
        self_update::Status::UpToDate(v) => {
            console.info(&cformat!(
                "You are already using the latest version: <m,s>{}</>",
                v
            ));
        }
        self_update::Status::Updated(v) => {
            console.info(&cformat!("Updated to version: <m,s>{}</>", v));
        }
    }

    Ok(())
}
