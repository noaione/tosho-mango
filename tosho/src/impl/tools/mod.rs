use std::path::PathBuf;

use clap::Subcommand;

pub(crate) mod merger;

#[derive(Subcommand)]
pub(crate) enum ToolsCommands {
    /// Automatically merge multiple folders of split chapters into one folder
    ///
    /// The chapter merging would depends on _info.json file generated automatically.
    #[command(name = "automerge")]
    AutoMerge {
        /// Input directory to use that contains the _info.json file and split chapters
        input_folder: PathBuf,
        /// Skip the last chapter merge, useful since the last chapter might not have full split chapters yet.
        #[arg(short, long)]
        skip_last: bool,
    },
    /// Merge multiple folders of split chapters into one folder
    ///
    /// The chapter merging would depends on _info.json file generated automatically.
    Merge {
        /// Input directory to use that contains the _info.json file and split chapters
        input_folder: PathBuf,
        /// Ignore the _info_manual_merge.json file which will filter out the chapters to merge
        #[arg(short = 's', long = "ignore-manual")]
        ignore_manual_merge: bool,
    },
}
