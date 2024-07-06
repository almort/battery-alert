use std::{io, thread, time::Duration};

use battery::*;
use notify_rust::Notification;
use units::ratio::percent;

fn main() {
    get_battery().unwrap();
}

fn notification() {
    Notification::new()
        .summary("Battery low")
        .body("Battery level under 20%, consider charging.")
        .urgency(notify_rust::Urgency::Critical)
        .show().unwrap();
}

fn get_battery() -> battery::Result<()> {
    let manager = Manager::new()?;
    let mut battery = match manager.batteries()?.next() {
        Some(Ok(battery)) => battery,
        Some(Err(e)) => {
            eprintln!("Unable to access battery information!");
            return Err(e);
        }
        None => {
            eprintln!("Unable to find any batteries");
            return Err(io::Error::from(io::ErrorKind::NotFound).into());
        }
    };

    let mut wait_time = 1;

    loop {
        let percentage = battery.state_of_charge().get::<percent>();
        let threshold: f32 = 0.20;
        if percentage <= threshold {
            notification();
            wait_time = 30000
        }
        thread::sleep(Duration::from_secs(wait_time));
        manager.refresh(&mut battery)?;
    }
}
