use fontdb;

const _DEFAULT_FONT_FOLDER: &str = "../assets/fonts";

#[derive(Debug)]
pub struct Font {
    name: String,
    is_bold_exist: bool,
    is_italic_exist: bool,
}

// fn is_italic_exist() = {
//     match item.style {
//         fontdb::Style::Normal => todo!(),
//         fontdb::Style::Italic => todo!(),
//         fontdb::Style::Oblique => todo!(),
//     }
// };

pub fn loading_font_lists_into_db_in_assets_folder() -> Vec<Font> {
    let mut font_db = fontdb::Database::new();
    font_db.load_fonts_dir(_DEFAULT_FONT_FOLDER);

    let mut font_list: Vec<Font> = vec![];

    for item in font_db.faces() {
        // println!("{:?}", item.families);
        for sub_item in &item.families {
            // println!("{:?}", sub_item);
            let temp_font = Font {
                name: sub_item.0.clone(),
                is_bold_exist: true,
                is_italic_exist: true,
            };
            font_list.push(temp_font);
        }
    }
    font_list
}
