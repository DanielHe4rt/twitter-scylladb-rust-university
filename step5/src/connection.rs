use std::sync::Arc;
use charybdis::options::Consistency;
use scylla::{ExecutionProfile, Session, SessionBuilder};
use scylla::load_balancing::DefaultPolicy;

pub async fn setup_connection() -> Session {

    let policies = DefaultPolicy::builder()
        .prefer_datacenter("DC1".to_string())
        .token_aware(false)
        .build();

    let execution_profile = ExecutionProfile::builder()
        .consistency(Consistency::LocalQuorum)
        .load_balancing_policy(policies)
        .build();

    let session = SessionBuilder::new()
        .known_nodes(vec!["localhost:9042","localhost:9043","localhost:9044"])
        .default_execution_profile_handle(execution_profile.into_handle())
        .build()
        .await
        .unwrap();

    let _ = session.use_keyspace("mykeyspace", true).await;

    session
}