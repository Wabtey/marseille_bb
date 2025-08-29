use crate::{activities::Activity, print_status, roommates::Roommate};
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::io::{self, Write};

pub struct Request<'a> {
    pub roommate: &'a Roommate,
    pub requested_activity: &'a Activity,
}

// Browse through requests using arrow keys while keeping the HUD visible
pub fn browse_requests(
    requests: &[Request<'_>],
    roommates: &[Roommate],
    day: u8,
) -> io::Result<()> {
    if requests.is_empty() {
        println!("No requests available.");
        return Ok(());
    }

    let mut current_index = 0;

    // Enable raw mode to capture keypresses
    enable_raw_mode()?;

    loop {
        // Clear screen and redisplay the status HUD
        print!("\x1B[2J\x1B[1;1H");
        // print_status(roommates.to_vec(), day);

        // Display request browser interface below the HUD
        println!("\n{:-^90}", " Request Browser ");
        println!("Use left (←) and right (→) arrows to navigate. Press q to quit.");
        println!("\nRequest {}/{}", current_index + 1, requests.len());
        println!("{}\n", requests[current_index].display());
        println!("Press q to return to the main menu");

        io::stdout().flush()?;

        // Wait for a key press
        if let Event::Key(key_event) = event::read()? {
            match key_event.code {
                KeyCode::Left => {
                    current_index = current_index.saturating_sub(1);
                }
                KeyCode::Right => {
                    if current_index < requests.len() - 1 {
                        current_index += 1;
                    }
                }
                KeyCode::Char('q') => break,
                _ => {}
            }
        }
    }

    // Disable raw mode before returning
    disable_raw_mode()?;
    Ok(())
}

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
    vec![
        Request {
            requested_activity: &activities[0],
            roommate: &roommates[4],
        },
        Request {
            requested_activity: &activities[1],
            roommate: &roommates[6],
        },
    ]
}
