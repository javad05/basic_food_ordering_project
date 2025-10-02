use crate::{
    handlers::read_string,
    models::{Admin, Item, Owner, Restaurant, User},
};

pub fn sign_in(
    users: &Vec<User>,
    owners: &Vec<Owner>,
    admin: &Vec<Admin>,
) -> Option<(String, String)> {
    println!("{}", "Enter your username");
    let username = read_string();

    println!("{}", "Enter your password");
    let password = read_string();

    for user in users {
        if user.username == username && user.password == password {
            println!("{}", "sign in successfully!");
            return Some(("user".to_string(), username));
        }
    }
    for owner in owners {
        if owner.username == username && owner.password == password {
            println!("{}", "sign in successfully!");
            return Some(("owner".to_string(), username));
        }
    }
    if admin[0].username == username && admin[0].password == password {
        println!("{}", "sign in successfully!");
        return Some(("admin".to_string(), username));
    }
    println!("{}", "username or password is wrong!");
    return None;
}
pub fn sign_up(users: &mut Vec<User>, owners: &mut Vec<Owner>) -> Option<(String, String)> {
    println!("{}", "Enter your username");
    let username = read_string();

    println!("{}", "Enter your password");
    let password = read_string();

    println!("{}", "Enter your role");
    let role = read_string();

    for user in users.iter().clone() {
        if user.username == username {
            println!("{}", "username already exists!");
            return None;
        }
    }

    match role.as_str() {
        "user" => {
            let user = User {
                username: username.clone(),
                password,
                role,
                wallet: 0.0,
            };
            users.push(user);
            println!("{}", "sign up successfully!");
            return Some(("user".to_string(), username));
        }
        "owner" => {
            let owner = Owner {
                username: username.clone(),
                password,
                role,
                wallet: 0.0,
                restaurant: Vec::new(),
            };
            println!("{}", "sign up successfully!");
            owners.push(owner);
            return Some(("owner".to_string(), username));
        }
        _ => {
            println!("{}", "role is wrong!");
            None
        }
    }
}
pub fn show_restaurants(restaurants: &Vec<Restaurant>) {
    if restaurants.is_empty() {
        println!("{}", "No restaurants available.");
    } else {
        for (i, restaurant) in restaurants.iter().enumerate() {
            println!(
                "{}. Name: {}, Category: {}",
                i + 1,
                restaurant.name,
                restaurant.category
            );
        }
    }
}
pub fn show_restaurant_menu(restaurant: &Restaurant) {
    for (i, item) in restaurant.menu.iter().enumerate() {
        println!(
            "{}. Name: {}, Price: {}, Description: {}",
            i + 1,
            item.name,
            item.price,
            item.description
        );
    }
}

pub fn show_searched_restaurants(
    restaurants: &Vec<Restaurant>,
    search_term: &str,
) -> Vec<Restaurant> {
    let mut rests = Vec::new();
    for (i, restaurant) in restaurants.iter().enumerate() {
        if restaurant
            .name
            .to_lowercase()
            .contains(&search_term.to_lowercase())
        {
            let mut rest = Restaurant {
                name: restaurant.name.clone(),
                owner: restaurant.owner.clone(),
                category: restaurant.category.clone(),
                menu: Vec::new(),
            };
            for item in &restaurant.menu {
                rest.menu.push(Item {
                    name: item.name.clone(),
                    price: item.price,
                    description: item.description.clone(),
                });
            }
            rests.push(rest);
            println!(
                "{}. Name: {}, Category: {}",
                i + 1,
                restaurant.name,
                restaurant.category
            );
        }
    }
    rests
}

pub fn show_all_items(restaurants: &Vec<Restaurant>) {
    if restaurants.is_empty() {
        println!("{}", "No restaurants available.");
    } else {
        for (i, rest) in restaurants.iter().enumerate() {
            println!("{}", "---------------------------------");
            for (j, item) in rest.menu.iter().enumerate() {
                println!(
                    "{}.{}. Restaurant: {}, Item: {}, Price: {}, Description: {}",
                    i + 1,
                    j + 1,
                    rest.name,
                    item.name,
                    item.price,
                    item.description
                );
            }
        }
    }
}
pub fn search_items(restaurants: &Vec<Restaurant>, search_term: &str) -> (Vec<Item>, Vec<String>) {
    let mut items = Vec::new();
    let mut restaurants_n = Vec::new();
    for rest in restaurants {
        for item in &rest.menu {
            if item
                .name
                .to_lowercase()
                .contains(&search_term.to_lowercase())
            {
                items.push(Item {
                    name: item.name.clone(),
                    price: item.price,
                    description: item.description.clone(),
                });
                restaurants_n.push(rest.name.clone());
            }
        }
    }
    (items, restaurants_n)
}
