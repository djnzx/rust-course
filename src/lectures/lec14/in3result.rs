use std::fs::File;
use std::io;
use std::io::Error;
use tokio_postgres::tls::NoTlsStream;
use tokio_postgres::{Client, Connection, NoTls, Socket};

#[ignore]
#[tokio::test]
async fn test1() {
    let mut buffer = String::new();
    let size: Result<usize, Error> = io::stdin().read_line(&mut buffer);

    let file: Result<File, Error> = File::open("input.txt");

    let sql: Result<(Client, Connection<Socket, NoTlsStream>), tokio_postgres::Error> =
        tokio_postgres::connect("...", NoTls).await;
}

#[ignore]
#[tokio::test]
async fn test2() {
    let mut buffer = String::new();
    let size = io::stdin()
        .read_line(&mut buffer)
        .unwrap();

    let file = File::open("input.txt").unwrap();

    let sql: Result<(Client, Connection<Socket, NoTlsStream>), tokio_postgres::Error> =
        tokio_postgres::connect("...", NoTls).await;
}

fn test3(s: String) {
    let x = s.parse::<i32>();
    let k = x.map(|n| n * 10);
    let z = k.unwrap_or(-13);
}
