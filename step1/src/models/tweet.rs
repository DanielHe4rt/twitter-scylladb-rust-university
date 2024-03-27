use charybdis::macros::charybdis_model;
use charybdis::types::{Text, Timestamp, Timeuuid, Uuid};

#[charybdis_model(
table_name = tweets,
partition_keys = [tweet_id],
clustering_keys = [time],
table_options = "CLUSTERING ORDER BY (time DESC)"
)]
#[derive(Debug, Clone, Default)]
pub struct Tweet {
    pub tweet_id: Uuid,
    pub author: Text,
    pub text: Text,
    pub time: Option<Timeuuid>,
    pub created_at: Timestamp
}