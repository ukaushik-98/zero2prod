use std::net::TcpListener;

use zero2prod::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("8000").expect("error binding port");
    let test = "dumb";

    run(listener)?.await
}
