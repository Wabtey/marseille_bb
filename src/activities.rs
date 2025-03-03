use crate::constraints::Constraints;

#[derive(Debug)]
pub struct Activity {
    pub name: &'static str,
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
        happiness_factor: u8,
        constraints: Option<Constraints>,
        duration: (u8, u8),
    ) -> Activity {
        Activity {
            name,
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
        Activity {
            name: "GIMME THE SWITCH!",
            happiness_factor: 1,
            duration: (2, 0),
            constraints: Some(Constraints {
                people: Some((0, 2)),
                energy: 1,
                money: 0,
            }),
        },
        Activity {
            name: "Doing stuff",
            happiness_factor: 3,
            duration: (2, 30),
            constraints: Some(Constraints::generate(7)),
        },
    ]
}
