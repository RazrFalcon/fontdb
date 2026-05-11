const DEMO_TTF: &[u8] = include_bytes!("./fonts/Tuffy.ttf");
use std::sync::Arc;

#[test]
fn query_family_name_case() {
    env_logger::init();
    let mut font_db = fontdb::Database::new();
    let ids = font_db.load_font_source(fontdb::Source::Binary(Arc::new(DEMO_TTF)));

    assert_eq!(ids.len(), 1);
    let id = ids[0];

    let name_variations = vec!["Tuffy", "tuffy", "TUFFY"];
    for name in name_variations.iter() {
        let query = fontdb::Query {
            families: &[fontdb::Family::Name(name)],
            ..fontdb::Query::default()
        };
        let retrieved_id = font_db.query(&query);
        assert!(!retrieved_id.is_none() && retrieved_id.unwrap() == id);
    }
}
