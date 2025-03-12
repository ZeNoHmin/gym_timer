mod cli; //cli added
mod storage;
mod timer; //timer.rs added //storage added

use clap::Parser;
use cli::{Cli, Commands};
use storage::add_timer;
use storage::list_timers;
use storage::remove_timer;

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Add {
            name,
            duration,
            pause,
            repetitions,
        } => {
            add_timer(name, duration, pause, repetitions);
        }
        Commands::List => {
            list_timers();
        }
        Commands::Remove { name } => {
            remove_timer(&name);
        }
        _ => println!("Unbekannter Befehl!"),
    }
}
