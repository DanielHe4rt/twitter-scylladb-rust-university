use std::sync::Arc;
use scylla::{ExecutionProfile, load_balancing, Session, SessionBuilder};
use scylla::statement::{Consistency, SerialConsistency};

pub async fn setup_connection() -> Session {

    let execution_profile = ExecutionProfile::builder()
        .consistency(Consistency::Quorum)
        .build();

    let session = SessionBuilder::new()
        .known_nodes(vec!["localhost:9042"])
        .default_execution_profile_handle(execution_profile.into_handle())
        .build()
        .await
        .unwrap();

    let _ = session.use_keyspace("uni_twitter", true).await;

    session
}