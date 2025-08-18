use tokio::runtime::Handle;
use tutor_nodb::connect_db;
use std::io;
use std::net::TcpListener;
use tutor_nodb::run; // from lib.rs
#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    let pool = Handle::current().block_on(connect_db()).expect("Could not connect to database");
    run(listener,pool)?.await
}
