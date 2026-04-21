mod cli;
mod cmd;
mod config;
mod error;
mod output;

use clap::{CommandFactory, Parser};
use cli::Command;

fn main() {
	let cli = cli::Cli::parse();

	output::init_color(cli.no_color);

	if let Command::Completion { shell } = cli.command {
		let mut app = cli::Cli::command();
		let name = app.get_name().to_string();
		clap_complete::generate(shell, &mut app, name, &mut std::io::stdout());
		return;
	}

	if let Err(e) = cmd::dispatch(cli) {
		output::print_error(&e.to_string());
		std::process::exit(1);
	}
}
