use tokio_postgres::Client;

pub async fn get_user(client: &Client) -> Result<(), tokio_postgres::Error> {
    client.execute("SELECT * FROM users", params)
}
