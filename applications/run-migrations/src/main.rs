use database_connection;
use database_connection::sqlx;

use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let connection = database_connection::connect().await.unwrap();

    let _ = sqlx::query("SELECT 1").fetch_one(&connection).await;

    sleep(Duration::from_millis(1000)).await;
    println!("I'm done now");
}
