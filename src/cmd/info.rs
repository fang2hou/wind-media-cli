use std::path::Path;

use crate::error::WindMediaError;
use crate::output;

pub fn run(addon_dir: &Path) -> Result<(), WindMediaError> {
	let data = wow_sharedmedia::read_data(addon_dir).map_err(WindMediaError::library)?;
	let name = wow_sharedmedia::addon_name(addon_dir);
	let title = wow_sharedmedia::addon_title(name);

	output::print_info("Addon", &format!("{name} ({title})"));
	output::print_info("Path", &addon_dir.display().to_string());
	output::print_info("Version", &data.version);
	output::print_info("Entries", &data.entries.len().to_string());
	eprintln!();

	if data.entries.is_empty() {
		return Ok(());
	}

	let mut counts: std::collections::BTreeMap<String, usize> = std::collections::BTreeMap::new();
	for entry in &data.entries {
		*counts.entry(entry.media_type.to_string()).or_insert(0) += 1;
	}

	let types: Vec<_> = counts.iter().collect();
	for (i, (mt, count)) in types.iter().enumerate() {
		let connector = if i == types.len() - 1 {
			"└─"
		} else {
			"├─"
		};
		output::print_info(&format!("{connector} {mt}"), &format!("{count} entries"));
	}

	Ok(())
}
