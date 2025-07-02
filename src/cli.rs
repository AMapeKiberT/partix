use std::fs::{File, OpenOptions, read_dir};
use std::path::Path;
use gptman::{GPT};
use rustyline::DefaultEditor;
use clap::{Parser, Subcommand, ValueEnum};
use anyhow::anyhow;

#[derive(Parser)]
#[command(disable_help_flag = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Clone, Debug, ValueEnum)]
enum Unit {
    A,
    B,
    KB,
    MB,
    G,  
}

#[derive(Subcommand)]
enum Commands {
    /// Show GPT partitions
    Show,
    /// Select a specific device
    Select {
        /// Device path (e.g., /dev/sda)
        device: String,
    },
    /// Delete partition
    Delete {
        ///Number partition for delete
        index: u32,
    },
    /// Select unit type for partition sizes
    Unit {
        /// Select unit type
        #[arg(value_enum)] // Ensure clap knows it's an enum
        unit: Option<Unit>,
    },
    /// Exit/Quit from CLI
    Exit,
}

pub fn repl_loop(device: Option<String>) -> anyhow::Result<()> {
    let mut rl = DefaultEditor::new()?;
    let mut current_device = device;
    let mut current_unit = Unit::A;

    if let Some(ref dev) = current_device {
        println!("Selected device: {}\n", dev);
    } else {
        println!("No device selected. Use 'select /dev/sdX' to select one.");
    }

    loop {
        let line = rl.readline("Partix >> ");
        match line {
            Ok(input) => {
                let input = input.trim();
                if input.is_empty() {
                    continue;
                }
                rl.add_history_entry(input)?;

                // Parse input directly without intermediate vector
                let args = std::iter::once("partix").chain(input.split_whitespace());
                match Cli::try_parse_from(args) {
                    Ok(cli) => {
                        if handle_command(cli.command, &mut current_device, &mut current_unit)? {
                            break;
                        }
                    }
                    Err(e) if e.kind() == clap::error::ErrorKind::UnknownArgument => {
                        println!("Unknown command. Type 'help' for available commands.");
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                    }
                }
            }
            Err(_) => {
                println!("\nExiting.");
                break;
            }
        }
    }

    Ok(())
}

fn handle_command(command: Commands, current_device: &mut Option<String>, current_unit: &mut Unit) -> anyhow::Result<bool> {
    match command {
        Commands::Exit => Ok(true), // Returns true to indicate exit
        Commands::Show => {
            if let Some(dev) = current_device {
                show_partitions(dev, current_unit)?;
            } else {
                list_all_gpt_devices(current_unit)?;
            }
            Ok(false) // Continues the loop
        }
        Commands::Select { device } => {
            if Path::new(&device).exists() {
                println!("Selected device: {}", &device);
                *current_device = Some(device);
            } else {
                println!("Device not found: {}", device);
            }
            Ok(false) // Continues the loop
        }
        Commands::Unit { unit } => {
            if let Some(new_unit) = unit {
                *current_unit = new_unit.clone();
                println!("Unit set to {:?}", new_unit);
            } else {
                // If no unit argument is provided, display the current unit
                println!("Current unit: {:?}", current_unit);
            }
            Ok(false) // Continues the loop
        }
        Commands::Delete { index } => {
            if let Some(device) = current_device {
                delete_partition(device, index)?;
            } else {
                println!("No device selected. Use 'select /dev/sdX' to select one.");
            }
            Ok(false) // Continues the loop
        }
    }
}

fn show_partitions(device: &str, unit: &Unit) -> anyhow::Result<()> {
    let mut file = File::open(device)?;
    let gpt = GPT::find_from(&mut file)?;
    print_partition_table(&gpt, device, unit);
    Ok(())
}

fn print_partition_table(gpt: &GPT, device: &str, unit: &Unit) {
    println!("GPT detected on {}\n", device);
    println!(
        "  {:<11} {:<14} {:<14} {:<21}",
        "PART:", "START:", "END:", "SIZE:"
    );

    for (i, partition) in gpt.iter() {
        if !partition.is_used() {
            continue;
        }

        let sector_size = gpt.sector_size;
        let start_bytes = partition.starting_lba * sector_size;
        let size_lba = partition.size().unwrap_or(0);
        let size_bytes = size_lba * sector_size;
        let end_bytes = (partition.starting_lba + size_lba - 1) * sector_size;

        println!(
            "  #{:<10} {:<14} {:<14} {:<21}",
            i,
            format_size(start_bytes, unit),
            format_size(end_bytes, unit),
            format_size(size_bytes, unit)
        );
    }
    println!("")
}

fn format_size(bytes: u64, unit: &Unit) -> String {
    match unit {
        Unit::A => { ///Auto-Unit
            const GIGABYTE: f64 = 1024.0 * 1024.0 * 1024.0;
            const MEGABYTE: f64 = 1024.0 * 1024.0;
            const KILOBYTE: f64 = 1024.0;

            let bytes_f64 = bytes as f64;

            if bytes_f64 >= GIGABYTE {
                format!("{:.1} G", bytes_f64 / GIGABYTE)
            } else if bytes_f64 >= MEGABYTE {
                format!("{:.1} MB", bytes_f64 / MEGABYTE)
            } else if bytes_f64 >= KILOBYTE {
                format!("{:.1} KB", bytes_f64 / KILOBYTE)
            } else {
                format!("{} B", bytes)
            }
        }
        Unit::B => format!("{} B", bytes),
        Unit::KB => format!("{:.1} KB", bytes as f64 / 1024.0),
        Unit::MB => format!("{:.1} MB", bytes as f64 / 1024f64.powi(2)),
        Unit::G => format!("{:.1} G", bytes as f64 / 1024f64.powi(3)),
    }
}

fn list_all_gpt_devices(unit: &Unit) -> anyhow::Result<()> {
    println!("Scanning all disks for GPT...\n");

    let device_prefixes = ["/dev/sd", "/dev/nvme", "/dev/vd"];
    for entry in read_dir("/dev")?.flatten() {
        let path = entry.path();
        let path_str = path.to_string_lossy();

        if !device_prefixes.iter().any(|prefix| path_str.starts_with(prefix)) {
            continue;
        }

        if let Ok(mut f) = File::open(&path) {
            if let Ok(gpt) = GPT::find_from(&mut f) {
                print_partition_table(&gpt, &path_str, unit);
                println!();
            }
        }
    }

    Ok(())
}

fn delete_partition(device: &str, index: u32) -> anyhow::Result<()> {
    println!("Removing partition #{} from device {}", index, device);

    // Open device with read-write
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(device)?;

    // Loading GPT
    let mut gpt = GPT::find_from(&mut file)?;

    // Check if the partition exist and use
    let partition_exists_and_used = gpt.iter().any(|(i, p)| i == index && p.is_used());

    if !partition_exists_and_used {
        // If the partition does not exist or is not in use, error
        let partition_exists = gpt.iter().any(|(i, _)| i == index);
        if !partition_exists {
            return Err(anyhow!("Partition #{} not found.", index));
        } else {
            return Err(anyhow!("Partition #{} not in use or deleted.", index));
        }
    }

    // Removing partition
    gpt.remove(index).map_err(|e| anyhow!("Error in removing partition #{}: {}", index, e))?;

    // Writing changed GPT on device
    gpt.write_into(&mut file)?;

    println!("Partition #{} successfully removed from the device {}.", index, device);
    Ok(())
}

