use crate::{Note, NoteTitle};
use std::fs;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;
use std::path::Path;

pub const NOTES_DIR: &str = "notes/";

pub fn create_note(note: &Note) {
    let path = format!("{}{}", NOTES_DIR, note.title.as_str());
    let mut data_file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(path.clone())
        .expect("cannot open file");

    // Write to a file
    let _ = data_file
        .write(note.description.as_str().as_bytes())
        .expect("write failed");
}

pub fn update_note(note: &Note) {
    let path = format!("{}{}", NOTES_DIR, note.title.as_str());
    let mut data_file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(path.clone())
        .expect("cannot open file");

    let _ = data_file
        .write(format!("{}\n", note.description).as_bytes())
        .expect("write failed");
}

pub fn read_note(note_title: &NoteTitle) -> String {
    let path = format!("{}{}", NOTES_DIR, note_title.title.as_str());
    let mut data_file = OpenOptions::new()
        .read(true)
        .open(path.clone())
        .expect("cannot open file");

    let mut data = String::new();
    data_file.read_to_string(&mut data).expect("read failed");
    format!("\n{}", data)
}

pub fn delete_note(note_title: &NoteTitle) {
    let path = format!("{}{}", NOTES_DIR, note_title.title.as_str());
    fs::remove_file(path).expect("can not remove file");
}

pub fn already_exists(title: &String) -> bool {
    let path = format!("{}{}", NOTES_DIR, title);
    Path::new(path.as_str()).exists()
}
