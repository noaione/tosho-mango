use color_eyre::eyre::Context;
use color_print::cformat;
use tosho_amap::AMClient;

use super::{common::do_print_search_information, config::Config};

pub(crate) async fn amap_my_favorites(
    client: &AMClient,
    acc_info: &Config,
    console: &crate::term::Terminal,
) -> color_eyre::Result<()> {
    console.info(cformat!(
        "Getting favorites list for <magenta,bold>{}</>...",
        acc_info.email
    ));

    let results = client
        .get_favorites()
        .await
        .context("Failed to fetch favorites")?;

    if results.comics().is_empty() {
        console.error("You don't have any favorites.");
        return Ok(());
    }

    console.info(cformat!(
        "Your favorites list (<m,s>{}</> results):",
        results.comics().len()
    ));

    do_print_search_information(results.comics(), false, None);

    Ok(())
}
