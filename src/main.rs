use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "Gut")]
#[command(about = "A fully featured version control system written in Rust.", long_about = None)]
struct Cli {
    /// Your name
    //#[arg(short, long)]
    //name: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Say hello
    Greet {
        /// Name to greet
        #[arg(short, long)]
        name: Option<String>,
    },

    Speak {
        #[arg(short, long)]
        message_to_speak: Option<String>,
    },

    Init,

    Status,

    Commit {
        #[arg(short, long)]
        message: Option<String>,
    },

    Branch {
        #[arg(short, long)]
        name: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    /*// Optional name flag
    if let Some(name) = &cli.name {
        println!("Hi, {}!", name);
    }*/

    // Subcommand logic
    match &cli.command {
        Some(Commands::Speak { message_to_speak }) => {
            let message_to_speak = message_to_speak.as_deref().unwrap_or("No Message Specified");
            println!("{}", message_to_speak);
        }
        Some(Commands::Init) => {
            println!("Repository initialized")
        }
        Some(Commands::Status) => {
            println!("Everything is up to date!");
        }
        Some(Commands::Commit { message }) => {
            println!("A commit has been made!");
            let message = message.as_deref().unwrap_or("No Message Specified");
            println!("Commit made with the message {}", message);
        }
        Some(Commands::Branch { name }) => {
            println!("A branch has been made");
            let name = name.as_deref().unwrap_or("No Name Specified");
            println!("A branch as been made named {}", name);
        }
        None => {}
        &Some(Commands::Greet { .. }) => todo!()
    }
    println!("Nothing was passed!"); // This line doesn't work as intended
}
