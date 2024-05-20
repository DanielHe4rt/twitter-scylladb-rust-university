use scylla::{Session, SessionBuilder};

pub async fn setup_connection() -> Session {


    let session = SessionBuilder::new()
        .known_nodes(vec!["localhost:9042"])
        .build()
        .await
        .unwrap();

    let _ = session.use_keyspace("mykeyspace", true).await;

    session
}