fn main() {
    std::env::set_var("RUST_LOG", "fontdb=trace");
    env_logger::init();

    let mut db = fontdb::Database::new();
    let now = std::time::Instant::now();
    load_system_fonts(&mut db);
    db.set_serif_family("Times New Roman");
    db.set_sans_serif_family("Arial");
    db.set_cursive_family("Comic Sans MS");
    db.set_fantasy_family("Impact");
    db.set_monospace_family("Courier New");
    println!("Loaded {} font faces in {}ms.", db.len(), now.elapsed().as_millis());

    const FAMILY_NAME: &str = "Times New Roman";
    let query = fontdb::Query {
        families: &[fontdb::Family::Name(FAMILY_NAME), fontdb::Family::SansSerif],
        weight: fontdb::Weight::BOLD,
        ..fontdb::Query::default()
    };

    let now = std::time::Instant::now();
    match db.query(&query) {
        Some(id) => {
            let (src, index) = db.face_source(id).unwrap();
            if let fontdb::Source::File(ref path) = &*src {
                println!("Font '{}':{} found in {}ms.", path.display(), index,
                         now.elapsed().as_micros() as f64 / 1000.0);
            }
        }
        None => {
            println!("Error: '{}' not found.", FAMILY_NAME);
        }
    }
}

// The `fontdb` crate doesn't handle system fonts loading, so we have to do it ourselves.
//
// Note that the code below is just an example and doesn't represent
// a complete solution for system fonts loading.

#[cfg(target_os = "windows")]
fn load_system_fonts(db: &mut fontdb::Database) {
    db.load_fonts_dir("C:\\Windows\\Fonts\\");
}

#[cfg(target_os = "macos")]
fn load_system_fonts(db: &mut fontdb::Database) {
    db.load_fonts_dir("/Library/Fonts");
    db.load_fonts_dir("/System/Library/Fonts");

    if let Ok(ref home) = std::env::var("HOME") {
        let path = std::path::Path::new(home).join("Library/Fonts");
        db.load_fonts_dir(path);
    }
}

#[cfg(all(unix, not(target_os = "macos")))]
fn load_system_fonts(db: &mut fontdb::Database) {
    db.load_fonts_dir("/usr/share/fonts/");
    db.load_fonts_dir("/usr/local/share/fonts/");

    if let Ok(ref home) = std::env::var("HOME") {
        let path = std::path::Path::new(home).join(".local/share/fonts");
        db.load_fonts_dir(path);
    }
}
