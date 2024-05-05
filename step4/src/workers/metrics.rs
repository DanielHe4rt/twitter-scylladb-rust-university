use std::sync::Arc;
use crate::repositories::timeline_service::{TimelineService, TimelineServiceTrait};


pub async fn fetch_timelines(
    timeline_service: Arc<TimelineService>,
) {
    loop {
        let _ = timeline_service
            .get_liked_timeline_by_username("danielhe4rt").await;

        let fetch_timeline = timeline_service
            .get_liked_timeline_by_username("danielhe4rt").await;

        match fetch_timeline {
            Ok(_) => println!("Timeline fetched!"),
            Err(e) => println!("Error fetching timeline: {:?}", e)
        }
    }
}