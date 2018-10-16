extern crate partition_identity;

use partition_identity::{PartitionID, PartitionSource};
use self::PartitionSource::*;
use std::env;
use std::process::exit;

fn main() {
    let mut args = env::args().skip(1);
    match args.next() {
        Some(arg) => match arg.as_str() {
            "from-path" => {
                let mut first = true;
                for device in args {
                    if ! first { println!() }
                    first = false;
                    println!("{}:", device);
                    println!("ID: {:?}", PartitionID::get_source(ID, &device).map(|id| id.id));
                    println!("Label: {:?}", PartitionID::get_source(Label, &device).map(|id| id.id));
                    println!("PartLabel: {:?}", PartitionID::get_source(PartLabel, &device).map(|id| id.id));
                    println!("PartUUID: {:?}", PartitionID::get_source(PartUUID, &device).map(|id| id.id));
                    println!("Path: {:?}", PartitionID::get_source(Path, &device).map(|id| id.id));
                    println!("UUID: {:?}", PartitionID::get_source(UUID, &device).map(|id| id.id));
                }
            }
            "by-uuid" => {
                for id in args {
                    let var = PartitionID::new_uuid(id.clone());
                    println!("{}: {:?}", id, var.get_device_path());
                }
            }
            "by-partuuid" => {
                for id in args {
                    let var = PartitionID::new_partuuid(id.clone());
                    println!("{}: {:?}", id, var.get_device_path());
                }
            }
            _ => {
                eprintln!("invalid subcommand: valid commansd: [from-path, by-uuid, by-partuuid, ]");
                exit(1);
            }
        }
        None => {
            eprintln!("must give subcommand: [from-path, by-uuid, by-partuuid, ]");
            exit(1);
        }
    }
}