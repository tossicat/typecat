extern crate printpdf;
use crate::markdown_parser::Rule;
use crate::models::FragmentType;
use printpdf::*;
use std::fs::File;
use std::io::BufWriter;

pub fn compile(parsed_data: Vec<(Rule, Vec<FragmentType>)>) {
    // A4기준
    let x_val = 210.0;
    let mut y_val = 297.0;

    let (doc, page1, layer1) = PdfDocument::new("test", Mm(x_val), Mm(y_val), "Layer 1");
    let mut current_layer = doc.get_page(page1).get_layer(layer1);

    // 한국어 폰트 테스트
    let mut font_reader =
        std::io::Cursor::new(include_bytes!("../assets/fonts/NotoSerifCJKkr-Regular.otf").as_ref());
    let font = doc.add_external_font(&mut font_reader).unwrap();

    current_layer.begin_text_section();
    y_val -= 30.0;

    for data in parsed_data {
        current_layer = convert_text(current_layer, y_val, &font, data);
        y_val -= 20.0;
    }
    current_layer.end_text_section();

    doc.save(&mut BufWriter::new(File::create("test.pdf").unwrap()))
        .unwrap();
}

fn convert_text(
    current_layer: PdfLayerReference,
    y_val: f32,
    font: &IndirectFontRef,
    data: (Rule, Vec<FragmentType>),
) -> PdfLayerReference {
    let content = merge_sentence(data.1);
    let size = set_font_size(data.0);

    current_layer.use_text(content, size, Mm(5.0), Mm(y_val), &font);

    return current_layer;
}

fn set_font_size(icon: Rule) -> f32 {
    let mut size = 0.0;
    match icon {
        Rule::H1 => size = 30.0,
        Rule::H2 => size = 28.0,
        Rule::H3 => size = 25.0,
        Rule::H4 => size = 22.0,
        Rule::H5 => size = 20.0,
        Rule::H6 => size = 18.0,
        Rule::LINE => size = 10.0,
        _ => {}
    }
    return size;
}

fn merge_sentence(list: Vec<FragmentType>) -> String {
    let mut sentence = String::new();

    for content in list {
        match content {
            FragmentType::WORD(x) => {
                sentence.push_str(&x.text);
            }
            FragmentType::LINK(x) => {
                sentence.push_str("");
            }
            FragmentType::Newline => {
                sentence.push_str("");
            }
        }
    }
    return sentence;
}
