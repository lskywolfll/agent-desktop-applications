use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "agent-app", version, about = "Desktop application automation for AI agents")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List all visible windows
    Windows,
    /// Get snapshot of a window (accessibility tree)
    Snapshot {
        /// Window handle or title
        target: String,
    },
    /// Click at coordinates
    Click { x: i32, y: i32 },
    /// Type text
    Type { text: String },
    /// Press a key
    Press { key: String },
    /// Screenshot
    Screenshot {
        /// Output path
        path: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Windows => {
            println!("Windows: (not yet implemented)");
        }
        Commands::Snapshot { target } => {
            println!("Snapshot of '{}': (not yet implemented)", target);
        }
        Commands::Click { x, y } => {
            println!("Click at ({}, {}): (not yet implemented)", x, y);
        }
        Commands::Type { text } => {
            println!("Type '{}': (not yet implemented)", text);
        }
        Commands::Press { key } => {
            println!("Press '{}': (not yet implemented)", key);
        }
        Commands::Screenshot { path } => {
            println!("Screenshot to '{}': (not yet implemented)", path);
        }
    }
}
