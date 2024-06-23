use scylla::{Session, SessionBuilder};

pub async fn setup_connection() -> Session {
    let session = SessionBuilder::new()
        .known_nodes(vec!["localhost:9042", "localhost:9040", "localhost:9041", "localhost:9043", "localhost:9044", "localhost:9045"])
        .build()
        .await
        .unwrap();

    let _ = session.use_keyspace("uni_twitter", true).await;

    session
}