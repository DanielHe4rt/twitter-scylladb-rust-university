use std::sync::Arc;
use rand::prelude::SliceRandom;
use crate::repositories::timeline_service::{TimelineService, TimelineServiceTrait};


pub async fn fetch_timelines(
    timeline_service: Arc<TimelineService>,
    users: Arc<Vec<String>>
) {
    loop {
        let user = users.choose(&mut rand::thread_rng()).unwrap();
        let fetch_timeline = timeline_service
            .get_liked_timeline_by_username(user).await;

        let fetch_timeline = timeline_service
            .get_reverse_timeline_by_username(user).await;

        match fetch_timeline {
            Ok(_) => println!("Timeline fetched!"),
            Err(e) => println!("Error fetching timeline: {:?}", e)
        }
    }
}