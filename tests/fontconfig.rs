#![cfg(all(
    unix,
    feature = "fontconfig",
    not(any(target_os = "macos", target_os = "ios", target_os = "android"))
))]

// A generic family resolves to the first installed family that fontconfig prefers for it,
// even when a later alias prefers another family.
#[test]
fn generic_family_is_first_installed_preference() {
    let fonts_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fonts");
    let config = format!(
        r#"<?xml version="1.0"?>
<fontconfig>
  <dir>{}</dir>
  <alias>
    <family>serif</family>
    <prefer><family>Missing Serif</family><family>Tuffy</family></prefer>
  </alias>
  <alias>
    <family>serif</family>
    <prefer><family>Missing Symbols</family></prefer>
  </alias>
  <alias>
    <family>monospace</family>
    <prefer><family>Missing Mono</family></prefer>
  </alias>
</fontconfig>
"#,
        fonts_dir.display()
    );
    let config_path = std::env::temp_dir().join(format!("fontdb-test-{}.conf", std::process::id()));
    std::fs::write(&config_path, config).unwrap();
    std::env::set_var("FONTCONFIG_FILE", &config_path);

    let mut db = fontdb::Database::new();
    db.load_system_fonts();
    std::fs::remove_file(&config_path).unwrap();

    assert_eq!(db.family_name(&fontdb::Family::Serif), "Tuffy");
    // Without an installed preference, the family is the one fontconfig would try first.
    assert_eq!(db.family_name(&fontdb::Family::Monospace), "Missing Mono");
}
