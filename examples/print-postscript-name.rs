#[cfg(not(feature = "fs"))]
fn main() {}

#[cfg(feature = "fs")]
fn main() {
    let path = std::env::args().nth(1).expect("usage: print-postscript-name <path-to-font>");

    let mut db = fontdb::Database::new();
    let ids = db.load_font_file(&path).map(|_| ()).unwrap_or_else(|e| {
        panic!("failed to load {}: {:?}", path, e);
    });
    let _ = ids;

    for face in db.faces() {
        let families: Vec<_> = face.families.iter().map(|(n, _)| n.as_str()).collect();
        println!(
            "index={} post_script_name={:?} families={:?} style={:?} weight={} stretch={:?}",
            face.index,
            face.post_script_name,
            families,
            face.style,
            face.weight.0,
            face.stretch,
        );
    }
}
