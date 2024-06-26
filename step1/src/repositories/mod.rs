use std::sync::Arc;

use scylla::Session;

use crate::repositories::timeline_service::TimelineService;
use crate::repositories::tweet_service::TweetService;

pub mod tweet_service;
pub mod timeline_service;

pub struct Repositories {
    pub tweet_service: TweetService,
    pub timeline_service: TimelineService,
}

impl Repositories {
    pub async fn new(connection: Arc<Session>) -> Self {
        let tweet_service = TweetService { connection: Arc::clone(&connection) };
        let timeline_service = TimelineService { connection: Arc::clone(&connection) };
        Repositories {
            tweet_service,
            timeline_service,
        }
    }
}