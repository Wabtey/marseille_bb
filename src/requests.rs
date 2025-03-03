use crate::{activities::Activity, roommates::Roommate};

pub struct Request<'a> {
    pub roommate: &'a Roommate,
    pub requested_activity: &'a Activity,
}

// TODO: Browse through requests

impl Request<'_> {
    pub fn display(&self) -> String {
        format!(
            "{}: {}",
            self.roommate.name,
            self.requested_activity.display()
        )
    }
}

pub fn init_requests<'a>(
    roommates: &'a [Roommate],
    activities: &'a [Activity],
) -> Vec<Request<'a>> {
    vec![Request {
        requested_activity: &activities[0],
        roommate: &roommates[4],
    }]
}
