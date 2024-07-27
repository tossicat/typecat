use std::path::PathBuf;

use fontdb::{self, Source};

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

pub fn search_for_font(font_list: &Vec<Font>, font_name: &str, font_style: FontStyle) {
    let mut path = PathBuf::new();
    for item in font_list {
        if item.name == font_name {
            match font_style {
                FontStyle::Bold => {
                    if item.is_bold == true {
                        match &item.source {
                            Source::Binary(_) => todo!(),
                            Source::File(e) => path = e.to_path_buf(),
                            Source::SharedFile(_, _) => todo!(),
                        }
                    }
                }
                FontStyle::Italic => {
                    if item.is_italic == true {
                        match &item.source {
                            Source::Binary(_) => todo!(),
                            Source::File(e) => path = e.to_path_buf(),
                            Source::SharedFile(_, _) => todo!(),
                        }
                    }
                }
                FontStyle::Normal => match &item.source {
                    Source::Binary(_) => todo!(),
                    Source::File(e) => path = e.to_path_buf(),
                    Source::SharedFile(_, _) => todo!(),
                },
            }
        } else {
            println!("Path: {:?}", path);
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
