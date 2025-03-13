use clap::{Parser, Subcommand};

#[derive(Parser, Clone, Debug)]
#[command(name = "Gym Timer")]
#[command(version = "1.0")]
#[command(about = "Ein CLI-Tool für Trainings-Timer")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Clone, Debug)]
pub enum Commands {
    //Starte einen Timer
    Start {
        #[arg(short, long)]
        name: String,
    },
    //Lösche einen Timer
    Remove {
        #[arg(short, long)]
        name: String,
    },
    List,
    //Füge einen neuen Timer Hinzu
    Add {
        #[arg(short, long)]
        name: String,
        #[arg(short, long)]
        duration: u32,
        #[arg(short, long)]
        pause: u32,
        #[arg(short, long)]
        repetitions: u32,
    },
}
