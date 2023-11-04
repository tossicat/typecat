use pdf_writer::types::{ActionType, AnnotationType, BorderType};
use pdf_writer::{Content, Finish, Name, Pdf, Rect, Ref, Str, TextStr};

pub fn compile() -> std::io::Result<()> {

    let mut pdf = Pdf::new();

    // Define some indirect reference ids we'll use.
    let catalog_id = Ref::new(1);
    let page_tree_id = Ref::new(2);
    let page_id = Ref::new(3);
    let font_id = Ref::new(4);
    let content_id = Ref::new(5);
    let font_name = Name(b"F1");

    // Write the document catalog with a reference to the page tree.
    pdf.catalog(catalog_id).pages(page_tree_id);
    // Write the page tree with a single child page.
    pdf.pages(page_tree_id).kids([page_id]).count(1);

    // Write a page.
    let mut page = pdf.page(page_id);
    page.media_box(Rect::new(0.0, 0.0, 595.0, 842.0));
    page.parent(page_tree_id);
    page.contents(content_id);

    // We also create the annotations list here that allows us to have things
    // like links or comments on the page.
    let mut annotations = page.annotations();
    let mut annotation = annotations.push();
    // Write an action for the annotation, telling it where to link to. Actions
    // can be associated with annotations, outline objects, and more and allow
    // creating interactive PDFs (open links, play sounds...).
    annotation
        .action()
        .action_type(ActionType::Uri)
        .uri(Str(b"https://www.rust-lang.org/"));

}
