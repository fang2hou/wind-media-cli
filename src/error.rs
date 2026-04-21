use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum WindMediaError {
	#[error("Config file not found: {0}\n\n  Run `wind-media config init` to create one.")]
	ConfigNotFound(PathBuf),

	#[error(
		"No addon directory configured.\n\n  Either:\n    1. Set it in config:  wind-media config init\n    2. Pass it directly:  wind-media --addon-dir <PATH> <command>"
	)]
	AddonDirNotConfigured,

	#[error("Failed to parse config file at {path}:\n  {detail}")]
	ConfigParse { path: PathBuf, detail: String },

	#[error("{message}")]
	Library {
		message: String,
		#[source]
		source: wow_sharedmedia::Error,
	},

	#[error("I/O error on {path}: {source}")]
	Io {
		#[source]
		source: std::io::Error,
		path: PathBuf,
	},
}

impl WindMediaError {
	pub fn library(err: wow_sharedmedia::Error) -> Self {
		let message = err.to_string();
		Self::Library {
			message,
			source: err,
		}
	}
}

impl From<std::io::Error> for WindMediaError {
	fn from(err: std::io::Error) -> Self {
		Self::Io {
			path: PathBuf::from("<unknown>"),
			source: err,
		}
	}
}
