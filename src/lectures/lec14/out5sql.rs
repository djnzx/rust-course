use tokio_postgres::{Error, NoTls};

// will hang / timeout without connection. so, disabled
#[ignore]
#[tokio::test]
async fn db_test() -> Result<(), Error> {
    let cred = "host=localhost user=postgres password=pg123456 dbname=fs8";
    let (client, connection) = tokio_postgres::connect(cred, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Помилка підключення: {}", e);
        }
    });

    let stmt = "INSERT INTO person (name, salary) VALUES ($1, $2)";

    client
        .execute(stmt, &[&"Олексій", &5000])
        .await?;

    println!("Дані успішно записано у базу!");

    Ok(())
}
