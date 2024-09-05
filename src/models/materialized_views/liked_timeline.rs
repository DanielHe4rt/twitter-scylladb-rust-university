use charybdis::macros::charybdis_view_model;
use charybdis::types::{Boolean, Text, Timeuuid, Uuid};

#[charybdis_view_model(
table_name=timeline_liked,
base_table=timeline,
partition_keys=[username, liked],
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

#[charybdis_view_model(
table_name=first_timeline_tweets,
base_table=timeline,
partition_keys=[username, liked],
clustering_keys=[created_at, tweet_id],
table_options = "CLUSTERING ORDER BY (created_at ASC)"
)]
#[derive(Debug, Default)]
pub struct FirstLikedTweets {
    pub username: Text,
    pub tweet_id: Uuid,
    pub author: Text,
    pub text: Text,
    pub liked: Boolean,
    pub bookmarked: Boolean,
    pub retweeted: Boolean,
    pub created_at: Timeuuid
}