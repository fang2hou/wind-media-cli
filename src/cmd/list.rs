use std::path::Path;

use comfy_table::{ContentArrangement, Table, modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL};

use crate::error::WindMediaError;
use crate::output;

pub fn run(addon_dir: &Path) -> Result<(), WindMediaError> {
	let data = wow_sharedmedia::read_data(addon_dir).map_err(WindMediaError::library)?;

	if data.entries.is_empty() {
		output::print_info("Entries", "none");
		return Ok(());
	}

	let mut table = Table::new();
	table
		.load_preset(UTF8_FULL)
		.apply_modifier(UTF8_ROUND_CORNERS)
		.set_content_arrangement(ContentArrangement::Dynamic)
		.set_header(vec!["ID", "TYPE", "KEY", "FILE"]);

	for entry in &data.entries {
		table.add_row(vec![
			output::short_uuid(entry.id),
			entry.media_type.to_string(),
			entry.key.clone(),
			entry.file.clone(),
		]);
	}

	println!("{table}");
	eprintln!("{} entries", data.entries.len());

	Ok(())
}
