use charybdis::macros::charybdis_model;
use charybdis::types::{Text, Timeuuid, Uuid};
use fake::Fake;
use fake::faker::internet::en::Username;
use fake::faker::lorem::en::Sentence;

#[charybdis_model(
table_name = tweets,
partition_keys = [tweet_id],
clustering_keys = [created_at],
table_options = "CLUSTERING ORDER BY (created_at DESC)"
)]
#[derive(Debug, Clone, Default)]
pub struct Tweet {
    pub tweet_id: Uuid,
    pub author: Text,
    pub text: Text,
    pub created_at: Timeuuid
}

impl Tweet {
    pub fn fake_tweet(&mut self) {
        self.tweet_id = Uuid::new_v4();
        self.author = Username().fake();
        self.text = Sentence(3..5).fake();
        self.created_at = Timeuuid::now_v1(&[1,2,3,4,5,6])
    }

}