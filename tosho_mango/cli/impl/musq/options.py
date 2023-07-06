import click

__all__ = ("account_id",)


account_id = click.option(
    "-a",
    "--account",
    "account_id",
    type=str,
    default=None,
    help="Account ID to use",
    required=False,
)
