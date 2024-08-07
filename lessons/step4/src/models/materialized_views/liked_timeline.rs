use charybdis::macros::charybdis_view_model;
use charybdis::types::{Boolean, Text, Timeuuid, Uuid};

#[charybdis_view_model(
table_name=liked_timeline,
base_table=timeline,
partition_keys=[username],
clustering_keys=[created_at, tweet_id]
)]
#[derive(Debug, Default)]
pub struct LikedTimeline {
    pub username: Text,
    pub tweet_id: Uuid,
    pub author: Text,
    pub text: Text,
    pub liked: Boolean,
    pub bookmarked: Boolean,
    pub retweeted: Boolean,
    pub created_at: Timeuuid
}