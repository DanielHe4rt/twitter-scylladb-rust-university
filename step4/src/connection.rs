use std::sync::Arc;
use scylla::{ExecutionProfile, load_balancing, Session, SessionBuilder};
use scylla::statement::{Consistency, SerialConsistency};

pub async fn setup_connection() -> Session {

    let execution_profile = ExecutionProfile::builder()
        .consistency(Consistency::Quorum)
        .load_balancing_policy(Arc::new(load_balancing::DefaultPolicy::builder()
            .prefer_datacenter("AWS_EU_CENTRAL_1".to_owned())
        ))
        .build();

    let session = SessionBuilder::new()
        .known_nodes(vec!["node-0.aws-eu-central-1.4cb7c0a2593e7a950fb5.clusters.scylla.cloud", "node-1.aws-eu-central-1.4cb7c0a2593e7a950fb5.clusters.scylla.cloud", "node-2.aws-eu-central-1.4cb7c0a2593e7a950fb5.clusters.scylla.cloud"])
        .user("scylla", "LyS3EBPpcZi6V1R")
        .build()
        .await
        .unwrap();

    let _ = session.use_keyspace("mykeyspace", true).await;

    session
}