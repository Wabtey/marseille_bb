use std::io::{self, Write};

/* -------------------------------------------------------------------------- */
/*                                    Model                                   */
/* -------------------------------------------------------------------------- */

static ROOMMATES_NAME: &[&str] = &["Ed'", "Cocoa", "Elia", "Marie", "Myriam", "Mae", "Flo"];

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Roommate {
    name: String,
    happiness: u8,
    energy: u8,
}

impl Default for Roommate {
    fn default() -> Self {
        Roommate {
            name: String::new(),
            happiness: 10,
            energy: 8,
        }
    }
}

impl Roommate {
    fn new(name: String) -> Self {
        Roommate {
            name,
            ..Default::default()
        }
    }
}

fn init() -> Vec<Roommate> {
    let mut roommates = vec![];
    for name in ROOMMATES_NAME {
        let mut roommate = Roommate::new(name.to_string());
        roommate.happiness = rand::random::<u8>() % 11; // Random happiness between 0 and 10
        roommates.push(roommate);
    }
    roommates
}

/* -------------------------------------------------------------------------- */
/*                                Gameplay loop                               */
/* -------------------------------------------------------------------------- */

fn main() {
    let roommates = init();
    let mut days = 1;

    while days <= 7 && !roommates.iter().any(|r| r.happiness == 0) {
        print_status(roommates.clone(), days);
        println!("Myriam: GIMME THE SWITCH!");
        // TODO: feat - browse though request
        // TODO: feat - assign activities

        /* ------------------------------- End of Day ------------------------------- */
        let mut input = String::new();
        print!("Press return to advance the day...");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        days += 1;
    }

    print_status(roommates.clone(), days);

    if days == 8 {
        println!("GG");
    } else {
        println!(
            "Game Over! {} was unhappy.",
            roommates.iter().find(|r| r.happiness == 0).unwrap().name
        );
    }
}

/* -------------------------------------------------------------------------- */
/*                                     CLI                                    */
/* -------------------------------------------------------------------------- */

fn print_status(roommates: Vec<Roommate>, days: u8) {
    print!("\x1B[2J\x1B[1;1H");
    println!("{:-^90}", format!("Day {}", days));
    let mut index = 0;
    while index < roommates.len() {
        let remaining = roommates.len() - index;
        let display_count = if remaining < 4 { remaining } else { 4 };

        for _ in 0..display_count {
            if index < roommates.len() {
                print!("{:>width$}: [", roommates[index].name, width = 7);
                for _ in 0..roommates[index].happiness {
                    print!("\x1b[93m-\x1b[0m");
                }
                for _ in roommates[index].happiness..10 {
                    print!(" ");
                }
                print!("]  ");
            }
            index += 1;
        }
        println!();
        for _ in 0..display_count {
            if index - 4 < roommates.len() {
                print!("{:>width$}[", "", width = 7 + 2);
                for _ in 0..roommates[index - 4].energy {
                    print!("\x1b[31m-\x1b[0m");
                }
                for _ in roommates[index - 4].energy..10 {
                    print!(" ");
                }
                print!("]  ");
            }
        }
        println!();
    }
}
