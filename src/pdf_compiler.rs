// use pdf_writer::types::{ActionType, AnnotationType, BorderType};
use pdf_writer::{Content, Finish, Name, Pdf, Rect, Ref, Str};
use crate::models::FragmentType;
use crate::markdown_parser::Rule;

pub fn compile(parsed_data: Vec<(Rule, Vec<FragmentType>)>) -> std::io::Result<()> {

    let mut pdf = Pdf::new();

    // A4기준
    let mut x_val = 108.0;
    let mut y_val = 764.0;

    // 기본 값 정의
    let catalog_id = Ref::new(1);
    let page_tree_id = Ref::new(2);
    let page_id = Ref::new(3);
    let font_id = Ref::new(4);
    let content_id = Ref::new(5);
    let font_name = Name(b"F1");

    // 페이지 트리 정의
    pdf.catalog(catalog_id).pages(page_tree_id);
    pdf.pages(page_tree_id).kids([page_id]).count(1);

    // 페이지 작성 시작
    let mut page = pdf.page(page_id);

    // 페이지 A4 기준으로 작성 시작
    page.media_box(Rect::new(0.0, 0.0, 595.0, 842.0));
    page.parent(page_tree_id);
    page.contents(content_id);

    let mut content = Content::new();

    for data in parsed_data {
        
        let merged_text = merge_sentence(data.1);
        println!("{:?}", merged_text);
        let sentence = Str(merged_text.as_bytes());

        match data.0 {
            Rule::H1 => {
                content = set_header_content(content, sentence, font_name, 40.0, x_val, y_val);
                println!("{:?}", y_val);
                y_val -= 40.0;
            },
            Rule::H2 => {
                content = set_header_content(content, sentence, font_name, 30.0, x_val, y_val);
                println!("{:?}", y_val);
                y_val -= 30.0;
            },
            Rule::H3 => {
                content = set_header_content(content, sentence, font_name, 25.0, x_val, y_val);
                println!("{:?}", y_val);
                y_val -= 25.0;
            },
            Rule::H4 => {
                content = set_header_content(content, sentence, font_name, 20.0, x_val, y_val);
                println!("{:?}", y_val);
                y_val -= 20.0;
            },
            Rule::H5 => {
                content = set_header_content(content, sentence, font_name, 15.0, x_val, y_val);
                println!("{:?}", y_val);
                y_val -= 15.0;
            },
            Rule::H6 => {
                content = set_header_content(content, sentence, font_name, 10.0, x_val, y_val);
                println!("{:?}", y_val);
                y_val -= 10.0;
            },
            Rule::LINE => {
                content = set_header_content(content, sentence, font_name, 5.0, x_val, y_val);
                println!("{:?}", y_val);
                y_val -= 5.0;
            },
            _ => {}
        }
    }
    page.resources().fonts().pair(font_name, font_id);
    page.finish();
    pdf.type1_font(font_id).base_font(Name(b"Helvetica"));
    pdf.stream(content_id, &content.finish());
    let buf: Vec<u8> = pdf.finish();

    std::fs::write("target/test.pdf", buf)

}

fn set_header_content(mut content: Content, text: Str, font_name: Name<'_>, size: f32, x: f32, y: f32) -> Content {
    
    content.begin_text();
    content.set_font(font_name, size);
    content.next_line(x, y);
    content.show(text);
    content.end_text();

    return content;

}

fn merge_sentence(list: Vec<FragmentType>) -> String {
    let mut sentence = String::new();

    for content in list {
        match content {
            FragmentType::WORD(x) => {
                sentence.push_str(&x.text);
            },
            FragmentType::LINK(x) => {
                sentence.push_str("");
            },
            FragmentType::Newline => {
                sentence.push_str("");
            }
        }
    }
    return sentence;
}
