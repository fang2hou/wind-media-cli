use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(
	name = "wind-media",
	about = "Manage the Wind Media WoW SharedMedia addon"
)]
pub struct Cli {
	/// Override addon directory path
	#[arg(long, global = true)]
	pub addon_dir: Option<PathBuf>,

	/// Disable colored output
	#[arg(long, global = true, env = "NO_COLOR")]
	pub no_color: bool,

	#[command(subcommand)]
	pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
	/// Initialize the addon directory structure
	Init,
	/// Import a media file into the addon
	Import {
		/// Media type
		media_type: CliMediaType,
		/// Display key for the entry
		key: String,
		/// Path to the source file
		source: PathBuf,
		/// Comma-separated locale names (fonts only)
		#[arg(long)]
		locales: Option<String>,
		/// Comma-separated tags
		#[arg(long)]
		tags: Option<String>,
		/// Allow duplicate keys instead of rejecting
		#[arg(long)]
		no_reject_duplicates: bool,
	},
	/// List all registered media entries
	List,
	/// Remove a media entry by UUID
	Remove {
		/// UUID of the entry to remove
		id: uuid::Uuid,
	},
	/// Update an existing media entry
	Update {
		/// UUID of the entry to update
		id: uuid::Uuid,
		/// New display key
		#[arg(long)]
		key: Option<String>,
		/// Comma-separated tags
		#[arg(long)]
		tags: Option<String>,
		/// Comma-separated locale names (fonts only)
		#[arg(long)]
		locales: Option<String>,
	},
	/// Show addon directory information
	Info,
	/// Create a default config file
	ConfigInit,
	/// Show current config and resolved paths
	ConfigShow,
	/// Print config file path
	ConfigPath,
	/// Generate shell completions
	Completion {
		/// Shell to generate completions for
		shell: clap_complete::Shell,
	},
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum CliMediaType {
	Statusbar,
	Background,
	Border,
	Font,
	Sound,
}

impl From<CliMediaType> for wow_sharedmedia::MediaType {
	fn from(mt: CliMediaType) -> Self {
		match mt {
			CliMediaType::Statusbar => Self::Statusbar,
			CliMediaType::Background => Self::Background,
			CliMediaType::Border => Self::Border,
			CliMediaType::Font => Self::Font,
			CliMediaType::Sound => Self::Sound,
		}
	}
}

impl std::fmt::Display for CliMediaType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Statusbar => write!(f, "statusbar"),
			Self::Background => write!(f, "background"),
			Self::Border => write!(f, "border"),
			Self::Font => write!(f, "font"),
			Self::Sound => write!(f, "sound"),
		}
	}
}

pub fn parse_csv(s: &str) -> Vec<String> {
	s.split(',')
		.map(|v| v.trim().to_string())
		.filter(|v| !v.is_empty())
		.collect()
}
