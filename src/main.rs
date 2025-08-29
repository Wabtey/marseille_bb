use std::{
    collections::HashMap,
    io::{self, Write},
};

use activities::{init_activities, Activity};
use requests::{browse_requests, init_requests};
use roommates::{init, Roommate};

mod activities;
mod constraints;
mod requests;
mod roommates;

/* -------------------------------------------------------------------------- */
/*                                Gameplay loop                               */
/* -------------------------------------------------------------------------- */

fn main() {
    let mut roommates = init();
    let mut day = 1;
    let activities = init_activities();
    // (Activity, start hour/min, end hour/min)
    let mut assigned_activities: HashMap<(Roommate, Activity), ((u8, u8), (u8, u8))> =
        HashMap::new();

    while day <= 7 && !roommates.iter().any(|r| r.happiness == 0) {
        // print_status(roommates.clone(), day);
        // TODO: feat - browse though request

        // show all requests
        let requests = init_requests(&roommates, &activities);
        if let Err(e) = browse_requests(&requests, &roommates, day) {
            eprintln!("Error while browsing requests: {}", e);
        }

        assign_activity(
            roommates.clone(),
            activities.clone(),
            &mut assigned_activities,
        );

        /* ------------------------------- End of Day ------------------------------- */
        let mut input = String::new();
        print!("Press return to advance the day...");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        day += 1;

        /* ------------ Compute happiness gained/lost, energy consumed ------------ */
        roommates = init();
        /*
        How can I compute such gain?
        - From every actions/events chosen get their happiness factor and apply it a random factor
        - Each person implic
         */
    }

    print_status(roommates.clone(), day);

    if day == 8 {
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

pub fn print_status(roommates: Vec<Roommate>, day: u8) {
    print!("\x1B[2J\x1B[1;1H");
    println!("{:-^90}", format!("Day {}", day));
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

/// TODO: feat - assign activities
fn assign_activity(
    roommates: Vec<Roommate>,
    activities: Vec<Activity>,
    assigned_activities: &mut HashMap<(Roommate, Activity), ((u8, u8), (u8, u8))>,
) {
    println!("Assign activities to a roommate:");
    for (i, roommate) in roommates.iter().enumerate() {
        println!("{}: {}", i + 1, roommate.name);
    }

    let mut input = String::new();
    print!("Select a roommate by number: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let roommate_index: usize = input.trim().parse().unwrap();
    let selected_roommate = &roommates[roommate_index - 1];

    println!("Select an activity:");
    for (i, activity) in activities.iter().enumerate() {
        println!("{}: {}", i + 1, activity.display());
    }

    input.clear();
    print!("Select an activity by number: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let activity_index: usize = input.trim().parse().unwrap();
    let selected_activity = &activities[activity_index - 1];

    input.clear();
    print!("Enter start time (HH MM): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let mut start_time: Vec<u8> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    if start_time.len() == 1 {
        start_time.push(0)
    }

    let duration = selected_activity.duration;
    let end_hour = start_time[0] + duration.0 + (start_time[1] + duration.1) / 60;
    let end_minute = (start_time[1] + duration.1) % 60;
    let end_time = [end_hour, end_minute];

    assigned_activities.insert(
        (selected_roommate.clone(), selected_activity.clone()),
        ((start_time[0], start_time[1]), (end_time[0], end_time[1])),
    );

    println!(
        "{} has been assigned to {} from {:02}:{:02} to {:02}:{:02}",
        selected_roommate.name,
        selected_activity.display(),
        start_time[0],
        start_time[1],
        end_time[0],
        end_time[1]
    );
}
