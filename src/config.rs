use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::error::WindMediaError;

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
}

fn default_addon_name() -> String {
	"!!!WindMedia".to_string()
}

fn project_dirs() -> Option<directories::ProjectDirs> {
	directories::ProjectDirs::from("", "", "wind-media")
}

pub fn config_dir() -> Option<PathBuf> {
	project_dirs().map(|d| d.config_dir().to_path_buf())
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
			defaults: None,
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
