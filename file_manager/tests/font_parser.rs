const DEMO_TTF: &[u8] = include_bytes!("./fonts/NanumGothic.ttf");

use fontdb;
use std::sync::Arc;

#[test]
fn add_font_and_get_ids_back() {
    let mut font_db = fontdb::Database::new();
    let ids = font_db.load_font_source(fontdb::Source::Binary(Arc::new(DEMO_TTF)));

    assert_eq!(ids.len(), 1);
    let id = ids[0];

    let font = font_db.face(id).unwrap();
    println!("reading `{:?}`", ids);
    assert!(font.families.iter().any(|(name, _)| name == "NanumGothic"));
    println!("reading `{:?}`", font.style);
}

#[test]
fn add_fonts_db() {
    pub const DEFAULT_FONT_FOLDER: &str = "../assets/fonts";
    let mut font_db = fontdb::Database::new();
    font_db.load_fonts_dir(DEFAULT_FONT_FOLDER);
    let now = std::time::Instant::now();

    println!(
        "Loaded {} font faces in {}ms.",
        font_db.len(),
        now.elapsed().as_millis()
    );
    println!("reading {:?}", font_db);
}