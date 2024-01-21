use derive_more::{From, Into};
use std::{error::Error, fs, fs::DirEntry, io};

pub trait Constrained<U, C, E> {
    fn constrain(unconstrained: U) -> Result<C, E>;
}
/// todo bonus: supplier alternative?
/// the idea would be instead of making a trait, use a function so i don't have to make a struct for each nest
#[derive(Debug, From)]
pub struct EntriesWithMetadata(fs::DirEntry);

impl Constrained<DirEntry, EntriesWithMetadata, io::Error> for EntriesWithMetadata {
    fn constrain(entry: DirEntry) -> Result<EntriesWithMetadata, io::Error> {
        let is_err = entry.metadata().is_err();
        if is_err {
            Err(entry.metadata().unwrap_err())
        } else {
            Ok(EntriesWithMetadata::from(entry))
        }
    }
}
