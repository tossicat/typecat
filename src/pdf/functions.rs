extern crate printpdf;
use crate::markdown::parser::Rule;
use crate::markdown::datatypes::FragmentType;
use printpdf::*;
 
pub fn convert_text(
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

pub fn count_letter_per_line(x_val: f32, left_margin: f32, right_margin: f32, font_size: f32) -> usize {
    // 글자 1포인트 
    let aval_line = x_val - (left_margin + right_margin);
    let one_size = font_size/3.5;
    return (aval_line/one_size) as usize
}

pub fn make_content_chunk(content: String, line_length: usize) -> Vec<String> {
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

pub fn set_font_size(icon: Rule) -> f32 {
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

pub fn merge_sentence(list: Vec<FragmentType>) -> String {
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
