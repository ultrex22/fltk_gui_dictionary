use crate::*;
use fltk::enums::Color;
use fltk::{prelude::*, *};

pub fn gui() {
    let a = app::App::default();
    let buf = text::TextBuffer::default();
    let mut win = window::Window::default()
        .with_size(500, 400)
        .with_label("Ron's Dictionary Search");
    let mut flex = group::Flex::default()
        .with_size(400, 350)
        .column()
        .center_of_parent();
    let mut search_term = frame::Frame::default().with_label("Enter search term:");
    flex.set_size(&mut search_term, 20);
    let mut search_input = input::IntInput::default();
    flex.set_size(&mut search_input, 30);
    let mut submit_button = button::Button::default().with_label("Submit");
    flex.set_size(&mut submit_button, 30);
    let mut results = frame::Frame::default().with_label("Three closest matches:");
    flex.set_size(&mut results, 30);
    let mut txt = text::TextEditor::default().with_size(280, 300);
    txt.set_buffer(buf);
    txt.wrap_mode(text::WrapMode::AtBounds, 0);
    txt.set_text_color(Color::Red);
    flex.end();
    win.end();
    win.show();

    let mut my_buf = txt.buffer().unwrap();

    my_buf.set_text("Lots of text...");

    submit_button.set_callback(move |btn| {
        println!("your age is {}", search_input.value());
    });

    a.run().unwrap();
}
