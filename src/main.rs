use constants::*;
use request_handling::*;
use rouille::*;

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
