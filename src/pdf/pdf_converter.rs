extern crate printpdf;
use crate::markdown::md_parser::Rule;
use crate::markdown::datatypes::FragmentType;
use printpdf::*;
use std::fs::File;
use std::env::current_dir;
use std::env;
use std::io::BufWriter;
use crate::pdf::functions::*;

pub fn convert(parsed_data: Vec<(Rule, Vec<FragmentType>)>) {
    // A4기준
    let x_val = 210.0;
    let mut y_val = 297.0;

    let (doc, page1, layer1) = PdfDocument::new("test", Mm(x_val), Mm(y_val), "Layer 1");
    let mut current_layer = doc.get_page(page1).get_layer(layer1);

    // TO DO: 한국어 폰트 테스트, 폰트 변경 로직 작업 필요
    let cwd_path = current_dir().unwrap().into_os_string().into_string().unwrap();
    env::set_var("CWD", cwd_path);
    let mut font_reader =
        std::io::Cursor::new(include_bytes!(concat!("../../assets/fonts/NanumGothic.ttf")).as_ref());
    let font = doc.add_external_font(&mut font_reader).unwrap();

    current_layer.begin_text_section();

    // top_margin
    y_val -= 20.0;
    let left_margin = 12.7;
    let right_margin = 12.7;

    for data in parsed_data {
        let content = merge_sentence(data.1);
        let font_size = set_font_size(data.0);
        let line_length = count_letter_per_line(x_val,left_margin, right_margin, font_size);
        let lines = make_content_chunk(content, line_length);
        for line in lines {
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
