fn create_test_png(path: &std::path::Path) {
	use std::io::Cursor;
	let img = image::DynamicImage::ImageRgba8(image::ImageBuffer::from_pixel(
		2,
		2,
		image::Rgba([255, 255, 255, 255]),
	));
	let mut buf = Cursor::new(Vec::new());
	img.write_to(&mut buf, image::ImageFormat::Png).unwrap();
	std::fs::write(path, buf.into_inner()).unwrap();
}

fn max_backups() -> u32 {
	wind_media_cli::config::DEFAULT_MAX_BACKUPS
}

#[test]
fn e2e_full_lifecycle() {
	let tmp = tempfile::TempDir::new().unwrap();
	let addon_dir = tmp.path().join("TestAddon");

	wow_sharedmedia::ensure_addon_dir(&addon_dir, max_backups()).unwrap();
	assert!(addon_dir.join("data.lua").exists());

	let source = tmp.path().join("bar.png");
	create_test_png(&source);

	let result = wow_sharedmedia::import_media(
		&addon_dir,
		wow_sharedmedia::ImportOptions::new(wow_sharedmedia::MediaType::Statusbar, "Bar", &source),
		max_backups(),
	)
	.unwrap();
	assert_eq!(result.entry.key, "Bar");
	assert_eq!(
		result.entry.media_type,
		wow_sharedmedia::MediaType::Statusbar
	);
	assert!(addon_dir.join(&result.entry.file).exists());

	let id = result.entry.id;

	let source2 = tmp.path().join("bg.png");
	create_test_png(&source2);

	let result2 = wow_sharedmedia::import_media(
		&addon_dir,
		wow_sharedmedia::ImportOptions::new(wow_sharedmedia::MediaType::Background, "BG", &source2),
		max_backups(),
	)
	.unwrap();
	assert_eq!(result2.entry.key, "BG");
	assert_eq!(
		result2.entry.media_type,
		wow_sharedmedia::MediaType::Background
	);

	let data = wow_sharedmedia::read_data(&addon_dir).unwrap();
	assert_eq!(data.entries.len(), 2);

	let updated = wow_sharedmedia::update_media(
		&addon_dir,
		&id,
		wow_sharedmedia::UpdateOptions {
			key: Some("Renamed Bar".to_string()),
			locales: None,
			tags: Some(vec!["renamed".to_string()]),
		},
		max_backups(),
	)
	.unwrap();
	assert_eq!(updated.key, "Renamed Bar");
	assert_eq!(updated.tags, vec!["renamed"]);

	let removed = wow_sharedmedia::remove_media(&addon_dir, &id, max_backups()).unwrap();
	assert_eq!(removed.entry.key, "Renamed Bar");
	assert!(!addon_dir.join(&removed.deleted_file).exists());

	let final_data = wow_sharedmedia::read_data(&addon_dir).unwrap();
	assert_eq!(final_data.entries.len(), 1);
	assert_eq!(final_data.entries[0].key, "BG");
}

#[test]
fn e2e_duplicate_rejection() {
	let tmp = tempfile::TempDir::new().unwrap();
	let addon_dir = tmp.path().join("DupeAddon");

	wow_sharedmedia::ensure_addon_dir(&addon_dir, max_backups()).unwrap();

	let source = tmp.path().join("same.png");
	create_test_png(&source);

	wow_sharedmedia::import_media(
		&addon_dir,
		wow_sharedmedia::ImportOptions::new(wow_sharedmedia::MediaType::Statusbar, "Same", &source),
		max_backups(),
	)
	.unwrap();

	let result = wow_sharedmedia::import_media(
		&addon_dir,
		wow_sharedmedia::ImportOptions::new(wow_sharedmedia::MediaType::Statusbar, "Same", &source),
		max_backups(),
	);
	assert!(result.is_err());
}

#[test]
fn e2e_invalid_media_type_string() {
	let result = "invalid_type".parse::<wow_sharedmedia::MediaType>();
	assert!(result.is_err());
}

#[test]
fn e2e_config_default_and_resolve() {
	let cfg = wind_media_cli::config::Config::default_config();
	assert_eq!(cfg.addon.name, "!!!WindMedia");
	assert!(cfg.addon.wow_path.is_none());
	assert!(cfg.addon.dir.is_none());
	assert!(cfg.resolve_addon_dir().is_none());
	assert_eq!(
		wind_media_cli::config::resolve_max_backups(Some(&cfg)),
		wind_media_cli::config::DEFAULT_MAX_BACKUPS
	);
	assert_eq!(
		wind_media_cli::config::resolve_max_backups(None),
		wind_media_cli::config::DEFAULT_MAX_BACKUPS
	);

	let mut cfg2 = wind_media_cli::config::Config::default_config();
	cfg2.addon.wow_path = Some(std::path::PathBuf::from("/wow"));
	let resolved = cfg2.resolve_addon_dir().unwrap();
	assert_eq!(
		resolved,
		std::path::PathBuf::from("/wow")
			.join("Interface")
			.join("AddOns")
			.join("!!!WindMedia")
	);

	let mut cfg3 = wind_media_cli::config::Config::default_config();
	cfg3.addon.dir = Some(std::path::PathBuf::from("/direct/path"));
	let resolved3 = cfg3.resolve_addon_dir().unwrap();
	assert_eq!(resolved3, std::path::PathBuf::from("/direct/path"));
}

#[test]
fn e2e_output_helpers_no_panic() {
	wind_media_cli::output::init_color(true);
	wind_media_cli::output::print_success("test success");
	wind_media_cli::output::print_warning("test warning");
	wind_media_cli::output::print_error("test error");
	wind_media_cli::output::print_info("label", "value");
}
