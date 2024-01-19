use std::{error::Error, fs::DirEntry};

pub(crate) trait Constrained<U, C> 
	where
	Self: Sized, {
	fn constrain(unconstrained: U) -> Result<C, Box<dyn Error>>;
}

pub struct DirectoriesWithMetadata {
	pub access: DirEntry
}

impl Constrained<DirEntry, DirectoriesWithMetadata> for DirectoriesWithMetadata {
	fn constrain(entry: DirEntry) -> Result<DirectoriesWithMetadata, Box<dyn Error>> {
		let is_err = entry.metadata().is_err();
		if is_err {
			Err("No metadata".into())
		} else {
			Ok(DirectoriesWithMetadata {access: entry})
		}
	}
}