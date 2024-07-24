const DEMO_TTF: &[u8] = include_bytes!("./fonts/NanumGothic.ttf");
const DEFAULT_FONT_FOLDER: &str = "../assets/fonts";

use std::sync::Arc;

use file_manager::read_font;

use fontdb;

#[test]
fn temp_test() {
    let temp = read_font(DEFAULT_FONT_FOLDER);
    println!("results:`{:?}`", temp);
}

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
fn _loading_font_lists_into_db_in_folder_asset() {
    pub const DEFAULT_FONT_FOLDER: &str = "../assets/fonts";
    let mut font_db = fontdb::Database::new();
    font_db.load_fonts_dir(DEFAULT_FONT_FOLDER);
    let now = std::time::Instant::now();

    println!(
        "Loaded {} font faces in {}ms.",
        font_db.len(),
        now.elapsed().as_millis()
    );

    // println!("reading {:?}", font_db.query(temp_query));
    println!("reading {:?}", font_db);
    for item in font_db.faces() {
        // let temp = if item.source {
        //     fontdb::Source::File(n) => n,
        //     _ => ,
        // };
        println!(
            "{:?}, {:?}, {:?}, {:#?}",
            item.families, item.style, item.weight, item.source
        );
    }
    // for item in font_db.faces() {
    //     println!("{:?}", item.families);
    //     for sub_item in &item.families {
    //         println!("{:?}", sub_item);
    //     }
    // }
}
