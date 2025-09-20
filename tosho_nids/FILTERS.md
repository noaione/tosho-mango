## tosho-nids - Filters

The `Filter` struct allows you to customize your requests to the NI API.
You can set various parameters to filter the results according to your needs.

For example, in the issues endpoint, you can use this `filter`:
- `title`: Filter issues by title.
- `series_run_id`: Filter issues by series run ID.
- `publisher_id`: Filter issues by publisher ID.
- `release_date_start`: Filter issues released after a specific date, this is in RFC 3339 format, e.g., `2023-10-01T00:00:00Z`.
- `release_date_end`: Filter issues released before a specific date, this is in RFC 3339 format, e.g., `2023-10-01T00:00:00Z`.
- `format`: Currently unknown? But on the website, they use `issue,ashcan` when requesting in the series page.

On the books endpoint, both marketplace and your collection, you can use:
- `series_run_id`: Filter marketplace editions by series run ID.

On the series run page, you can use:
- `publisher_slug`: Filter series runs by publisher slug.

There is also something called `scope` that you could use in the issues endpoint:
- `Frontlist`/`frontlist`: Recently released issues.
- `Backlist`/`backlist`: Recently added issues that are not new releases.
- `OnSale`/`on_sale`: Issues that are currently on sale.

In the CLI version, you can use these filters like this:

```bash
$ tosho ni issues --filter "title=Batman"
```

It could be repeated too:

```bash
$ tosho ni issues --filter "title=Batman" --filter "series_run_id=1476"
```
