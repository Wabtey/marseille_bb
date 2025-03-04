#[derive(Debug, Default, Clone)]
pub struct Constraints {
    /// (min, max)
    pub people: Option<(u8, u8)>,
    pub energy: u8,
    pub money: u8,
}

impl Constraints {
    pub fn display(&self) -> String {
        format!(
            "{}{}{}",
            if let Some(min_max) = self.people {
                format!("min {}, max {} people", min_max.0, min_max.1)
            } else {
                String::new()
            },
            if self.energy != 0 {
                format!(", consume {} energy", self.energy)
            } else {
                String::new()
            },
            if self.money == 0 {
                String::new()
            } else {
                format!(", need ${}", self.money)
            }
        )
    }

    pub fn generate(people_number: u8) -> Self {
        Constraints {
            people: {
                if !rand::random_bool(0.5) {
                    None
                } else {
                    let prob = rand::random::<u8>() % 100;
                    let min = match prob {
                        0..=29 => 0,
                        30..=39 => 1,
                        40..=74 => 2 + rand::random::<u8>() % 4, // 2 <= x < 6
                        75..=84 => 6,
                        85..=99 => people_number,
                        _ => u8::MAX,
                    };

                    let max = match prob {
                        0..=29 => 0,
                        30..=39 => 1,
                        40..=74 => 2 + rand::random::<u8>() % 4, // 2 <= x < 6
                        75..=84 => 6,
                        85..=99 => people_number,
                        _ => u8::MAX,
                    };

                    if min == 0 && max == people_number {
                        None
                    } else {
                        Some((min, max))
                    }
                }
            },
            energy: 0,
            money: 100,
        }
    }
}
