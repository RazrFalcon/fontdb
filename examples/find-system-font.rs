//use std::fmt::{Error, Result};
//use std::fmt::Result;
use fontdb::{Database, Family, ID, Query, Source, Weight};

fn query_family_name(db: &Database, family_name: &str) -> Result<ID, String> {
    let query = Query {
        families: &[Family::Name(&family_name), Family::Serif],
        weight: Weight::NORMAL,
        ..Query::default()
    };

    let now = std::time::Instant::now();
    match db.query(&query) {
        Some(id) => {
            let (src, index) = db.face_source(id).unwrap();
            if let Source::File(ref path) = &src {
                println!("fontdb: Font '{}':{} found in {}ms.", path.display(), index,
                         now.elapsed().as_micros() as f64 / 1000.0);
            }
            return Ok(id)
        },
        None => {
            let error = format!("fontdb: Font family '{}' not found.", &family_name);
            return Err(error)
        }
    }
}

fn main() {
    std::env::set_var("RUST_LOG", "fontdb=trace");
    env_logger::init();

    let mut db = Database::new();
    let now = std::time::Instant::now();
    db.load_system_fonts();
    db.set_serif_family("Times New Roman");
    db.set_sans_serif_family("Arial");
    db.set_cursive_family("Comic Sans MS");
    db.set_fantasy_family("Impact");
    db.set_monospace_family("Courier New");
    println!("Loaded {} font faces in {}ms.", db.len(), now.elapsed().as_millis());

    const FAMILY_MDL2: &str = "mdl2";
    match query_family_name(&db, &FAMILY_MDL2) {
        Ok(id) => println!("fontdb: Font family '{}': face '{:?}'", FAMILY_MDL2, id),
        Err(e) => println!("{}", e),
    }

    const FAMILY_MDL_ICONS: &str = "Material Icons";
    match query_family_name(&db, &FAMILY_MDL_ICONS) {
        Ok(id) => println!("fontdb: Font family '{}': face '{:?}'", FAMILY_MDL_ICONS, id),
        Err(e) => println!("{}", e),
            }

    const FAMILY_NOTO_SANS: &str = "Noto Sans";
    match query_family_name(&db, &FAMILY_NOTO_SANS) {
        Ok(id) => println!("Font family '{}': face '{:?}'", FAMILY_NOTO_SANS, id),
        Err(e) => println!("{}", e),
        }

    const FAMILY_NOTO_SERIF: &str = "Noto Serif";
    match query_family_name(&db, &FAMILY_NOTO_SERIF) {
        Ok(id) => println!("fontdb: Font family '{}': face '{:?}'", FAMILY_NOTO_SERIF, id),
        Err(e) => println!("{}", e),
        }

    const FAMILY_ROBOTO: &str = "Roboto Mono";
    match query_family_name(&db, &FAMILY_ROBOTO) {
        Ok(id) => println!("fontdb: Font family '{}': face '{:?}'", FAMILY_ROBOTO, id),
        Err(e) => println!("{}", e),
    }

    const FAMILY_TIMES_NEW_ROMAN: &str = "Times New Roman";
    match query_family_name(&db, &FAMILY_TIMES_NEW_ROMAN) {
        Ok(id) => println!("fontdb: Font family '{}': face '{:?}'", FAMILY_TIMES_NEW_ROMAN, id),
        Err(e) => println!("{}", e),
    }

    const FAMILY_TIMES: &str = "Times-Roman";
    match query_family_name(&db, &FAMILY_TIMES) {
        //Ok(id) => println!("Got {} with face id {:?}", FAMILY_TIMES, ID::id),
        Ok(id) => println!("fontdb: Font family '{}': face '{:?}'", FAMILY_TIMES, id),
        Err(e) => println!("{}", e),
    }

    const FAMILY_CJL_SANS: &str = "NotoSansTC-Regular-Alphabetic";
    match query_family_name(&db, &FAMILY_CJL_SANS) {
        Ok(id) => println!("fontdb: Font family '{}': face '{:?}'", FAMILY_CJL_SANS, id),
        Err(e) => println!("{}", e),
    }

    const FAMILY_CJL_SERIF: &str = "NotoSerifSC-Regular-Alphabetic";
        match query_family_name(&db, &FAMILY_CJL_SERIF) {
        Ok(id) => println!("fontdb: Font family '{}': face '{:?}'", FAMILY_CJL_SERIF, id),
        Err(e) => println!("{}", e),
    }

}
