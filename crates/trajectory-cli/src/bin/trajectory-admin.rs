use anyhow::{Context, Result};
use serde_json::json;
use std::path::PathBuf;
use trajectory_cli::{load_client_registry, save_client_registry};

fn main() {
    if let Err(error) = run() {
        eprintln!("{error:#}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let Some(command) = args.next() else {
        print_usage();
        return Ok(());
    };

    match command.as_str() {
        "--help" | "-h" | "help" => {
            print_usage();
            Ok(())
        }
        "create-client" => create_client(args.collect()),
        "list-clients" => list_clients(args.collect()),
        "enable-client" => set_client_enabled(args.collect(), true),
        "disable-client" => set_client_enabled(args.collect(), false),
        "delete-client" => delete_client(args.collect()),
        other => anyhow::bail!("unknown command: {other}"),
    }
}

fn create_client(args: Vec<String>) -> Result<()> {
    let mut client_db = None::<PathBuf>;
    let mut label = None::<String>;
    let mut format = OutputFormat::Text;

    let mut args = args.into_iter();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--client-db" => {
                client_db = Some(PathBuf::from(
                    args.next().context("missing client db path")?,
                ))
            }
            "--label" => label = Some(args.next().context("missing client label")?),
            "--format" => {
                format = OutputFormat::parse(&args.next().context("missing format")?)?;
            }
            "--help" | "-h" => {
                print_create_usage();
                return Ok(());
            }
            other => anyhow::bail!("unknown create-client argument: {other}"),
        }
    }

    let client_db = client_db.context("missing required --client-db")?;
    let label = label.context("missing required --label")?;

    let mut registry = load_client_registry(&client_db)?;
    let record = registry.create_key(label)?;
    save_client_registry(&client_db, &registry)?;
    let access_key = record.access_key_string()?;

    match format {
        OutputFormat::Text => {
            println!("created client");
            println!("registry={}", client_db.display());
            println!("id={:08x}", record.id);
            println!("label={}", record.label);
            println!("enabled={}", record.enabled);
            println!("access_key={access_key}");
        }
        OutputFormat::Json => {
            println!(
                "{}",
                serde_json::to_string_pretty(&json!({
                    "registry": client_db.display().to_string(),
                    "id_hex": format!("{:08x}", record.id),
                    "label": record.label,
                    "enabled": record.enabled,
                    "access_key": access_key,
                }))?
            );
        }
        OutputFormat::Key => println!("{access_key}"),
    }

    Ok(())
}

fn list_clients(args: Vec<String>) -> Result<()> {
    let mut client_db = None::<PathBuf>;
    let mut format = OutputFormat::Text;

    let mut args = args.into_iter();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--client-db" => {
                client_db = Some(PathBuf::from(
                    args.next().context("missing client db path")?,
                ))
            }
            "--format" => {
                format = OutputFormat::parse(&args.next().context("missing format")?)?;
            }
            "--help" | "-h" => {
                print_list_usage();
                return Ok(());
            }
            other => anyhow::bail!("unknown list-clients argument: {other}"),
        }
    }

    let client_db = client_db.context("missing required --client-db")?;
    let registry = load_client_registry(&client_db)?;
    match format {
        OutputFormat::Text => {
            if registry.keys.is_empty() {
                println!("no clients");
                return Ok(());
            }
            for entry in registry.keys {
                println!(
                    "{:08x}\tenabled={}\tcreated={}\tlabel={}",
                    entry.id, entry.enabled, entry.created_unix, entry.label
                );
            }
        }
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&registry.keys)?);
        }
        OutputFormat::Key => anyhow::bail!("--format key is only supported for create-client"),
    }
    Ok(())
}

fn set_client_enabled(args: Vec<String>, enabled: bool) -> Result<()> {
    let (client_db, client_id) = parse_id_command_args(
        args,
        if enabled {
            "enable-client"
        } else {
            "disable-client"
        },
    )?;
    let mut registry = load_client_registry(&client_db)?;
    let entry = registry
        .keys
        .iter_mut()
        .find(|entry| entry.id == client_id)
        .with_context(|| format!("client {:08x} not found", client_id))?;
    entry.enabled = enabled;
    let label = entry.label.clone();
    save_client_registry(&client_db, &registry)?;
    println!(
        "{} {:08x} ({label})",
        if enabled { "enabled" } else { "disabled" },
        client_id
    );
    Ok(())
}

fn delete_client(args: Vec<String>) -> Result<()> {
    let (client_db, client_id) = parse_id_command_args(args, "delete-client")?;
    let mut registry = load_client_registry(&client_db)?;
    let removed = registry
        .remove_key_by_id(client_id)
        .with_context(|| format!("client {:08x} not found", client_id))?;
    save_client_registry(&client_db, &registry)?;
    println!("deleted {:08x} ({})", removed.id, removed.label);
    Ok(())
}

fn parse_id_command_args(args: Vec<String>, command: &str) -> Result<(PathBuf, u32)> {
    let mut client_db = None::<PathBuf>;
    let mut client_id = None::<u32>;

    let mut args = args.into_iter();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--client-db" => {
                client_db = Some(PathBuf::from(
                    args.next().context("missing client db path")?,
                ))
            }
            "--id" => {
                client_id = Some(parse_client_id(&args.next().context("missing client id")?)?)
            }
            "--help" | "-h" => {
                print_id_usage(command);
                std::process::exit(0);
            }
            other => anyhow::bail!("unknown {command} argument: {other}"),
        }
    }

    Ok((
        client_db.context("missing required --client-db")?,
        client_id.context("missing required --id")?,
    ))
}

fn parse_client_id(value: &str) -> Result<u32> {
    let trimmed = value.trim();
    if let Some(hex) = trimmed.strip_prefix("0x") {
        return u32::from_str_radix(hex, 16).context("invalid hex client id");
    }
    if trimmed.chars().all(|ch| ch.is_ascii_hexdigit())
        && trimmed.chars().any(|ch| ch.is_ascii_alphabetic())
    {
        return u32::from_str_radix(trimmed, 16).context("invalid hex client id");
    }
    trimmed.parse().context("invalid client id")
}

#[derive(Clone, Copy)]
enum OutputFormat {
    Text,
    Json,
    Key,
}

impl OutputFormat {
    fn parse(value: &str) -> Result<Self> {
        match value {
            "text" => Ok(Self::Text),
            "json" => Ok(Self::Json),
            "key" => Ok(Self::Key),
            other => anyhow::bail!("unknown output format: {other}"),
        }
    }
}

fn print_usage() {
    println!(
        "\
Usage: trajectory-admin <command> [options]

Commands:
  create-client    Create and print a client access key
  list-clients     List stored clients
  enable-client    Enable a stored client by id
  disable-client   Disable a stored client by id
  delete-client    Delete a stored client by id
  help             Show this help

Examples:
  trajectory-admin create-client --client-db trajectory-clients.json --label phone
  trajectory-admin list-clients --client-db trajectory-clients.json
  trajectory-admin disable-client --client-db trajectory-clients.json --id 0123abcd"
    );
}

fn print_create_usage() {
    println!(
        "\
Usage: trajectory-admin create-client --client-db <PATH> --label <LABEL> [--format text|json|key]"
    );
}

fn print_list_usage() {
    println!(
        "\
Usage: trajectory-admin list-clients --client-db <PATH> [--format text|json]"
    );
}

fn print_id_usage(command: &str) {
    println!("Usage: trajectory-admin {command} --client-db <PATH> --id <CLIENT_ID>");
}
