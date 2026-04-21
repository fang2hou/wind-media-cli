use std::path::Path;

use crate::cli::CliMediaType;
use crate::config::Config;
use crate::error::WindMediaError;
use crate::output;

#[allow(clippy::too_many_arguments)]
pub fn run(
	addon_dir: &Path,
	config: Option<&Config>,
	media_type: CliMediaType,
	key: &str,
	source: &Path,
	locales: Option<&str>,
	tags: Option<&str>,
	no_reject_duplicates: bool,
) -> Result<(), WindMediaError> {
	if key.trim().is_empty() {
		return Err(WindMediaError::InvalidInput("key cannot be empty".into()));
	}

	let mt: wow_sharedmedia::MediaType = media_type.into();
	let mut opts = wow_sharedmedia::ImportOptions::new(mt, key, source);

	if let Some(tags_str) = tags {
		opts.tags = crate::cli::split_comma_list(tags_str)
			.map_err(|e| WindMediaError::InvalidInput(format!("invalid --tags: {e}")))?;
	}
	if let Some(locales_str) = locales {
		opts.locales = crate::cli::split_comma_list(locales_str)
			.map_err(|e| WindMediaError::InvalidInput(format!("invalid --locales: {e}")))?;
	}

	if no_reject_duplicates {
		opts.reject_duplicates = false;
	} else if let Some(cfg) = config
		&& let Some(ref defaults) = cfg.defaults
		&& let Some(reject) = defaults.reject_duplicates
	{
		opts.reject_duplicates = reject;
	}

	let result = wow_sharedmedia::import_media(addon_dir, opts).map_err(WindMediaError::library)?;

	output::print_success(&format!(
		"Imported \"{}\" ({})",
		result.entry.key, result.entry.media_type
	));
	output::print_info("ID", &result.entry.id.to_string());
	output::print_info("File", &result.entry.file);
	if let Some(ref original) = result.entry.original_name {
		output::print_info("Original", original);
	}

	for w in &result.warnings {
		output::print_warning(&format!("[{}] {}", w.code, w.message));
	}

	Ok(())
}
