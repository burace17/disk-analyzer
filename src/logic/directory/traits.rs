use std::{error::Error, fs::DirEntry};

pub(crate) trait Constrained<U, C>
where
    Self: Sized,
{
    fn constrain(unconstrained: U) -> Result<C, Box<dyn Error>>;
}
/// todo bonus: supplier alternative?
/// the idea would be instead of making a trait, use a function so i don't have to make a struct for each nest
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct EntriesWithMetadata {
    pub access: DirEntry,
}

impl Constrained<DirEntry, EntriesWithMetadata> for EntriesWithMetadata {
    fn constrain(entry: DirEntry) -> Result<EntriesWithMetadata, Box<dyn Error>> {
        let is_err = entry.metadata().is_err();
        if is_err {
            Err("No metadata".into())
        } else {
            Ok(EntriesWithMetadata { access: entry })
        }
    }
}
