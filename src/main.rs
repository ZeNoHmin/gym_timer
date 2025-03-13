mod cli; //cli added
mod storage; //storage added
mod timer; //timer.rs added

use clap::Parser;
use cli::{Cli, Commands};
use storage::add_timer;
use storage::list_timers;
use storage::remove_timer;
use storage::start_timer_from_storage;
use Commands::*;

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    match args.command {
        Start { name } => {
            start_timer_from_storage(&name).await;
        }

        Add {
            //add function
            name,
            duration,
            pause,
            repetitions,
        } => {
            add_timer(name, duration, pause, repetitions);
        }
        List => {
            //List Timers
            list_timers();
        }
        Remove { name } => {
            //Remove Timers
            remove_timer(&name);
        }
    }
}
