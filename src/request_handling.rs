use crate::constants::*;
use crate::file_handling::*;
use rouille::*;
use serde::Deserialize;

use crate::error_codes::*;
// mod constants;
// mod error_code;
// mod file_handling;

#[derive(Deserialize)]
pub struct Note {
    pub title: String,
    pub description: String,
}

#[derive(Deserialize)]
pub struct NoteTitle {
    pub title: String,
}

pub fn handle_note_creation(request: &Request) -> Response {
    let note: Note = try_or_400!(rouille::input::json_input(request));
    if already_exists(&note.title) {
        respond(BAD_REQUEST, "Note already present", Some(ALREADY_EXISTS))
    } else {
        create_note(&note);
        respond(SUCCESS, "Note created", None)
    }
}

pub fn handle_note_update(request: &Request) -> Response {
    let note: Note = try_or_400!(rouille::input::json_input(request));
    if !already_exists(&note.title) {
        respond(
            BAD_REQUEST,
            "The note you are trying to update does not exists",
            Some(NOTE_NOT_PRESENT),
        )
    } else {
        update_note(&note);
        respond(SUCCESS, "Note updated", None)
    }
}

pub fn handle_note_read(request: &Request) -> Response {
    let note_title: NoteTitle = try_or_400!(rouille::input::json_input(request));
    if !already_exists(&note_title.title) {
        respond(
            BAD_REQUEST,
            "The note you are trying to read does not exists",
            Some(NOTE_NOT_PRESENT),
        )
    } else {
        let note_description = read_note(&note_title);
        respond(SUCCESS, note_description.as_str(), None)
    }
}

pub fn handle_note_delete(request: &Request) -> Response {
    let note_title: NoteTitle = try_or_400!(rouille::input::json_input(request));
    if !already_exists(&note_title.title) {
        respond(
            BAD_REQUEST,
            "The note you are trying to delete does not exists",
            Some(NOTE_NOT_PRESENT),
        )
    } else {
        delete_note(&note_title);
        respond(SUCCESS, "Note deleted", None)
    }
}

pub fn handle_invalid_method() -> Response {
    respond(BAD_REQUEST, "Invalid method", None)
}

pub fn handle_invalid_url() -> Response {
    respond(BAD_REQUEST, "Invalid url", Some(INVALID_URL))
}

fn respond(response_code: u16, msg: &str, error_code: Option<u8>) -> Response {
    match response_code {
        BAD_REQUEST => {
            let msg = format!(
                "{} - msg: {}, error code: {}\n",
                FAILURE_MSG,
                msg,
                error_code.unwrap()
            );
            Response::text(msg).with_status_code(BAD_REQUEST)
        }
        _ => {
            let msg = format!("{} - msg: {}\n", SUCCESS_MSG, msg);
            Response::text(msg)
        }
    }
}
