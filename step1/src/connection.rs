use scylla::{Session, SessionBuilder};

pub async fn setup_connection() -> Session {
    let session = SessionBuilder::new()
        .known_nodes(vec!["node-0.aws-sa-east-1.5dd4505d4dd889a2057e.clusters.scylla.cloud", "node-1.aws-sa-east-1.5dd4505d4dd889a2057e.clusters.scylla.cloud", "node-2.aws-sa-east-1.5dd4505d4dd889a2057e.clusters.scylla.cloud"])
        .user("scylla", "K3AIgWveEL68aij")
        .build()
        .await
        .unwrap();

    let _ = session.use_keyspace("mykeyspace", true).await;

    session
}