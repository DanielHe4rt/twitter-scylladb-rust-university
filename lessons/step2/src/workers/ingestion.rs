use std::fmt::format;
use std::sync::Arc;
use std::time::Instant;

use log::{debug, trace};

use crate::models::timeline::Timeline;
use crate::models::tweet::Tweet;
use crate::repositories::Repositories;
use crate::repositories::timeline_service::TimelineServiceTrait;
use crate::repositories::tweet_service::TweetServiceTrait;
use crate::workers::stats::Stats;

pub async fn twitter_ingestion(
    id: usize,
    repositories: Arc<Repositories>,
) -> anyhow::Result<()>{
    debug!("worker # {} ready", id);

    let prefix = format!("#{}", id);
    let mut s = Stats::new();

    let mut tweet = Tweet::default();
    let mut timeline = Timeline::default();

    loop {
        tweet.fake_tweet();
        timeline.fake_timeline(tweet.clone());

        repositories.tweet_service.create_tweet(&tweet).await?;
        repositories.timeline_service.insert_to_timeline(&timeline).await?;
        repositories.timeline_service.get_timeline_by_username(&timeline.username).await?;

        let ts = Instant::now();
        trace!("worker # {} insert/read {:?}", id, ts.elapsed());

        s.record(ts);
        s.print(&prefix);
    }
}