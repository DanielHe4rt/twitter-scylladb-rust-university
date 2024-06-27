use std::sync::Arc;

use scylla::prepared_statement::PreparedStatement;
use scylla::Session;

use crate::models::timeline::Timeline;

pub trait TimelineServiceTrait {
    async fn insert_to_timeline(&self, timeline: &Timeline) -> anyhow::Result<()>;

    async fn get_timeline_by_username(&self, username: &str) -> anyhow::Result<()>;

    async fn get_liked_timeline_by_username(&self, username: &str) -> anyhow::Result<()>;
}

pub struct TimelineService {
    connection: Arc<Session>,
    timeline_insert_query: PreparedStatement,
    timeline_select_query: PreparedStatement,
    liked_timeline_query: PreparedStatement,
}

const INSERT_TIMELINE_QUERY: &str = "INSERT INTO timeline (username, tweet_id, author, text, liked, bookmarked, retweeted, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?)";

const SELECT_TIMELINE_QUERY: &str = "SELECT username, tweet_id, author, text, liked, bookmarked, retweeted, created_at FROM timeline WHERE username = ? LIMIT 50";

const SELECT_LIKED_TIMELINE_QUERY: &str = "SELECT \
                        username, tweet_id, author, text, liked, bookmarked, retweeted, created_at \
                   FROM \
                        timeline \
                   WHERE \
                        username = ? AND \
                        liked = ? \
                   LIMIT 50
                   ALLOW FILTERING";

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

        let liked_timeline_query = connection
            .prepare(SELECT_LIKED_TIMELINE_QUERY)
            .await
            .unwrap();

        Self {
            connection,
            timeline_insert_query,
            timeline_select_query,
            liked_timeline_query,
        }
    }
}

impl TimelineServiceTrait for TimelineService {
    async fn insert_to_timeline(&self, timeline: &Timeline) -> anyhow::Result<()> {
        self.connection.execute(&self.timeline_insert_query, (
            &timeline.username,
            &timeline.tweet_id,
            &timeline.author,
            &timeline.text,
            &timeline.liked,
            &timeline.bookmarked,
            &timeline.retweeted,
            &timeline.created_at,
        )).await?;

        Ok(())
    }

    async fn get_timeline_by_username(&self, username: &str) -> anyhow::Result<()> {
        self.connection.execute(&self.timeline_select_query, (username, )).await?;

        Ok(())
    }

    async fn get_liked_timeline_by_username(&self, username: &str) -> anyhow::Result<()> {
        self.connection.execute(&self.liked_timeline_query, (username, true)).await?;

        Ok(())
    }
}