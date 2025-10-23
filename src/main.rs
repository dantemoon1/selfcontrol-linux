use clap::{Parser, Subcommand};

//selfcontrol-linux - block distracting websites temporarily
#[derive(Parser)]
#[command(name = "selfcontrol")]
#[command(about = "Block websites for a set amount of time", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Block listed sites for N minutes
    Block {
        #[arg(default_values_t = &["youtube.com".to_string(), "reddit.com".to_string(), "x.com".to_string()])]
        sites: Vec<String>,
        ///list of sites to block
        #[arg(short, long)]
        for_minutes: u64,
    },
    Status,
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Block { sites, for_minutes } => {
            println!("Blocking sites: {:?}", sites);
            println!("For {} minutes", for_minutes);
        }
        Commands::Status => {
            println!("Status command");
        }
    }
}
