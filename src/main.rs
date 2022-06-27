use std::fs::File;
use std::io::Read;
use std::path::Path;

use difflib::get_close_matches as gcm;
use fltk::enums::Color;
use fltk::{prelude::*, *};
use serde_json::{self, Map, Value};

fn gui() {
    let a = app::App::default();
    let buf = text::TextBuffer::default();
    let mut win = window::Window::default()
        .with_size(500, 500)
        .with_label("Ron's Dictionary Search");
    let mut flex = group::Flex::default()
        .with_size(400, 450)
        .column()
        .center_of_parent();
    let mut search_term = frame::Frame::default().with_label("Enter search term:");
    flex.set_size(&mut search_term, 20);
    let mut search_input = input::MultilineInput::default();
    flex.set_size(&mut search_input, 30);
    let mut submit_button = button::Button::default().with_label("Submit");
    flex.set_size(&mut submit_button, 30);
    let mut results = frame::Frame::default().with_label("Three closest matches:");
    flex.set_size(&mut results, 30);
    let mut txt = text::TextEditor::default().with_size(280, 300);
    txt.set_buffer(buf);
    txt.wrap_mode(text::WrapMode::AtBounds, 0);
    txt.set_text_color(Color::DarkBlue);
    flex.end();
    win.end();
    win.show();

    let map = read_from_file("src/jsonformatter.txt");

    submit_button.set_callback(move |_submit_button| {
        let all_word_keys: Vec<&str> = map.keys().map(|s| s.as_ref()).collect();
        let results = gcm(
            search_input.value().trim().as_ref(),
            all_word_keys.clone(),
            3,
            0.6,
        );
        let mut my_buf = txt.buffer().unwrap();
        my_buf.set_text("");
        if results.is_empty() {
            my_buf.set_text("Search term not found in dictionary")
        } else {
            for key in results.clone() {
                my_buf.append(format!("Term: {}", key).as_ref());
                my_buf.append(map.get(key).unwrap().to_string().as_ref());
                my_buf.append("\n");
            }
        }
    });

    a.run().unwrap();
}

fn read_from_file<P: AsRef<Path>>(path: P) -> Map<String, Value> {
    let mut file = File::open(path).expect("failed to open file");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("failed to read to string");
    let map: Map<String, Value> =
        serde_json::from_str(&contents).expect("failed to parse into Value");
    map
}

fn main() {
    gui();
    println!("Goodbye!")
}
