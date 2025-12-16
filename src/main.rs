use std::ops::Add;

mod command;
mod database;
mod server;

fn main() {
    let add = "127.0.0.1:7635";
    eprintln!("Server will start at {:?}", add);
}
