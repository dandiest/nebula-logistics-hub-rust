use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::io;
use std::process::exit;
use std::{fs, vec};

#[derive(Serialize, Deserialize, Debug)]
enum ShipStatus {
    Active,
    InMaintenance,
    Docked,
}

#[derive(Serialize, Deserialize, Debug)]
struct Cargo {
    material: String,
    weight: u32,
    timestamp: DateTime<Utc>,
}
#[derive(Serialize, Deserialize, Debug)]
struct Spaceship {
    id: u32,
    name: String,
    status: ShipStatus,
    inventory: Vec<Cargo>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Fleet {
    ships: Vec<Spaceship>,
}

fn inputs() -> String {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("An error occurred");
    buffer.trim().to_string()
}

impl Fleet {
    fn list_ships(&self) {
        for ship in &self.ships {
            println!(
                "Ship name: {}, Ship ID: {}, Ship status: {:?}, Ship inventory: {:?}",
                ship.name, ship.id, ship.status, ship.inventory
            );
            for item in &ship.inventory {
                let m = &item.material;
                let w = item.weight;
                let t = item.timestamp.format("%d/%m/%Y %H:%M").to_string();

                println!(
                    "==>   SHIP INVENTORY   <==\nMaterial: {}.\nWeight: {}.\nTimestamp: {}.\n",
                    m, w, t,
                );
            }
        }
    }
    fn add_ships(&mut self) {
        println!("What`s your new ship ID?");
        let new_id_string = inputs();
        let new_id_num = new_id_string
            .parse()
            .expect("An error occurred during new id parsing.");
        println!("What`s your new ship name?");
        let new_name = inputs();
        println!("What`s your new ship status?\n1. Active.\n2. In maintenance.\n3. Docked");
        let new_status = inputs();
        let status = match new_status.trim() {
            "1" => ShipStatus::Active,
            "2" => ShipStatus::InMaintenance,
            "3" => ShipStatus::Docked,
            _ => {
                println!("Invalid input detected. Defaulting 'Docked' status.");
                ShipStatus::Docked
            }
        };
        println!("Initial material?");
        let mat = inputs();

        println!("Initial weight?");
        let weight: u32 = inputs()
            .parse()
            .expect("An error occurred during weight parsing.");

        let first_cargo = Cargo {
            material: mat.to_string(),
            weight: weight,
            timestamp: Utc::now(),
        };

        let new_ship = Spaceship {
            id: new_id_num,
            name: new_name.to_string(),
            status,
            inventory: vec![first_cargo],
        };

        if self.ships.iter().any(|s| s.id == new_id_num) {
            println!("Error. There is alredy a ship with this ID.");
            return;
        } else {
            self.ships.push(new_ship);
            println!("Successfully saved data on port's servers.");
        }
    }
    fn remove_ships(&mut self, target_id: u32) {
        let initial_len = self.ships.len();
        self.ships.retain(|ship| ship.id != target_id);

        if self.ships.len() < initial_len {
            println!("Ship with ID {} successfully removed.", target_id);
        } else {
            println!("No ships found with ID {}", target_id);
        }
    }
    fn cargo_to_ship(&mut self, ship_name: &str) {
        println!("List your ship inventory.");
        println!("Material?");
        let mat_inp = inputs();
        println!("Weight?");
        let weight_input = inputs()
            .parse()
            .expect("An error occurred during weight parsing.");
        println!("Thank you. Data is being saved on the port's servers.");

        let new_cargo = Cargo {
            material: mat_inp.to_string(),
            weight: weight_input,
            timestamp: Utc::now(),
        };

        let target_ship = self.ships.iter_mut().find(|s| s.name == ship_name);

        match target_ship {
            Some(ship) => {
                ship.inventory.push(new_cargo);
                println!("Successfully loaded cargo onto {}.", ship.name);
            }
            None => {
                println!("Error: No ship found with the name '{}'.", ship_name);
            }
        }
    }
    fn fleet_stats(&self) {
        let mut total_weight: u32 = 0;
        for ship in &self.ships {
            for item in &ship.inventory {
                total_weight += item.weight;
            }
        }
        println!(
            "Total ships: {}\n Total weight: {}.",
            self.ships.len(),
            total_weight
        );
    }
    fn exit_and_save(&mut self, filepath: &str) {
        println!("Goodbye, Captain.");
        let json =
            serde_json::to_string_pretty(self).expect("An error occurred during .JSON saving.");
        fs::write(filepath, json).expect("An error occurred during .JSON writing.");
        println!("Successfully saved your ship details.");
        println!("Type 123 to quit.");
        exit(123);
    }
}

fn main() {
    let mut fleet: Fleet = match fs::read_to_string("fleet.json") {
        Ok(data) => {
            let fleet_charged: Fleet =
                serde_json::from_str(&data).unwrap_or(Fleet { ships: Vec::new() });
            fleet_charged
        }
        Err(_) => Fleet { ships: Vec::new() },
    };
    loop {
        println!(
            "Welcome, Captain. Please, choose an option:\n1. List all ships.\n2. Add a new ship.\n3. Add cargo to a ship.\n4. Remove a ship.\n5. View fleet statistics\n6. Exit and save.\n"
        );
        let choice_input = match inputs().parse::<u32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Please, insert a valid number.");
                continue;
            }
        };

        match choice_input {
            1 => fleet.list_ships(),
            2 => fleet.add_ships(),
            3 => {
                println!("What's your ship name?");
                let shipname = inputs();
                fleet.cargo_to_ship(&shipname);
            }
            4 => {
                println!("What ship do you want to remove?\nPlease insert the ID.");
                let id_input = inputs()
                    .parse()
                    .expect("An error occurred during ID parsing.");
                fleet.remove_ships(id_input);
            }
            5 => fleet.fleet_stats(),
            6 => fleet.exit_and_save("fleet.json"),
            _ => println!("Option not available."),
        }
    }
}
