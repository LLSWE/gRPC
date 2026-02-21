use tokio_postgres::{Client, Error, NoTls};

pub async fn connect_db() -> Result<(), Error> {
    let (client, connection) =
        tokio_postgres::connect("host=localhost user=postgres", NoTls).await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Error Connecting to database: {}", e);
        }
    });
    Ok(())
}
