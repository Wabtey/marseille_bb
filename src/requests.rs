use rand::{Rng, seq::IndexedRandom};

use crate::{activities::Activity, constraints::Constraints, roommates::Roommate};

pub struct Request<'a> {
    pub roommate: &'a Roommate,
    pub requested_activity: &'a Activity,
    /// additional constraint that the roommate could have.
    /// Could theoretically be incompatible with the activity's constraints ("I want to climb alone").
    ///
    /// "Cocoa: I want to hangout but not with more than 3 ppl but I don't want to be alone neither."
    pub constraints: Option<Constraints>,
}

// TODO: Browse through requests

impl Request<'_> {
    pub fn display(&self) -> String {
        format!(
            "{}: {}, {:?}",
            self.roommate.name,
            self.requested_activity.display(),
            self.constraints
        )
    }

    /// Converts the request to a natural language description
    pub fn to_natural_language(&self) -> String {
        let activity_phrase = {
            let sentences = self.requested_activity.full_sentences;
            let idx = rand::rng().random_range(0..sentences.len());
            sentences[idx].to_string()
        };

        // Handle people constraints if present
        let people_phrase = if let Some(constraints) = &self.constraints {
            if let Some((min, max)) = constraints.people {
                match (min, max) {
                    (0, 0) => " alone".to_string(),
                    (1, 1) => " with exactly one other person".to_string(),
                    (1, max) if max > 1 => format!(
                        " but not with more than {} people but I don't want to be alone either",
                        max
                    ),
                    (min, max) if min == max => format!(" with exactly {} other people", min),
                    (min, max) => format!(
                        " with at least {} but no more than {} other people",
                        min, max
                    ),
                }
            } else {
                "".to_string()
            }
        } else {
            "".to_string()
        };

        // Handle energy constraints
        let energy_phrase = if let Some(constraints) = &self.constraints {
            if constraints.energy > 0 {
                format!(" (will cost {} additional energy)", constraints.energy)
            } else {
                "".to_string()
            }
        } else {
            "".to_string()
        };

        // Handle money constraints
        let money_phrase = if let Some(constraints) = &self.constraints {
            if constraints.money > 0 {
                format!(" (I want to spend at least {}e)", constraints.money)
            } else {
                "".to_string()
            }
        } else {
            "".to_string()
        };

        format!(
            "{}: {}{}{}{}",
            self.roommate.name, activity_phrase, people_phrase, energy_phrase, money_phrase
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
        constraints: None,
    }]
}
