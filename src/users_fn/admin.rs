use crate::models::{Order, Owner, Restaurant, User};

pub fn show_all_users(users: &Vec<User>) {
    for (i, user) in users.iter().enumerate() {
        println!(
            "{}. username: {} password: {} wallet: {}",
            i + 1,
            user.username,
            user.password,
            user.wallet
        );
    }
}
pub fn show_all_owners(owners: &Vec<Owner>) {
    for (i, owner) in owners.iter().enumerate() {
        print!(
            "{}. username: {} password: {} wallet: {} restaurants: ",
            i + 1,
            owner.username,
            owner.password,
            owner.wallet
        );
        for rest in owner.restaurant.iter() {
            print!("({}) ", rest)
        }
        println!();
    }
}
pub fn show_all_restaurants(restaurants: &Vec<Restaurant>) {
    for (i, rest) in restaurants.iter().enumerate() {
        print!(
            "{}. naem: {} owner: {} category: {} menu: ",
            i + 1,
            rest.name,
            rest.owner,
            rest.category
        );
        for item in &rest.menu {
            print!(
                "(name: {} price: {} describtion: {}) ",
                item.name, item.price, item.description
            )
        }
        println!();
    }
}

pub fn show_all_orders(orders: &Vec<Order>) {
    for (i, order) in orders.iter().enumerate() {
        println!("{}. username: {}", i + 1, order.username);
        println!("{}", "orders:");
        for (j, rest) in order.restaurants.iter().enumerate() {
            println!("{}. restaurant: {}", j + 1, rest.restaurant);
            println!("{}", "items:");
            for item in rest.items.iter() {
                println!("{} x{}", item.name, item.quantity);
            }
            println!("price: {}", rest.price);
        }
        println!("finall price: {}", order.total_price);
        println!("status: {} datetime: {}", order.status, order.datetime);
    }
}
