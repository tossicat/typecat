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
    let left_margin = 12.0;
    let right_margin = 12.0;

    for data in parsed_data {
        let content = merge_sentence(data.1);
        let font_size = set_font_size(data.0);
        let line_length = count_letter_per_line(x_val,left_margin, right_margin, font_size);
        let lines = make_content_chunk(content, line_length);
        for line in lines {
            println!("{:?}", line);
            // 행간
            y_val -= font_size/4.0;

            current_layer = convert_text(current_layer, line, font_size, left_margin, y_val, &font);
            // 행간
            y_val -= font_size/4.0;
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
    current_layer.set_line_height(font_size+2.0);
    current_layer.use_text(line, font_size, Mm(left_margin), Mm(y_val), &font);
    current_layer.add_line_break();
    return current_layer;
    }

fn count_letter_per_line(x_val: f32, left_margin: f32, right_margin: f32, font_size: f32) -> usize {
    // 글자 1포인트 
    let aval_line = x_val - (left_margin + right_margin);
    let one_size = font_size/3.3;
    return (aval_line/one_size) as usize
}

fn make_content_chunk(content: String, line_length: usize) -> Vec<String> {
    let line_count = content.chars().count()/line_length;
    if line_count > 0 {
        return _split_content(line_count,line_length, content);
    }
    else {
        return vec![content];
    }
}

fn _split_content(line_count: usize, line_length:usize, content: String) -> Vec<String> {
    let mut lines = vec![];
    let mut idx = 0;
    let char_vec: Vec<char> = content.chars().collect();
    for _i in 0..line_count+1 {
        if content.chars().count() >= idx+line_length {
            let chunk = &char_vec[idx..idx+line_length];
            let result: String = chunk.iter().collect();
            println!("{:?}", result.len());
            lines.push(result);
            idx = idx + line_length;
        }
        else {
            let chunk = &char_vec[idx..];
            lines.push(chunk.iter().collect());
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
