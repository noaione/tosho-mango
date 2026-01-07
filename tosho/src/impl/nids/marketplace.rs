use std::collections::HashMap;

use crate::{
    r#impl::nids::common::{PaginateAction, fmt_date, pagination_helper},
    linkify,
};
use color_eyre::eyre::Context;
use color_print::cformat;
use num_format::{Locale, ToFormattedString};
use tosho_nids::constants::BASE_HOST;

fn print_marketplace_edition(
    edition: &tosho_nids::models::MarketplaceDetailedEdition,
    console: &crate::term::Terminal,
) {
    let issue = edition.issue();
    let item_url = format!("https://{}/item/{}/{}", BASE_HOST, issue.id(), issue.slug());
    let linked_title = linkify!(&item_url, issue.full_title());

    let title_text = cformat!(
        "<s>{}</s> (<m,s>{}</m,s> / {})",
        linked_title,
        issue.id(),
        issue.uuid()
    );

    let title_smol_info = cformat!(
        "Edition <s>#{}</s> | <s>{}</s> | <g,s>$</g,s>{:.2}",
        edition.index(),
        issue.publisher().name(),
        tosho_nids::format_price(edition.price_usd())
    );

    let sold_by_info = cformat!(
        "Sold by <s>{}</s> | <b,s>{}</b,s>",
        edition.seller().username(),
        fmt_date(edition.created_at())
    );

    console.info(format!("  {}", title_text));
    console.info(format!("   {}", item_url));
    console.info(format!("   {}", title_smol_info));
    console.info(format!("   {}", sold_by_info));
}

fn print_marketplace_books(
    book: &tosho_nids::models::MarketplaceBook,
    console: &crate::term::Terminal,
) {
    let item_url = format!("https://{}/item/{}/{}", BASE_HOST, book.id(), book.slug());
    let linked_title = linkify!(&item_url, book.full_title());

    let title_text = cformat!(
        "<s>{}</s> (<m,s>{}</m,s> / {})",
        linked_title,
        book.id(),
        book.uuid()
    );

    let pubs_text = if let Some(imprint) = book.imprint()
        && let Some(imprint_type) = imprint.imprint_type()
        && imprint_type != "primary"
    {
        cformat!(
            "<s>{}</s> / <s>{}</s>",
            book.publisher().name(),
            imprint.name()
        )
    } else {
        cformat!("<s>{}</s>", book.publisher().name())
    };

    let mut smol_infos = vec![
        cformat!(
            "<s>{}</s> Editions",
            book.marketplace_count().to_formatted_string(&Locale::en)
        ),
        cformat!(
            "<s>{}</s> Remarques",
            book.marketplace_remarque_count()
                .to_formatted_string(&Locale::en)
        ),
        pubs_text,
    ];
    let (min_price, max_price) = (book.minimum_price(), book.maximum_price());

    if min_price == max_price {
        smol_infos.push(cformat!(
            "<g,s>$</g,s>{:.2}",
            tosho_nids::format_price(min_price)
        ))
    } else {
        smol_infos.push(cformat!(
            "<g,s>$</g,s>{:.2} - <g,s>$</g,s>{:.2}",
            tosho_nids::format_price(min_price),
            tosho_nids::format_price(max_price)
        ));
    }

    console.info(format!("  {}", title_text));
    console.info(format!("   {}", item_url));
    console.info(format!("   {}", smol_infos.join(" | ")));
}

pub async fn nids_get_marketplace_ungrouped(
    base_filter: &mut tosho_nids::Filter,
    client: &tosho_nids::NIClient,
    console: &crate::term::Terminal,
) -> color_eyre::Result<()> {
    console.info("Fetching initial marketplace with provided filters");
    let marketplace_editions = client
        .get_marketplace_editions(Some(base_filter))
        .await
        .context("Failed to get marketplace editions")?;

    if marketplace_editions.data().is_empty() {
        console.warn("No editions found with the provided filters");
        return Ok(());
    }

    if marketplace_editions.pages() > 1 {
        // Do paginated
        let mut current_page: u32 = 1;
        let mut maximum_pages: u32 = marketplace_editions.pages();
        let mut collected_editions: HashMap<
            u32,
            Vec<tosho_nids::models::MarketplaceDetailedEdition>,
        > = HashMap::from([(1, marketplace_editions.data().to_vec())]);
        let mut current_data = collected_editions.get(&1).expect("We just inserted this");

        loop {
            console.info(cformat!(
                "Showing page <magenta,bold>{}</> of <magenta,bold>{}</>:",
                current_page,
                maximum_pages
            ));
            for edition in current_data.iter() {
                print_marketplace_edition(edition, console);
            }
            if current_data.is_empty() {
                console.info("No editions found on this page.");
            }

            match pagination_helper(current_page, maximum_pages, console).await {
                PaginateAction::Next => {
                    current_page += 1;
                }
                PaginateAction::Previous => {
                    if current_page > 1 {
                        current_page -= 1;
                    }
                }
                PaginateAction::Exit => {
                    break;
                }
            }

            // Fetch new stuff
            base_filter.set_page(current_page);
            if let Some(editions) = collected_editions.get(&current_page) {
                current_data = editions;
                console.clear_screen();
            } else {
                console.info(cformat!("Loading page <m,s>{}</m,s>...", current_page));
                let new_editions = client
                    .get_marketplace_editions(Some(base_filter))
                    .await
                    .context("Failed to get marketplace editions")?;

                console.clear_screen();

                maximum_pages = new_editions.pages();
                // add correct data to collected_issues
                collected_editions.insert(current_page, new_editions.data().to_vec());
                current_data = collected_editions
                    .get(&current_page)
                    .expect("Somehow missing page after insert");
            }
        }
    } else {
        for edition in marketplace_editions.data() {
            print_marketplace_edition(edition, console);
        }
    }

    Ok(())
}

pub async fn nids_get_marketplace_grouped(
    base_filter: &mut tosho_nids::Filter,
    client: &tosho_nids::NIClient,
    console: &crate::term::Terminal,
) -> color_eyre::Result<()> {
    console.info("Fetching initial marketplace with provided filters");
    let marketplace_books = client
        .get_marketplace_books(Some(base_filter))
        .await
        .context("Failed to get marketplace books")?;

    if marketplace_books.data().is_empty() {
        console.warn("No books found with the provided filters");
        return Ok(());
    }

    if marketplace_books.pages() > 1 {
        // Do paginated
        let mut current_page: u32 = 1;
        let mut maximum_pages: u32 = marketplace_books.pages();
        let mut collected_books: HashMap<u32, Vec<tosho_nids::models::MarketplaceBook>> =
            HashMap::from([(1, marketplace_books.data().to_vec())]);
        let mut current_data = collected_books.get(&1).expect("We just inserted this");

        loop {
            console.info(cformat!(
                "Showing page <magenta,bold>{}</> of <magenta,bold>{}</>:",
                current_page,
                maximum_pages
            ));
            for book in current_data.iter() {
                print_marketplace_books(book, console);
            }
            if current_data.is_empty() {
                console.info("No books found on this page.");
            }

            match pagination_helper(current_page, maximum_pages, console).await {
                PaginateAction::Next => {
                    current_page += 1;
                }
                PaginateAction::Previous => {
                    if current_page > 1 {
                        current_page -= 1;
                    }
                }
                PaginateAction::Exit => {
                    break;
                }
            }

            // Fetch new stuff
            base_filter.set_page(current_page);
            if let Some(books) = collected_books.get(&current_page) {
                current_data = books;
                console.clear_screen();
            } else {
                console.info(cformat!("Loading page <m,s>{}</m,s>...", current_page));
                let new_books = client
                    .get_marketplace_books(Some(base_filter))
                    .await
                    .context("Failed to get marketplace books")?;

                console.clear_screen();

                maximum_pages = new_books.pages();
                // add correct data to collected_issues
                collected_books.insert(current_page, new_books.data().to_vec());
                current_data = collected_books
                    .get(&current_page)
                    .expect("Somehow missing page after insert");
            }
        }
    } else {
        for book in marketplace_books.data() {
            print_marketplace_books(book, console);
        }
    }

    Ok(())
}
