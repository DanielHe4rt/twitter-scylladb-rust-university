use charybdis::macros::charybdis_model;
use charybdis::types::{Boolean, Text, Timeuuid, Uuid};
use fake::Fake;
use fake::faker::internet::en::Username;
use crate::models::tweet::Tweet;

#[charybdis_model(
table_name = timeline,
partition_keys = [username],
clustering_keys = [created_at, tweet_id],
table_options = "CLUSTERING ORDER BY (created_at DESC)"
)]
#[derive(Debug, Default, Clone)]

pub struct Timeline {
    pub username: Text,
    pub tweet_id: Uuid,
    pub author: Text,
    pub text: Text,
    pub liked: Boolean,
    pub bookmarked: Boolean,
    pub retweeted: Boolean,
    pub created_at: Timeuuid
}

impl Timeline {
    pub fn fake_timeline(&mut self, tweet: Tweet) {
        self.username = Username().fake();
        self.tweet_id = tweet.tweet_id;
        self.author = tweet.author;
        self.text = tweet.text;
        self.liked = rand::random::<bool>();
        self.bookmarked = rand::random::<bool>();
        self.retweeted = rand::random::<bool>();
        self.created_at = Timeuuid::now_v1(&[1,2,3,4,5,6]);
    }

}