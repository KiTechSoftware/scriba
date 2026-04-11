use scriba::{Format, Output, Ui};

#[test]
fn ui_renders_markdown_output() {
    let ui = Ui::new().with_format(Format::Markdown);

    let output = Output::new()
        .heading(1, "scriba")
        .paragraph("hello");

    let rendered = ui.render(&output).unwrap();

    assert!(rendered.contains("# scriba"));
    assert!(rendered.contains("hello"));
}