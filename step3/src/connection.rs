use scylla::{Session, SessionBuilder};

pub async fn setup_connection() -> Session {
    let session = SessionBuilder::new()
        .known_nodes(vec!["node-0.aws-eu-west-2.c19ad2013a0beadca5a8.clusters.scylla.cloud", "node-1.aws-eu-west-2.c19ad2013a0beadca5a8.clusters.scylla.cloud", "node-2.aws-eu-west-2.c19ad2013a0beadca5a8.clusters.scylla.cloud"])
        .user("scylla", "Sco98er4ZhuimVX")
        .build()
        .await
        .unwrap();

    let _ = session.use_keyspace("mykeyspace", true).await;

    session
}