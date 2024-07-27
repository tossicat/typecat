use std::path::PathBuf;

use fontdb::{self, Database, Source};

#[derive(Debug)]
pub struct Font {
    name: String,
    is_bold: bool,
    is_italic: bool,
    source: Source,
}

pub enum FontStyle {
    Bold,
    Italic,
    Normal,
}

fn search_for_font_style(font_style: FontStyle) -> (u16, bool) {
    match font_style {
        FontStyle::Bold => (700, false),
        FontStyle::Italic => (700, true),
        FontStyle::Normal => (400, false),
    }
}

pub fn search_for_font_in_db(font_db: Database, font_name: &str, font_style: FontStyle) {
    let (temp_weight, is_italic) = search_for_font_style(font_style);
    if is_italic == true {
        let query = fontdb::Query {
            families: &[fontdb::Family::Name(font_name), fontdb::Family::SansSerif],
            weight: fontdb::Weight(temp_weight),
            style: fontdb::Style::Italic,
            ..fontdb::Query::default()
        };

        match font_db.query(&query) {
            Some(id) => {
                let (src, index) = font_db.face_source(id).unwrap();
                if let fontdb::Source::File(ref path) = &src {
                    println!("Font '{}':{} found", path.display(), index,);
                }
            }
            None => {
                println!("Error: '{}' not found.", font_name);
            }
        }
    } else {
        let query = fontdb::Query {
            families: &[fontdb::Family::Name(font_name), fontdb::Family::SansSerif],
            weight: fontdb::Weight(temp_weight),
            style: fontdb::Style::Normal,
            ..fontdb::Query::default()
        };

        match font_db.query(&query) {
            Some(id) => {
                let (src, index) = font_db.face_source(id).unwrap();
                if let fontdb::Source::File(ref path) = &src {
                    println!("Font '{}':{} found", path.display(), index,);
                }
            }
            None => {
                println!("Error: '{}' not found.", font_name);
            }
        }
    }
}

pub fn loading_font_lists_into_db_in_assets_folder(files_loc: &str) -> Vec<Font> {
    let mut font_db = fontdb::Database::new();
    font_db.load_fonts_dir(files_loc);

    let mut font_list: Vec<Font> = vec![];

    for item in font_db.faces() {
        // println!("{:?}", item.families);
        for sub_item in &item.families {
            // println!("{:?}", sub_item);
            let temp_font = Font {
                name: sub_item.0.clone(),
                source: item.source.clone(),
                is_bold: {
                    match item.weight {
                        fontdb::Weight(700) => true,
                        fontdb::Weight(400) => false,
                        _ => false,
                    }
                },
                is_italic: {
                    match item.style {
                        fontdb::Style::Normal => false,
                        fontdb::Style::Italic => true,
                        fontdb::Style::Oblique => false,
                    }
                },
            };
            font_list.push(temp_font);
        }
    }
    font_list
}
