use crate::config::{self, Config};
use crate::error::WindMediaError;
use crate::output;
pub fn run_init(existing: Option<Config>) -> Result<(), WindMediaError> {
	let cfg = existing.unwrap_or_else(Config::default_config);
	cfg.save()?;
	output::print_success(&format!(
		"Config written to {}",
		config::config_path()
			.map(|p| p.display().to_string())
			.unwrap_or_default()
	));
	Ok(())
}

pub fn run_show(existing: Option<Config>) -> Result<(), WindMediaError> {
	match existing {
		Some(cfg) => {
			output::print_info(
				"Config file",
				&config::config_path()
					.map(|p| p.display().to_string())
					.unwrap_or_default(),
			);
			eprintln!();
			output::print_info("addon.name", &cfg.addon.name);
			if let Some(ref wow_path) = cfg.addon.wow_path {
				output::print_info("addon.wow_path", &wow_path.display().to_string());
			}
			if let Some(ref dir) = cfg.addon.dir {
				output::print_info("addon.dir", &dir.display().to_string());
			}
			match cfg.resolve_addon_dir() {
				Some(ref resolved) => {
					output::print_info("Resolved path", &resolved.display().to_string());
				}
				None => {
					output::print_info("Resolved path", "<not configured>");
				}
			}
			if let Some(ref defaults) = cfg.defaults {
				eprintln!();
				if let Some(ref locales) = defaults.locales {
					output::print_info("defaults.locales", &locales.join(", "));
				}
				if let Some(reject) = defaults.reject_duplicates {
					output::print_info("defaults.reject_duplicates", &reject.to_string());
				}
				if let Some(max_backups) = defaults.max_backups {
					output::print_info("defaults.max_backups", &max_backups.to_string());
				}
			}
		}
		None => {
			output::print_info("Config", "<not found>");
		}
	}
	Ok(())
}

pub fn run_path() -> Result<(), WindMediaError> {
	match config::config_path() {
		Some(p) => println!("{}", p.display()),
		None => return Err(WindMediaError::ConfigDirUnavailable),
	}
	Ok(())
}
