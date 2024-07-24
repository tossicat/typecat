use fontdb::{self, Source};

#[derive(Debug)]
pub struct Font {
    name: String,
    is_bold: bool,
    is_italic: bool,
    source: Source,
}

pub fn loading_font_lists_into_db_in_assets_folder(files_loc:&str) -> Vec<Font> {
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
