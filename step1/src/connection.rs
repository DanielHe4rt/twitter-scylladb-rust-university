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
        .known_nodes(vec!["node-0.aws-sa-east-1.37e013c7e0d4ae45432a.clusters.scylla.cloud", "node-1.aws-sa-east-1.37e013c7e0d4ae45432a.clusters.scylla.cloud", "node-2.aws-sa-east-1.37e013c7e0d4ae45432a.clusters.scylla.cloud"])
        .user("scylla".to_string(), "W0xMPBaT48kNOvZ".to_string())
        .default_execution_profile_handle(execution_profile.into_handle())
        .build()
        .await
        .unwrap();

    let _ = session.use_keyspace("uni_twitter", true).await;

    session
}