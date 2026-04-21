use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::error::WindMediaError;

/// CLI default for the maximum number of data.lua backups retained per write.
pub const DEFAULT_MAX_BACKUPS: u32 = 5;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
	#[serde(default)]
	pub addon: AddonConfig,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub defaults: Option<DefaultsConfig>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AddonConfig {
	#[serde(default = "default_addon_name")]
	pub name: String,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub wow_path: Option<PathBuf>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub dir: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultsConfig {
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub locales: Option<Vec<String>>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub reject_duplicates: Option<bool>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub max_backups: Option<u32>,
}

fn default_addon_name() -> String {
	"!!!WindMedia".to_string()
}

/// Resolve the XDG-style config directory across all platforms.
///
/// - Unix (macOS, Linux): `$XDG_CONFIG_HOME/wind-media` (defaults to `~/.config/wind-media`)
/// - Windows: `C:\Users\<Username>\.config\wind-media` (respects `$XDG_CONFIG_HOME`)
pub fn config_dir() -> Option<PathBuf> {
	#[cfg(unix)]
	{
		if let Ok(xdg) = std::env::var("XDG_CONFIG_HOME") {
			let p = PathBuf::from(xdg);
			if p.is_absolute() {
				return Some(p.join("wind-media"));
			}
		}
		std::env::var("HOME")
			.ok()
			.map(|h| PathBuf::from(h).join(".config").join("wind-media"))
	}

	#[cfg(windows)]
	{
		if let Ok(xdg) = std::env::var("XDG_CONFIG_HOME") {
			let p = PathBuf::from(xdg);
			if p.is_absolute() {
				return Some(p.join("wind-media"));
			}
		}
		std::env::var("USERPROFILE")
			.ok()
			.map(|h| PathBuf::from(h).join(".config").join("wind-media"))
	}
}

pub fn config_path() -> Option<PathBuf> {
	config_dir().map(|d| d.join("config.toml"))
}

impl Config {
	pub fn default_config() -> Self {
		Self {
			addon: AddonConfig {
				name: default_addon_name(),
				wow_path: None,
				dir: None,
			},
			defaults: Some(DefaultsConfig {
				locales: None,
				reject_duplicates: None,
				max_backups: Some(DEFAULT_MAX_BACKUPS),
			}),
		}
	}

	pub fn load() -> Result<Self, WindMediaError> {
		let path = config_path().ok_or_else(|| {
			WindMediaError::ConfigNotFound(PathBuf::from(
				"<platform config dir>/wind-media/config.toml",
			))
		})?;

		if !path.exists() {
			return Err(WindMediaError::ConfigNotFound(path));
		}

		let content = std::fs::read_to_string(&path).map_err(|e| WindMediaError::Io {
			source: e,
			path: path.clone(),
		})?;

		let config: Config = toml::from_str(&content).map_err(|e| WindMediaError::ConfigParse {
			path,
			detail: e.to_string(),
		})?;

		Ok(config)
	}

	pub fn save(&self) -> Result<(), WindMediaError> {
		let dir = config_dir().ok_or_else(|| {
			WindMediaError::ConfigNotFound(PathBuf::from(
				"<platform config dir>/wind-media/config.toml",
			))
		})?;

		std::fs::create_dir_all(&dir).map_err(|e| WindMediaError::Io {
			source: e,
			path: dir.clone(),
		})?;

		let path = dir.join("config.toml");
		let content = toml::to_string_pretty(self).map_err(|e| WindMediaError::ConfigParse {
			path: path.clone(),
			detail: e.to_string(),
		})?;

		let tmp_path = dir.join("config.toml.tmp");
		std::fs::write(&tmp_path, &content).map_err(|e| WindMediaError::Io {
			source: e,
			path: tmp_path.clone(),
		})?;
		std::fs::rename(&tmp_path, &path).map_err(|e| WindMediaError::Io { source: e, path })?;

		Ok(())
	}

	pub fn resolve_addon_dir(&self) -> Option<PathBuf> {
		if let Some(ref dir) = self.addon.dir {
			return Some(dir.clone());
		}

		if let Some(ref wow_path) = self.addon.wow_path {
			return Some(wow_path.join("Interface/AddOns").join(&self.addon.name));
		}

		None
	}
}

pub fn resolve_max_backups(config: Option<&Config>) -> u32 {
	config
		.and_then(|c| c.defaults.as_ref())
		.and_then(|d| d.max_backups)
		.unwrap_or(DEFAULT_MAX_BACKUPS)
}
