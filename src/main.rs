use std::{env, sync::Arc};


use salvo::prelude::*;
use sqlx::sqlite::SqlitePoolOptions;
use sub_conver::*;
use tokio::sync::Mutex;

mod handlers;
use handlers::*;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();
    dotenv::dotenv().ok();
    let db_url = env::var("DATABASE_URL").unwrap();
    let domain = env::var("DOMAIN").unwrap();
    let port = env::var("PORT").unwrap();
    let db_pool: DBPool = Arc::new(Mutex::new(State{
        db_pool: SqlitePoolOptions::new()
            .max_connections(4)
            .connect(&db_url)
            .await
            .expect("数据库连接失败"),
        domain: domain,
        port:port.clone(),
    }));

    let router = Router::new()
        .hoop(affix_state::inject(db_pool))
        .get(index)
        .push(Router::with_path("api")
            .post(new)
            .push(Router::with_path("<urlid>")
                .get(get)
            )
        );

    let acceptor = TcpListener::new(format!("0.0.0.0:{port}")).bind().await;
    Server::new(acceptor).serve(router).await;
}
