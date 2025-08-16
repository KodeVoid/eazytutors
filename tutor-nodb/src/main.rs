use std::io;
use std::net::TcpListener;
use tutor_nodb::run; // from lib.rs

#[actix_rt::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    run(listener)?.await
}
