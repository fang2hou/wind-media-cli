use owo_colors::OwoColorize;

pub fn init_color(no_color: bool) {
	if no_color {
		owo_colors::set_override(false);
	}
}

pub fn print_success(msg: &str) {
	eprintln!("{} {msg}", "✔".green().bold());
}

pub fn print_warning(msg: &str) {
	eprintln!("{} {msg}", "⚠".yellow().bold());
}

pub fn print_error(msg: &str) {
	eprintln!("{} {msg}", "Error:".red().bold());
}

pub fn print_info(label: &str, value: &str) {
	eprintln!("  {}: {}", label.dimmed(), value);
}
