use constants::*;
use request_handling::*;
use rouille::*;
// use std::thread::Builder;
// use std::collections::HashMap;
// use crossbeam_channel::*;

mod constants;
mod error_codes;
mod file_handling;
mod request_handling;

fn main() {
    let server = Server::new("localhost:8080", |request| {
        let method = request.method();
        println!("method {}", method);
        return match method {
            "GET" => match request.url().as_str() {
                READ => handle_note_read(request),
                _ => handle_invalid_url(),
            },
            "POST" => match request.url().as_str() {
                CREATE => handle_note_creation(request),
                UPDATE => handle_note_update(request),
                _ => handle_invalid_url(),
            },
            "DELETE" => match request.url().as_str() {
                DELETE => handle_note_delete(request),
                _ => handle_invalid_url(),
            },
            _ => handle_invalid_method(),
        };
    })
    .unwrap();
    println!("Listening on {:?}", server.server_addr());
    server.run();
}

#[cfg(test)]
mod tests {
    use crate::file_handling::NOTES_DIR;
    use crossbeam_channel::unbounded;
    use std::collections::HashMap;
    use std::fs;
    use std::fs::OpenOptions;
    use std::io::Read;
    use std::thread::Builder;
    use std::time::Duration;

    use self::file_handling::{already_exists, create_note};

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    fn init_server() -> Server<impl Fn(&Request) -> Response> {
        Server::new("localhost:8080", |request| {
            let method = request.method();
            return match method {
                "GET" => match request.url().as_str() {
                    READ => handle_note_read(request),
                    _ => handle_invalid_url(),
                },
                "POST" => match request.url().as_str() {
                    CREATE => handle_note_creation(request),
                    UPDATE => handle_note_update(request),
                    _ => handle_invalid_url(),
                },
                "DELETE" => match request.url().as_str() {
                    DELETE => handle_note_delete(request),
                    _ => handle_invalid_url(),
                },
                _ => handle_invalid_method(),
            };
        })
        .unwrap()
    }

    #[test]
    fn test_note_creation() {
        let path = format!("{}{}", NOTES_DIR, "note");
        if already_exists(&String::from("note")) {
            fs::remove_file(path.clone()).expect("can not remove file");
        }

        let (sender, receiver) = unbounded();

        let server_handle = Builder::new()
            .spawn(move || {
                let server = init_server();
                loop {
                    server.poll_timeout(Duration::from_millis(100));
                    if let Ok(_) = receiver.recv_timeout(Duration::from_millis(100)) {
                        break;
                    }
                }
            })
            .unwrap();

        let client_handle = Builder::new()
            .spawn(move || {
                let client = reqwest::blocking::Client::new();
                let mut map = HashMap::new();
                map.insert("title", "note");
                map.insert("description", "xyz");

                let res = client
                    .post("http://127.0.0.1:8080/create")
                    .json(&map)
                    .send()
                    .unwrap();
                let output = res.text().unwrap();
                assert!(output.contains("SUCCESS"));
                assert!(already_exists(&String::from("note")));
                let mut data_file = OpenOptions::new()
                    .read(true)
                    .open(path.clone())
                    .expect("cannot open file");

                let mut data = String::new();
                data_file.read_to_string(&mut data).expect("read failed");

                assert!(data.contains("xyz"));

                let _ = sender.send(true);
            })
            .unwrap();

        let _ = server_handle.join();
        let _ = client_handle.join();
    }

    #[test]
    fn test_note_update() {
        let path = format!("{}{}", NOTES_DIR, "note");
        if already_exists(&String::from("note")) {
            fs::remove_file(path.clone()).expect("can not remove file");
        }

        let note = Note {
            title: String::from("note"),
            description: String::from("xyz"),
        };
        create_note(&note);

        let (sender, receiver) = unbounded();

        let server_handle = Builder::new()
            .spawn(move || {
                let server = init_server();
                loop {
                    server.poll_timeout(Duration::from_millis(100));
                    if let Ok(_) = receiver.recv_timeout(Duration::from_millis(100)) {
                        break;
                    }
                }
            })
            .unwrap();

        let client_handle = Builder::new()
            .spawn(move || {
                let client = reqwest::blocking::Client::new();
                let mut map = HashMap::new();
                map.insert("title", "note");
                map.insert("description", "some_updates");

                let res = client
                    .post("http://127.0.0.1:8080/update")
                    .json(&map)
                    .send()
                    .unwrap();
                let output = res.text().unwrap();
                assert!(output.contains("SUCCESS"));
                assert!(already_exists(&String::from("note")));
                let mut data_file = OpenOptions::new()
                    .read(true)
                    .open(path.clone())
                    .expect("cannot open file");

                let mut data = String::new();
                data_file.read_to_string(&mut data).expect("read failed");

                assert!(data.contains("some_updates"));

                let _ = sender.send(true);
            })
            .unwrap();

        let _ = server_handle.join();
        let _ = client_handle.join();
    }

    #[test]
    fn test_note_read() {
        let path = format!("{}{}", NOTES_DIR, "note");
        if already_exists(&String::from("note")) {
            fs::remove_file(path.clone()).expect("can not remove file");
        }

        let note = Note {
            title: String::from("note"),
            description: String::from("xyz"),
        };
        create_note(&note);

        let (sender, receiver) = unbounded();

        let server_handle = Builder::new()
            .spawn(move || {
                let server = init_server();
                loop {
                    server.poll_timeout(Duration::from_millis(100));
                    if let Ok(_) = receiver.recv_timeout(Duration::from_millis(100)) {
                        break;
                    }
                }
            })
            .unwrap();

        let client_handle = Builder::new()
            .spawn(move || {
                let client = reqwest::blocking::Client::new();
                let mut map = HashMap::new();
                map.insert("title", "note");

                let res = client
                    .get("http://127.0.0.1:8080/read")
                    .json(&map)
                    .send()
                    .unwrap();
                let output = res.text().unwrap();
                assert!(output.contains("SUCCESS"));
                assert!(already_exists(&String::from("note")));
                let mut data_file = OpenOptions::new()
                    .read(true)
                    .open(path.clone())
                    .expect("cannot open file");

                let mut data = String::new();
                data_file.read_to_string(&mut data).expect("read failed");

                assert!(data.contains("xyz"));

                let _ = sender.send(true);
            })
            .unwrap();

        let _ = server_handle.join();
        let _ = client_handle.join();
    }

    #[test]
    fn test_note_delete() {
        let path = format!("{}{}", NOTES_DIR, "note");
        if already_exists(&String::from("note")) {
            fs::remove_file(path.clone()).expect("can not remove file");
        }

        let note = Note {
            title: String::from("note"),
            description: String::from("xyz"),
        };
        create_note(&note);

        let (sender, receiver) = unbounded();

        let server_handle = Builder::new()
            .spawn(move || {
                let server = init_server();
                loop {
                    server.poll_timeout(Duration::from_millis(100));
                    if let Ok(_) = receiver.recv_timeout(Duration::from_millis(100)) {
                        break;
                    }
                }
            })
            .unwrap();

        let client_handle = Builder::new()
            .spawn(move || {
                let client = reqwest::blocking::Client::new();
                let mut map = HashMap::new();
                map.insert("title", "note");

                let res = client
                    .delete("http://127.0.0.1:8080/delete")
                    .json(&map)
                    .send()
                    .unwrap();
                let output = res.text().unwrap();
                assert!(output.contains("SUCCESS"));
                assert!(!already_exists(&String::from("note")));

                let _ = sender.send(true);
            })
            .unwrap();

        let _ = server_handle.join();
        let _ = client_handle.join();
    }
}
