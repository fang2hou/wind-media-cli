use std::path::Path;

use crate::config;
use crate::error::WindMediaError;
use crate::output;

pub fn run(
	addon_dir: &Path,
	id: &uuid::Uuid,
	config: Option<&config::Config>,
) -> Result<(), WindMediaError> {
	let max_backups = config::resolve_max_backups(config);
	let removed = wow_sharedmedia::remove_media(addon_dir, id, max_backups)
		.map_err(WindMediaError::library)?;

	output::print_success(&format!(
		"Removed \"{}\" ({})",
		removed.entry.key, removed.entry.media_type
	));
	output::print_info("ID", &removed.entry.id.to_string());
	output::print_info("Deleted file", &removed.deleted_file.display().to_string());

	Ok(())
}
