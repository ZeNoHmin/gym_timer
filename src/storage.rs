use crate::timer::start_timer;
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write}; // import timer function
use tokio::time::{sleep, Duration};

const FILE_PATH: &str = "timers.json"; //Timers

#[derive(Serialize, Deserialize, Debug)]
pub struct Timer {
    pub name: String,
    pub duration: u32,
    pub pause: u32,
    pub repetitions: u32,
}
pub async fn start_timer_from_storage(name: &str) {
    let timers = load_timers();
    //load timer from list

    //search for a matching timer
    if let Some(timer) = timers.iter().find(|t| t.name == name) {
        println!(
            "Start timer {}, {}s training, {}s pause, {} repetitions", // display timer
            // informations
            timer.name,
            timer.duration,
            timer.pause,
            timer.repetitions
        );

        for i in 1..=timer.repetitions {
            println!("Round {}/{}", i, timer.repetitions); //display the amount of reps
            start_timer(&timer.name, timer.duration).await;

            if i < timer.repetitions {
                println!("Pause {}s", timer.pause); //Pause timer
                for j in (1..=timer.pause).rev() {
                    println!("Remaining time:{}", j);
                    sleep(Duration::from_secs(1)).await;
                }
                //timer in
                //u64 as only positive timers are allowed
            }
        }
        println!("Timer {} finished", timer.name);
    } else {
        println!("Timer {} not found", name);
    }
}
pub fn save_timers(timers: &Vec<Timer>) {
    //load timer file write and save it
    let json = serde_json::to_string_pretty(timers).unwrap();
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(FILE_PATH)
        .unwrap();
    file.write_all(json.as_bytes()).unwrap();
}

pub fn load_timers() -> Vec<Timer> {
    let mut file = match File::open(FILE_PATH) {
        Ok(file) => file,
        Err(_) => return Vec::new(), // Leere Liste bei Err
    };
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    serde_json::from_str(&data).unwrap_or_else(|_| Vec::new()) //Leere Liste falls Err
}

//Timer hinzufügen und speichern
pub fn add_timer(name: String, duration: u32, pause: u32, repetitions: u32) {
    let mut timers = load_timers(); //Lade Timer
    timers.push(Timer {
        name,
        duration,
        pause,
        repetitions,
    }); //Timer hinzufügen
    save_timers(&timers); //Neue Liste Speichern
    println!("New timer saved successfully!");
}

//Timer-Liste anzeigen
pub fn list_timers() {
    let timers = load_timers();
    if timers.is_empty() {
        println!("No Timers found.");
    } else {
        println!("Saved Timers:");
        for timer in &timers {
            println!(
                "{}: {}s, Pause:{}s Repetitions:{}",
                timer.name, timer.duration, timer.pause, timer.repetitions
            );
        }
    }
}

//Timer löschen
pub fn remove_timer(name: &str) {
    let mut timers = load_timers();

    //Timer suchen
    if !timers.iter().any(|t| t.name == name) {
        println!("Timer {} was not found!", name);
        return;
    }

    //Bestätigung
    print!("Should the timer {} really be deleted? (y/n)", name);
    io::stdout().flush().unwrap(); //Direktes zeigen

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim().to_lowercase();

    if input == "y" {
        timers.retain(|t| t.name != name);
        save_timers(&timers);
        println!("Timer {} has been deleted!", name);
    } else {
        println!("Timer {} has not been deleted!", name);
    }
}
