use std::io::{self, Write};

use activities::init_activities;
use requests::init_requests;
use roommates::{Roommate, init};

mod activities;
mod constraints;
mod requests;
mod roommates;

/* -------------------------------------------------------------------------- */
/*                                Gameplay loop                               */
/* -------------------------------------------------------------------------- */

fn main() {
    let mut roommates = init();
    let mut days = 1;
    let activities = init_activities();

    while days <= 7 && !roommates.iter().any(|r| r.happiness == 0) {
        print_status(roommates.clone(), days);
        // TODO: feat - browse though request
        // TODO: feat - assign activities

        // show all requests
        let requests = init_requests(&roommates, &activities);
        for request in &requests {
            println!("{}", request.to_natural_language())
        }
        // // show all activities
        // for activity in &activities {
        //     println!("- {}", activity.display())
        // }

        /* ------------------------------- End of Day ------------------------------- */
        let mut input = String::new();
        print!("Press return to advance the day...");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        days += 1;

        /* ------------ Compute happiness gained/lost, energy consumed ------------ */
        roommates = init();
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
