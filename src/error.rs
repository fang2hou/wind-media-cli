use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum WindMediaError {
	#[error("Config file not found: {0}\n\n  Run `wind-media config-init` to create one.")]
	ConfigNotFound(PathBuf),

	#[error(
		"No addon directory configured.\n\n  Either:\n    1. Set it in config:  wind-media config-init\n    2. Pass it directly:  wind-media --addon-dir <PATH> <command>"
	)]
	AddonDirNotConfigured,

	#[error("Failed to parse config file at {path}:\n  {detail}")]
	ConfigParse { path: PathBuf, detail: String },

	#[error("{source:#}")]
	Library {
		#[source]
		source: wow_sharedmedia::Error,
	},

	#[error("I/O error on {path}: {source}")]
	Io {
		#[source]
		source: std::io::Error,
		path: PathBuf,
	},

	#[error("Nothing to update. Provide at least one of --key, --tags, or --locales.")]
	NoUpdateFields,

	#[error(
		"Could not determine config directory.\n\n  Your system does not support standard config paths."
	)]
	ConfigDirUnavailable,

	#[error("Invalid input: {0}")]
	InvalidInput(String),
}

impl WindMediaError {
	pub fn library(err: wow_sharedmedia::Error) -> Self {
		Self::Library { source: err }
	}
}
