use crate::constraints::Constraints;

/// You can't assign an activity twice to the same person
#[derive(Debug, Clone)]
pub struct Activity {
    pub name: &'static str,
    pub full_sentences: &'static [&'static str],
    pub happiness_factor: u8,
    /// IDEA: control how long the activity is (with a minimum, etc)
    ///
    /// (hour, min)
    pub duration: (u8, u8),
    pub constraints: Option<Constraints>,
}

impl Activity {
    pub fn new(
        name: &'static str,
        full_sentences: &'static [&'static str],
        happiness_factor: u8,
        constraints: Option<Constraints>,
        duration: (u8, u8),
    ) -> Activity {
        Activity {
            name,
            full_sentences,
            happiness_factor,
            duration,
            constraints,
        }
    }

    /// Displays activity like that:
    /// - Play Rocket League, 2h, min 2/max 6
    /// - Shower, 30min, max 2
    /// - Museum visit, need $5
    pub fn display(&self) -> String {
        format!(
            "{}, {}{} - {}",
            self.name,
            if self.duration.0 == 0 {
                String::new()
            } else {
                format!("{}h", self.duration.0)
            },
            if self.duration.1 == 0 {
                String::new()
            } else {
                self.duration.1.to_string()
            },
            if let Some(constraint) = &self.constraints {
                constraint.display()
            } else {
                String::new()
            }
        )
    }
}

pub fn init_activities() -> Vec<Activity> {
    vec![
        /* --------------------------------- Gaming --------------------------------- */
        Activity {
            name: "GIMME THE SWITCH!",

            full_sentences: &[
                "GIMME THE SWITCH!",
                "Who wants to play MarioKart real fast?",
                "Switch Time!",
            ],
            happiness_factor: 1,
            duration: (2, 0),
            constraints: Some(Constraints {
                people: Some((0, 2)),
                energy: 1,
                ..Default::default()
            }),
        },
        Activity {
            name: "Rocket League",
            full_sentences: &[
                "Who for a smol Rocket League?",
                "I just want to play Rocket League",
            ],
            happiness_factor: 3,
            duration: (2, 0),
            constraints: Some(Constraints {
                people: Some((0, 6)),
                energy: 2,
                ..Default::default()
            }),
        },
        Activity {
            name: "Hyper Rocket League",
            full_sentences: &[
                "5 hours of Rocket League RIGHT NOW",
                "URGE TO PLAY ROCKET LEAGUE ALL NIGHT",
                "Hyper focus Rocket League or else",
            ],
            happiness_factor: 5,
            duration: (5, 0),
            constraints: Some(Constraints {
                people: Some((0, 6)),
                energy: 4,
                ..Default::default()
            }),
        },
        /* --------------------------------- Nature --------------------------------- */
        Activity {
            name: "Climbing in the Callenques",
            full_sentences: &[
                "Just want to go climb man.",
                "Who's down to go climbing with me?",
            ],
            happiness_factor: 5,
            duration: (4, 0),
            constraints: Some(Constraints {
                people: Some((2, 5)),
                energy: 5,
                ..Default::default()
            }),
        },
        Activity {
            name: "Walk in the Callenques",
            full_sentences: &["It's HIKING time!", "Do you want to go hike with me?"],
            happiness_factor: 5,
            duration: (3, 0),
            constraints: Some(Constraints {
                people: None,
                energy: 4,
                ..Default::default()
            }),
        },
        /* --------------------------------- Cosina --------------------------------- */
        // REFACTOR: these activities must be succeed in the day
        Activity {
            name: "Cook for the Homies",
            full_sentences: &[
                "Need to cook, jesse.",
                "The roommates are starving. And it's my duty to save them.",
            ],
            happiness_factor: 3,
            duration: (2, 0),
            constraints: Some(Constraints {
                people: Some((0, 3)),
                energy: 2,
                money: 10,
            }),
        },
        Activity {
            name: "Buy food for the Homies",
            full_sentences: &[
                "Can't be bothered to cook, just order smth",
                "Can't we order something instead?",
                "I won't cook.",
                "I want to try out the slice of pizza at 2e :)",
            ],
            happiness_factor: 3,
            duration: (0, 30),
            constraints: Some(Constraints {
                people: None,
                energy: 0,
                money: 50,
            }),
        },
    ]
}
