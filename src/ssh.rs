use crate::host::Host;
use anyhow::Result;
use std::process::Command;

pub fn connect(host: &Host) -> Result<()> {
    let mut args = vec![
        "-p".to_string(),
        host.port.to_string(),
    ];
    if let Some(ref identity) = host.identity {
        args.push("-i".to_string());
        args.push(identity.clone());
    }
    args.push(host.connection_string());

    Command::new("ssh")
        .args(&args)
        .status()?;
    Ok(())
}

pub fn ping(host: &Host) -> Result<bool> {
    let output = Command::new("ssh")
        .args([
            "-p", &host.port.to_string(),
            "-o", "ConnectTimeout=5",
            "-o", "BatchMode=yes",
            &host.connection_string(),
            "echo ok",
        ])
        .output()?;
    Ok(output.status.success())
}

pub fn run_command(host: &Host, cmd: &str) -> Result<()> {
    let mut args = vec![
        "-p".to_string(),
        host.port.to_string(),
    ];
    if let Some(ref identity) = host.identity {
        args.push("-i".to_string());
        args.push(identity.clone());
    }
    args.push(host.connection_string());
    args.push(cmd.to_string());

    Command::new("ssh").args(&args).status()?;
    Ok(())
}

pub fn tunnel(host: &Host, local: u16, remote: u16) -> Result<()> {
    let forward = format!("{}:localhost:{}", local, remote);
    let mut args = vec![
        "-p".to_string(), host.port.to_string(),
        "-L".to_string(), forward,
        "-N".to_string(),
    ];
    if let Some(ref identity) = host.identity {
        args.push("-i".to_string());
        args.push(identity.clone());
    }
    args.push(host.connection_string());

    println!("Tunnel open: localhost:{} -> {}:{}", local, host.hostname, remote);
    Command::new("ssh").args(&args).status()?;
    Ok(())
}

pub fn sync_file(host: &Host, file: &str) -> Result<()> {
    let dest = format!("{}:{}", host.connection_string(), "~/");
    Command::new("scp")
        .args(["-P", &host.port.to_string(), file, &dest])
        .status()?;
    Ok(())
}
