pub mod config_cmd;
pub mod import;
pub mod info;
pub mod init;
pub mod list;
pub mod remove;
pub mod update;

use std::path::PathBuf;

use crate::cli::{Cli, Command};
use crate::config::Config;
use crate::error::WindMediaError;

fn resolve_addon_dir(cli: &Cli, config: Option<&Config>) -> Result<PathBuf, WindMediaError> {
	let dir = if let Some(ref dir) = cli.addon_dir {
		dir.clone()
	} else if let Some(config) = config
		&& let Some(dir) = config.resolve_addon_dir()
	{
		dir
	} else {
		return Err(WindMediaError::AddonDirNotConfigured);
	};

	if dir.file_name().is_none() {
		return Err(WindMediaError::AddonDirNotConfigured);
	}
	Ok(dir)
}

fn resolve_existing_addon_dir(
	cli: &Cli,
	config: Option<&Config>,
) -> Result<PathBuf, WindMediaError> {
	let dir = resolve_addon_dir(cli, config)?;
	if !dir.join("data.lua").exists() {
		return Err(WindMediaError::InvalidInput(format!(
			"Not an addon directory (missing data.lua): {}\n\n  Run `wind-media init` first, or check the path.",
			dir.display()
		)));
	}
	Ok(dir)
}

pub fn dispatch(cli: Cli) -> Result<(), WindMediaError> {
	let config = match Config::load() {
		Ok(c) => Some(c),
		Err(WindMediaError::ConfigNotFound(_)) => None,
		Err(e) => return Err(e),
	};

	match cli.command {
		Command::Init => {
			let dir = resolve_addon_dir(&cli, config.as_ref())?;
			init::run(&dir, config.as_ref())?;
		}
		Command::Import {
			media_type,
			ref key,
			ref source,
			ref locales,
			ref tags,
			no_reject_duplicates,
		} => {
			let dir = resolve_addon_dir(&cli, config.as_ref())?;
			import::run(
				&dir,
				config.as_ref(),
				media_type,
				key,
				source,
				locales.as_deref(),
				tags.as_deref(),
				no_reject_duplicates,
			)?;
		}
		Command::List => {
			let dir = resolve_existing_addon_dir(&cli, config.as_ref())?;
			list::run(&dir)?;
		}
		Command::Remove { id } => {
			let dir = resolve_existing_addon_dir(&cli, config.as_ref())?;
			remove::run(&dir, &id, config.as_ref())?;
		}
		Command::Update {
			id,
			ref key,
			ref tags,
			ref locales,
		} => {
			let dir = resolve_existing_addon_dir(&cli, config.as_ref())?;
			update::run(
				&dir,
				&id,
				key.as_deref(),
				tags.as_deref(),
				locales.as_deref(),
				config.as_ref(),
			)?;
		}
		Command::Info => {
			let dir = resolve_existing_addon_dir(&cli, config.as_ref())?;
			info::run(&dir)?;
		}
		Command::ConfigInit => config_cmd::run_init(config)?,
		Command::ConfigShow => config_cmd::run_show(config)?,
		Command::ConfigPath => config_cmd::run_path()?,
		Command::Completion { .. } => {}
	}

	Ok(())
}
