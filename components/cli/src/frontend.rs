use common::database;
use common::prelude::*;
use frontend::{listener, router, server};

fn listener_from_args(args: &Vec<String>) -> String {
    if args.len() > 0 {
        return args[0].clone();
    }
    "127.0.0.1:3000".to_string()
}

pub async fn run(args: &Vec<String>) -> NoResult {
    let url = env::var("DATABASE_URL")?;

    let pool = database::connect(&url).await?;
    let listen = listener::new(listener_from_args(args)).await?;
    let route = router::new(pool);

    info!("starting frontend runner...");

    server::serve(listen, route).await
}
