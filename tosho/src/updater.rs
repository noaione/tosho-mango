use color_print::cformat;
use self_update::cargo_crate_version;

const UPDATE_CHECK_EVERY: i64 = 60 * 60 * 24; // 24 hours in seconds

fn create_updater(
    debug: bool,
) -> Result<Box<dyn self_update::update::ReleaseUpdate>, self_update::errors::Error> {
    self_update::backends::github::Update::configure()
        .repo_owner("noaione")
        .repo_name("tosho-mango")
        .bin_name("tosho")
        .show_download_progress(true)
        .current_version(cargo_crate_version!())
        .show_output(debug)
        .build()
}

pub(crate) async fn get_last_update_check_time() -> anyhow::Result<i64> {
    let target_file = crate::config::get_user_path().join("last_update_check.tmd");

    if !target_file.exists() {
        return Ok(0);
    }

    let content = tokio::fs::read_to_string(&target_file).await?;
    let content = content.trim();

    let time = content.parse::<i64>()?;

    Ok(time)
}

async fn write_last_update_check_time(time: i64) -> anyhow::Result<()> {
    let target_file = crate::config::get_user_path().join("last_update_check.tmd");

    tokio::fs::write(&target_file, time.to_string()).await?;

    Ok(())
}

pub(crate) async fn check_for_update(console: &crate::term::Terminal) -> anyhow::Result<()> {
    // Only check for update once every 24 hours
    let last_check = get_last_update_check_time().await?;
    let now = chrono::Utc::now().timestamp();
    if last_check + UPDATE_CHECK_EVERY > now {
        return Ok(());
    }

    let updater = create_updater(console.is_debug())?;

    let latest_version = updater.get_latest_release()?;
    let current_version = updater.current_version();

    if self_update::version::bump_is_greater(&current_version, &latest_version.version)? {
        console.info(&cformat!(
            "There is a new version available: <m,s>{}</>",
            latest_version.version
        ));
        console.info(&cformat!(
            "Update now by running <m,s>tosho update</> or <m,s>cargo [b]install tosho</>!",
        ));
    }

    write_last_update_check_time(chrono::Utc::now().timestamp()).await?;

    Ok(())
}

pub(crate) async fn perform_update(console: &crate::term::Terminal) -> anyhow::Result<()> {
    console.info("Checking for update...");

    let status = create_updater(console.is_debug())?.update()?;

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
