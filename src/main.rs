mod cli;
mod config;
mod error;
mod host;
mod ssh;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands, GroupCommands};
use colored::*;
use config::Config;
use host::Host;
use std::fs;

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Quick connect: hop myserver
    if let Some(name) = cli.quick_connect {
        if cli.command.is_none() {
            let config = Config::load()?;
            let host = config.get_host(&name)?;
            println!("{} {}", "Connecting to".green(), host.name.bold());
            return ssh::connect(host);
        }
    }

    let cmd = match cli.command {
        Some(c) => c,
        None => {
            // No args: show list
            return cmd_list(None);
        }
    };

    match cmd {
        Commands::Add { name, host, user, port, identity, group } => {
            let mut config = Config::load()?;
            let mut h = Host::new(&name, &host, &user, port);
            h.identity = identity;
            h.group = group;
            config.add_host(h)?;
            config.save()?;
            println!("{} {}", "✓ Added".green(), name.bold());
        }

        Commands::List { group } => cmd_list(group)?,

        Commands::Connect { name } => {
            let config = Config::load()?;
            let host = config.get_host(&name)?;
            println!("{} {}", "Connecting to".green(), host.name.bold());
            ssh::connect(host)?;
        }

        Commands::Remove { name } => {
            let mut config = Config::load()?;
            config.remove_host(&name)?;
            config.save()?;
            println!("{} {}", "✓ Removed".red(), name.bold());
        }

        Commands::Edit { name, host, user, port, identity } => {
            let mut config = Config::load()?;
            {
                let h = config.get_host_mut(&name)?;
                if let Some(v) = host     { h.hostname = v; }
                if let Some(v) = user     { h.user = v; }
                if let Some(v) = port     { h.port = v; }
                if let Some(v) = identity { h.identity = Some(v); }
            }
            config.save()?;
            println!("{} {}", "✓ Updated".green(), name.bold());
        }

        Commands::Rename { old, new } => {
            let mut config = Config::load()?;
            let mut h = config.remove_host(&old)?;
            h.name = new.clone();
            config.hosts.insert(new.clone(), h);
            config.save()?;
            println!("{} {} → {}", "✓ Renamed".green(), old.bold(), new.bold());
        }

        Commands::Copy { source, dest } => {
            let mut config = Config::load()?;
            let mut h = config.get_host(&source)?.clone();
            h.name = dest.clone();
            config.hosts.insert(dest.clone(), h);
            config.save()?;
            println!("{} {} → {}", "✓ Copied".green(), source.bold(), dest.bold());
        }

        Commands::Ping { name } => {
            let config = Config::load()?;
            let host = config.get_host(&name)?;
            print!("Pinging {}... ", host.name.bold());
            if ssh::ping(host)? {
                println!("{}", "✓ Online".green());
            } else {
                println!("{}", "✗ Unreachable".red());
            }
        }

        Commands::Info { name } => {
            let config = Config::load()?;
            let h = config.get_host(&name)?;
            println!("{}", "─────────────────────".dimmed());
            println!("  Name:     {}", h.name.bold());
            println!("  Host:     {}", h.hostname.cyan());
            println!("  User:     {}", h.user);
            println!("  Port:     {}", h.port);
            if let Some(ref id) = h.identity {
                println!("  Identity: {}", id);
            }
            if let Some(ref g) = h.group {
                println!("  Group:    {}", g.yellow());
            }
            if !h.tags.is_empty() {
                println!("  Tags:     {}", h.tags.join(", ").yellow());
            }
            println!("{}", "─────────────────────".dimmed());
        }

        Commands::Tag { name, tag } => {
            let mut config = Config::load()?;
            let h = config.get_host_mut(&name)?;
            if !h.tags.contains(&tag) {
                h.tags.push(tag.clone());
            }
            config.save()?;
            println!("{} tag '{}' to {}", "✓ Added".green(), tag.yellow(), name.bold());
        }

        Commands::Search { query } => {
            let config = Config::load()?;
            let q = query.to_lowercase();
            let results: Vec<_> = config.hosts.values().filter(|h| {
                h.name.to_lowercase().contains(&q)
                    || h.hostname.to_lowercase().contains(&q)
                    || h.tags.iter().any(|t| t.to_lowercase().contains(&q))
                    || h.group.as_deref().unwrap_or("").to_lowercase().contains(&q)
            }).collect();

            if results.is_empty() {
                println!("{}", "No hosts found.".dimmed());
            } else {
                for h in results {
                    print_host_row(h);
                }
            }
        }

        Commands::Group { cmd } => match cmd {
            GroupCommands::Add { group, host } => {
                let mut config = Config::load()?;
                let h = config.get_host_mut(&host)?;
                h.group = Some(group.clone());
                config.save()?;
                println!("{} {} → group '{}'", "✓ Assigned".green(), host.bold(), group.yellow());
            }
            GroupCommands::List => {
                let config = Config::load()?;
                let mut groups: std::collections::HashSet<String> = std::collections::HashSet::new();
                for h in config.hosts.values() {
                    if let Some(ref g) = h.group {
                        groups.insert(g.clone());
                    }
                }
                if groups.is_empty() {
                    println!("{}", "No groups defined.".dimmed());
                } else {
                    for g in groups {
                        println!("  {}", g.yellow());
                    }
                }
            }
        },

        Commands::Export { output } => {
            let config = Config::load()?;
            let content = toml::to_string_pretty(&config)?;
            match output {
                Some(path) => {
                    fs::write(&path, &content)?;
                    println!("{} to {}", "✓ Exported".green(), path.bold());
                }
                None => print!("{}", content),
            }
        }

        Commands::Import { file } => {
            let content = fs::read_to_string(&file)?;
            let imported: Config = toml::from_str(&content)?;
            let mut config = Config::load()?;
            let count = imported.hosts.len();
            for (k, v) in imported.hosts {
                config.hosts.insert(k, v);
            }
            config.save()?;
            println!("{} {} hosts", "✓ Imported".green(), count);
        }

        Commands::Tunnel { name, ports } => {
            let config = Config::load()?;
            let host = config.get_host(&name)?;
            let parts: Vec<&str> = ports.split(':').collect();
            if parts.len() != 2 {
                anyhow::bail!("Port format must be local:remote (e.g. 8080:3000)");
            }
            let local: u16 = parts[0].parse()?;
            let remote: u16 = parts[1].parse()?;
            ssh::tunnel(host, local, remote)?;
        }

        Commands::Run { name, command } => {
            let config = Config::load()?;
            let host = config.get_host(&name)?;
            ssh::run_command(host, &command)?;
        }

        Commands::Sync { name, file } => {
            let config = Config::load()?;
            let host = config.get_host(&name)?;
            ssh::sync_file(host, &file)?;
        }
    }

    Ok(())
}

fn cmd_list(group: Option<String>) -> Result<()> {
    let config = Config::load()?;
    if config.hosts.is_empty() {
        println!("{}", "No hosts yet. Use `hop add` to add one.".dimmed());
        return Ok(());
    }
    let mut hosts: Vec<&Host> = config.hosts.values().collect();
    hosts.sort_by(|a, b| a.name.cmp(&b.name));

    if let Some(ref g) = group {
        hosts.retain(|h| h.group.as_deref() == Some(g.as_str()));
    }

    println!("{}", "─────────────────────────────────────────".dimmed());
    for h in hosts {
        print_host_row(h);
    }
    println!("{}", "─────────────────────────────────────────".dimmed());
    Ok(())
}

fn print_host_row(h: &Host) {
    let group_str = h.group.as_deref()
        .map(|g| format!(" [{}]", g).yellow().to_string())
        .unwrap_or_default();
    let tags_str = if h.tags.is_empty() {
        String::new()
    } else {
        format!(" #{}", h.tags.join(" #")).dimmed().to_string()
    };
    println!(
        "  {:<20} {}@{}:{}{}{}", 
        h.name.bold().green(),
        h.user,
        h.hostname.cyan(),
        h.port,
        group_str,
        tags_str
    );
}
