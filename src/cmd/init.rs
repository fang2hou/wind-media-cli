use std::path::Path;

use crate::config::{self, Config};
use crate::error::WindMediaError;
use crate::output;

pub fn run(addon_dir: &Path, config: Option<&Config>) -> Result<(), WindMediaError> {
	let is_new = !addon_dir.join("data.lua").exists();
	let max_backups = config::resolve_max_backups(config);
	let data = wow_sharedmedia::ensure_addon_dir(addon_dir, max_backups)
		.map_err(WindMediaError::library)?;
	let name = wow_sharedmedia::addon_name(addon_dir);
	let title = wow_sharedmedia::addon_title(name);

	if is_new {
		output::print_success(&format!(
			"Created addon directory: {}",
			addon_dir.display()
		));
	} else {
		output::print_success(&format!(
			"Refreshed addon directory: {}",
			addon_dir.display()
		));
	}
	output::print_info("Name", name);
	output::print_info("Title", title);
	output::print_info("Entries", &data.entries.len().to_string());
	output::print_info("Schema version", &data.schema_version.to_string());

	Ok(())
}
