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
        std::io::Cursor::new(include_bytes!("../assets/fonts/NanumGothic.ttf").as_ref());
    let font = doc.add_external_font(&mut font_reader).unwrap();

    current_layer.begin_text_section();
    // top_margin
    y_val -= 30.0;
    let left_margin = 5.0;
    let right_margin = 5.0;

    for data in parsed_data {
        let content = merge_sentence(data.1);
        let font_size = set_font_size(data.0);
        let lines = make_content_chunk(content, x_val - left_margin, right_margin, font_size);
        for line in lines {
            current_layer = convert_text(current_layer, line, font_size, left_margin, y_val, &font);
            // 행간
            y_val -= 10.0;
        }
    }
    current_layer.end_text_section();

    doc.save(&mut BufWriter::new(File::create("test.pdf").unwrap()))
        .unwrap();
}

fn convert_text(
    current_layer: PdfLayerReference,
    line: String,
    font_size: f32,
    left_margin: f32,
    y_val: f32,
    font: &IndirectFontRef
) -> PdfLayerReference {
    current_layer.use_text(line, font_size, Mm(left_margin), Mm(y_val), &font);
    current_layer.add_line_break();
    return current_layer;
    }

fn make_content_chunk(content: String, x_val: f32, right_margin: f32, font_size: f32) -> Vec<String> {
    // slice 값 바이트 기준으로 변경, byte index 176 is not a char boundary
    let line_length = 76;
    println!("{:?}", content.len());
    let line_count = content.len()/line_length;
    if line_count > 1 {
        return split_content(line_count,line_length, content);
    }
    else {
        return vec![content];
    }
}

fn split_content(line_count: usize, line_length:usize, content: String) -> Vec<String> {
    let mut lines = vec![];
    let mut idx = 0;
    for _i in 0..line_count+1 {
        if content.len() >= idx+line_length {
            let chunk = &content[idx..idx+line_length];
            lines.push(chunk.to_string());
            idx = idx + line_length;
        }
        else {
            let chunk = &content[idx..];
            lines.push(chunk.to_string());
        }
    }
    return lines
}

fn set_font_size(icon: Rule) -> f32 {
    let mut size = 0.0;
    // font_size
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
