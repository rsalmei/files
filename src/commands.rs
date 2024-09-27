mod dupes;
mod join;
mod list;
mod rebuild;
mod rename;

use crate::entries::{find_entries, EntryKind, Filters};
use clap::Subcommand;
use std::fmt;
use std::path::PathBuf;

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Find possibly duplicated files by both size and filename.
    Dupes(dupes::Dupes),
    /// Rebuild the filenames of media collections intelligently.
    Rebuild(rebuild::Rebuild),
    /// List files from the given paths.
    List(list::List),
    /// Rename files in batch, according to the given rules.
    Rename(rename::Rename),
    /// Join all files into the same directory.
    Join(join::Join),
}

/// The common interface for commands.
///
/// Implemented for each command's options, conferring its functionality.
pub trait Refine {
    type Media: TryFrom<PathBuf, Error: fmt::Display>;
    const OPENING_LINE: &'static str;
    const ENTRY_KIND: EntryKind;

    fn refine(self, medias: &mut Vec<Self::Media>) -> anyhow::Result<()>;
}

impl Command {
    pub fn run(self, entries: Entries) -> Result<()> {
        match self {
            Command::Dupes(cmd) => run(cmd, entries),
            Command::Rebuild(cmd) => run(cmd, entries),
            Command::List(cmd) => run(cmd, entries),
            Command::Rename(cmd) => run(cmd, entries),
            Command::Join(cmd) => run(cmd, entries),
        }
    }
}

    println!("=> {}\n", R::OPENING_LINE);
    let entries = find_entries(filters, paths, R::ENTRY_KIND)?;
    let mut medias = gen_medias(entries);
    cmd.refine(&mut medias)
}

fn gen_medias<T>(entries: impl Iterator<Item = PathBuf>) -> Vec<T>
where
    T: TryFrom<PathBuf, Error: fmt::Display>,
{
    entries
        .map(|path| T::try_from(path))
        .inspect(|res| {
            if let Err(err) = res {
                eprintln!("error: load media: {err}");
            }
        })
        .flatten()
        .collect()
}
