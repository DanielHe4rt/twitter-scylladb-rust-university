use std::sync::Arc;
use log::info;
use crate::repositories::timeline_service::{TimelineService, TimelineServiceTrait};

pub async fn fetch_timelines(
    username: Arc<String>,
    timeline_service: Arc<TimelineService>,
) {
    let username = username.as_str();
    loop {
        let fetch_timeline = timeline_service
            .get_timeline_by_username(username).await;

        match fetch_timeline {
            Ok(_) => info!("Timeline fetched!"),
            Err(e) => info!("Error fetching timeline: {:?}", e)
        }
    }
}
