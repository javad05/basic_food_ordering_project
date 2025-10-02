use crate::models::{Admin, Order, Owner, Restaurant, User};

use serde_json::{self, Value};
use std::{io, path::Path};

pub fn get_data() -> (
    Vec<Order>,
    Vec<Restaurant>,
    Vec<User>,
    Vec<Owner>,
    Vec<Admin>,
) {
    let file_path = Path::new("src/data/orders.json");
    let json_str = std::fs::read_to_string(file_path).expect("Unable to read file");
    let orders: Vec<Order> = serde_json::from_str(&json_str).expect("Failed to parse JSON");

    let file_path = Path::new("src/data/restaurants.json");
    let json_str = std::fs::read_to_string(file_path).expect("Unable to read file");
    let restaurants: Vec<Restaurant> =
        serde_json::from_str(&json_str).expect("Failed to parse JSON");

    let file_path = Path::new("src/data/accounts.json");
    let json_str = std::fs::read_to_string(file_path).expect("Unable to read file");
    let raw_entries: Vec<Value> = serde_json::from_str(&json_str).expect("Failed to parse JSON");

    let mut users: Vec<User> = Vec::new();
    let mut owners: Vec<Owner> = Vec::new();
    let mut admin: Vec<Admin> = Vec::new();

    for entry in raw_entries {
        let role = entry.get("role").and_then(Value::as_str).unwrap();
        match role {
            "user" => {
                users.push(serde_json::from_value::<User>(entry).expect("Failed to parse user"));
            }
            "owner" => {
                owners.push(
                    self::serde_json::from_value::<Owner>(entry).expect("Failed to parse owner"),
                );
            }
            "admin" => {
                admin.push(
                    self::serde_json::from_value::<Admin>(entry).expect("Failed to parse admin"),
                );
            }
            _ => println!("Unknown role: {}", role),
        }
    }

    (orders, restaurants, users, owners, admin)
}

pub fn update_data(
    orders: Vec<Order>,
    restaurants: Vec<Restaurant>,
    users: Vec<User>,
    owners: Vec<Owner>,
    admin: Vec<Admin>,
) {
    let orders_json = serde_json::to_string_pretty(&orders).expect("Failed to serialize orders");
    std::fs::write("src/data/orders.json", orders_json).expect("Unable to write file");

    let restaurants_json =
        serde_json::to_string_pretty(&restaurants).expect("Failed to serialize restaurants");
    std::fs::write("src/data/restaurants.json", restaurants_json).expect("Unable to write file");

    let mut accounts: Vec<Value> = Vec::new();
    for user in users {
        accounts.push(serde_json::to_value(user).expect("Failed to serialize user"));
    }
    for owner in owners {
        accounts.push(serde_json::to_value(owner).expect("Failed to serialize owner"));
    }
    for admin in admin {
        accounts.push(serde_json::to_value(admin).expect("Failed to serialize admin"));
    }

    let accounts_json =
        serde_json::to_string_pretty(&accounts).expect("Failed to serialize accounts");
    std::fs::write("src/data/accounts.json", accounts_json).expect("Unable to write file");
}

pub fn read_string() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
pub fn read_i32() -> Option<i32> {
    match read_string().parse::<i32>() {
        Ok(n) => return Some(n),
        Err(_) => println!("{}", "It's not a number!"),
    }
    None
}
