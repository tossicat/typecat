const DEFAULT_FONT_FOLDER: &str = "../assets/fonts";

use file_manager::read_font;

use fontdb;

#[test]
fn temp_test() {
    let temp = read_font(DEFAULT_FONT_FOLDER);
    println!("results:`{:?}`", temp);
}

#[test]
/// font_db를 이용한 폰트 검색 
fn _font_db_test() {
    let mut font_db = fontdb::Database::new();
    font_db.load_fonts_dir(DEFAULT_FONT_FOLDER);

    const FAMILY_NAME: &str = "나눔고딕";
    let query = fontdb::Query {
        families: &[fontdb::Family::Name(FAMILY_NAME), fontdb::Family::SansSerif],
        weight: fontdb::Weight::BOLD,
        ..fontdb::Query::default()
    };

    let now = std::time::Instant::now();
    match font_db.query(&query) {
        Some(id) => {
            let (src, index) = font_db.face_source(id).unwrap();
            if let fontdb::Source::File(ref path) = &src {
                println!(
                    "Font '{}':{} found in {}ms.",
                    path.display(),
                    index,
                    now.elapsed().as_micros() as f64 / 1000.0
                );
            }
        }
        None => {
            println!("Error: '{}' not found.", FAMILY_NAME);
        }
    }
}
#[test]
fn _loading_font_lists_into_db_in_folder_asset() {
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
