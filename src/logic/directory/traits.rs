trait Constrained {
	fn constrain(self: U) -> Result<Self, Error>
	where
		Self: Sized;
}

struct DirectoriesWithMetadata {
	value: DirEntry
}

impl Constrained for DirectoriesWithMetadata {
	fn constrain(entry: DirEntry) -> Result<DirectoriesWithMetadata, Error> {
		let is_err = entry.metadata().is_err();
		if is_err {
			Err("No metadata")
		} else {
			Ok(DirectoriesWithMetadata::new(entry))
		}
	}
}