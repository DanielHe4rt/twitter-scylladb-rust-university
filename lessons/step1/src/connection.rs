use charybdis::options::Consistency;
use scylla::{ExecutionProfile, Session, SessionBuilder};
use scylla::load_balancing::DefaultPolicy;

pub async fn setup_connection() -> Session {
    let policies = DefaultPolicy::builder()
        .token_aware(false)
        .build();

    let execution_profile = ExecutionProfile::builder()
        .consistency(Consistency::Quorum)
        .load_balancing_policy(policies)
        .build();

    let session = SessionBuilder::new()
        .known_nodes(vec!["localhost:9040", "localhost:9041", "localhost:9042"])
        .default_execution_profile_handle(execution_profile.into_handle())
        .build()
        .await
        .unwrap();

    let _ = session.use_keyspace("uni_twitter", true).await;

    session
}