use std::path::Path;

use crate::error::WindMediaError;
use crate::output;

pub fn run(
	addon_dir: &Path,
	id: &uuid::Uuid,
	key: Option<&str>,
	tags: Option<&str>,
	locales: Option<&str>,
) -> Result<(), WindMediaError> {
	if key.is_none() && tags.is_none() && locales.is_none() {
		return Err(WindMediaError::NoUpdateFields);
	}

	let opts = wow_sharedmedia::UpdateOptions {
		key: key.map(|s| s.to_string()),
		locales: locales.map(crate::cli::parse_csv),
		tags: tags.map(crate::cli::parse_csv),
	};

	let updated =
		wow_sharedmedia::update_media(addon_dir, id, opts).map_err(WindMediaError::library)?;

	output::print_success(&format!(
		"Updated \"{}\" ({})",
		updated.key, updated.media_type
	));
	output::print_info("ID", &updated.id.to_string());
	output::print_info("Type", &updated.media_type.to_string());
	output::print_info("File", &updated.file);
	if !updated.tags.is_empty() {
		output::print_info("Tags", &updated.tags.join(", "));
	}

	Ok(())
}
