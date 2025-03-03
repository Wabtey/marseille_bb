static ROOMMATES_NAME: &[&str] = &["Ed'", "Cocoa", "Elia", "Marie", "Myriam", "Mae", "Flo"];

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Roommate {
    pub name: String,
    pub happiness: u8,
    pub energy: u8,
    pub money: u8,
}

impl Default for Roommate {
    fn default() -> Self {
        Roommate {
            name: String::new(),
            happiness: 10,
            energy: 10,
            money: 250,
        }
    }
}

impl Roommate {
    pub fn new(name: String) -> Self {
        Roommate {
            name,
            ..Default::default()
        }
    }
}

pub fn init() -> Vec<Roommate> {
    let mut roommates = vec![];
    for name in ROOMMATES_NAME {
        let mut roommate = Roommate::new(name.to_string());
        roommate.happiness = rand::random::<u8>() % 11; // Random happiness between 0 and 10
        roommates.push(roommate);
    }
    roommates
}
