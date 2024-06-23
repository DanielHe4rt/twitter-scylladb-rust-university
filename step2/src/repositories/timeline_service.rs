use std::sync::Arc;

use charybdis::QueryError;
use charybdis::types::Timeuuid;
use scylla::prepared_statement::PreparedStatement;
use scylla::Session;

use crate::models::timeline::Timeline;
use crate::models::tweet::Tweet;

pub trait TimelineServiceTrait {
    async fn insert_to_timeline(&self, username: &str, tweet: &Tweet) -> Result<Timeline, QueryError>;

    async fn get_timeline_by_username(&self, username: &str) -> Result<(), QueryError>;
}

pub struct TimelineService {
    connection: Arc<Session>,
    timeline_insert_query: PreparedStatement,
    timeline_select_query: PreparedStatement
}

const INSERT_TIMELINE_QUERY: &str = "INSERT INTO timeline (username, tweet_id, author, text, liked, bookmarked, retweeted, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?)";

const SELECT_TIMELINE_QUERY: &str = "SELECT username, tweet_id, author, text, liked, bookmarked, retweeted, created_at FROM timeline WHERE username = ? LIMIT 50";
impl TimelineService {
    pub async fn new(connection: Arc<Session>) -> Self {
        let timeline_insert_query = connection
            .prepare(INSERT_TIMELINE_QUERY)
            .await
            .unwrap();

        let timeline_select_query = connection
            .prepare(SELECT_TIMELINE_QUERY)
            .await
            .unwrap();

        Self {
            connection,
            timeline_insert_query,
            timeline_select_query
        }
    }
}

impl TimelineServiceTrait for TimelineService {

    async fn insert_to_timeline(&self, username: &str, tweet: &Tweet) -> Result<Timeline, QueryError> {
        let random_liked = rand::random::<bool>();
        let random_bookmarked = rand::random::<bool>();
        let random_retweeted = rand::random::<bool>();

        let timeline = Timeline {
            username: username.to_string(),
            tweet_id: tweet.tweet_id.clone(),
            author: tweet.author.clone(),
            text: tweet.author.clone(),
            liked: random_liked,
            bookmarked: random_bookmarked,
            retweeted: random_retweeted,
            created_at: Timeuuid::now_v1(&[1, 2, 3, 4, 5, 6]),
        };

        let session = self.connection.execute(&self.timeline_insert_query, (
            timeline.username.clone(),
            timeline.tweet_id.clone(),
            timeline.author.clone(),
            timeline.text.clone(),
            timeline.liked.clone(),
            timeline.bookmarked.clone(),
            timeline.retweeted.clone(),
            tweet.created_at,
        )).await;

        match session {
            Ok(_) => Ok(timeline),
            Err(e) => Err(e),
        }
    }

    async fn get_timeline_by_username(&self, username: &str) -> Result<(), QueryError> {


        let timeline = self.connection.execute(&self.timeline_select_query, (username, )).await;

        match timeline {
            Ok(_res) => Ok(()),
            Err(e) => Err(e),
        }
    }
}