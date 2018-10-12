extern crate partition_identity;

use partition_identity::{PartitionID, PartitionIDVariant};
use self::PartitionIDVariant::*;
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
                    println!("ID: {:?}", PartitionID::by_id(ID, &device).map(|id| id.id));
                    println!("Label: {:?}", PartitionID::by_id(Label, &device).map(|id| id.id));
                    println!("PartLabel: {:?}", PartitionID::by_id(PartLabel, &device).map(|id| id.id));
                    println!("PartUUID: {:?}", PartitionID::by_id(PartUUID, &device).map(|id| id.id));
                    println!("Path: {:?}", PartitionID::by_id(Path, &device).map(|id| id.id));
                    println!("UUID: {:?}", PartitionID::by_id(UUID, &device).map(|id| id.id));
                }
            }
            "by-uuid" => {
                for id in args {
                    let var = PartitionID::new(UUID, id.clone());
                    println!("{}: {:?}", id, var.from_id());
                }
            }
            "by-partuuid" => {
                for id in args {
                    let var = PartitionID::new(PartUUID, id.clone());
                    println!("{}: {:?}", id, var.from_id());
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