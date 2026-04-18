cargo publish# ⚡ hop

> Blazing fast SSH host manager for developers

```
hop add prod -H 192.168.1.1 -u root
hop prod
```

## Install

```bash
cargo install hop-ssh
```

## Commands

| Command | Description |
|---|---|
| `hop add <name> -H <host> -u <user> -p <port>` | Add a host |
| `hop list` | List all hosts |
| `hop <name>` | Quick connect |
| `hop connect <name>` | Connect to host |
| `hop remove <name>` | Remove a host |
| `hop edit <name>` | Edit a host |
| `hop rename <old> <new>` | Rename a host |
| `hop copy <src> <dest>` | Duplicate a host |
| `hop ping <name>` | Test connection |
| `hop info <name>` | Show host details |
| `hop tag <name> <tag>` | Tag a host |
| `hop search <query>` | Search hosts |
| `hop group add <group> <host>` | Add to group |
| `hop group list` | List groups |
| `hop export` | Export config |
| `hop import <file>` | Import config |
| `hop tunnel <name> <local:remote>` | Port forwarding |
| `hop run <name> "<cmd>"` | Run remote command |
| `hop sync <name> <file>` | SCP file to host |

## License
MIT
