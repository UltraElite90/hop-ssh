use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "hop", about = "⚡ Blazing fast SSH host manager", version)]
pub struct Cli {
    /// Quick connect: hop <name>
    pub quick_connect: Option<String>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add a new host
    Add {
        name: String,
        #[arg(short = 'H', long)]
        host: String,
        #[arg(short, long, default_value = "root")]
        user: String,
        #[arg(short, long, default_value_t = 22)]
        port: u16,
        #[arg(short, long)]
        identity: Option<String>,
        #[arg(short, long)]
        group: Option<String>,
    },
    /// List all hosts
    List {
        #[arg(short, long)]
        group: Option<String>,
    },
    /// Connect to a host
    Connect { name: String },
    /// Remove a host
    Remove { name: String },
    /// Edit a host
    Edit {
        name: String,
        #[arg(short = 'H', long)]
        host: Option<String>,
        #[arg(short, long)]
        user: Option<String>,
        #[arg(short, long)]
        port: Option<u16>,
        #[arg(short, long)]
        identity: Option<String>,
    },
    /// Rename a host
    Rename { old: String, new: String },
    /// Copy a host config
    Copy { source: String, dest: String },
    /// Ping a host (test connection)
    Ping { name: String },
    /// Show full host details
    Info { name: String },
    /// Tag a host
    Tag { name: String, tag: String },
    /// Search hosts
    Search { query: String },
    /// Group management
    Group {
        #[command(subcommand)]
        cmd: GroupCommands,
    },
    /// Export config
    Export {
        #[arg(short, long)]
        output: Option<String>,
    },
    /// Import config
    Import { file: String },
    /// Port forwarding shortcut
    Tunnel {
        name: String,
        /// format: local:remote (e.g. 8080:3000)
        ports: String,
    },
    /// Run a remote command
    Run { name: String, command: String },
    /// Quick SCP file sync
    Sync { name: String, file: String },
}

#[derive(Subcommand)]
pub enum GroupCommands {
    /// Add host to group
    Add { group: String, host: String },
    /// List groups
    List,
}
