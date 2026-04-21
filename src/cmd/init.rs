use std::path::Path;

use crate::error::WindMediaError;
use crate::output;

pub fn run(addon_dir: &Path) -> Result<(), WindMediaError> {
	let data = wow_sharedmedia::ensure_addon_dir(addon_dir).map_err(WindMediaError::library)?;
	let name = wow_sharedmedia::addon_name(addon_dir);
	let title = wow_sharedmedia::addon_title(name);

	output::print_success(&format!(
		"Initialized addon directory: {}",
		addon_dir.display()
	));
	output::print_info("Name", name);
	output::print_info("Title", title);
	output::print_info("Entries", &data.entries.len().to_string());
	output::print_info("Schema version", &data.schema_version.to_string());

	Ok(())
}
