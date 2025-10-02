use crate::{
    handlers::read_i32,
    models::{Admin, Order, Owner, User},
};

pub fn calculate_price(orders: &Order, profit: f64) -> (f64, f64, f64) {
    let (mut total_p, profit_p, final_p);
    total_p = 0.0;
    for order in &orders.restaurants {
        total_p += order.price;
    }
    profit_p = total_p * profit;
    final_p = total_p + profit_p;
    (total_p, profit_p, final_p)
}

pub fn show_all_user_orders(orders: &Vec<Order>) {
    for (i, order) in orders.iter().enumerate() {
        for rest in &order.restaurants {
            println!("{}. Restaurant: {}", i + 1, rest.restaurant);
            for item in &rest.items {
                println!("Item: {}, Quantity: {}", item.name, item.quantity);
            }
            println!("Total Price for this restaurant: {}", rest.price);
        }
        println!("total price {}", order.total_price);
        println!("Status: {}", order.status);
        println!("Date and Time: {}", order.datetime);
        println!("-----------------------------------");
    }
}

pub fn add_mony_to_wallet(users: &mut Vec<User>, username: &str) {
    for user in users {
        if user.username == username {
            println!("{}", "How much money do you want to add to your wallet?");
            let inp = read_i32();
            if inp.is_none() {
                return;
            }
            let inp = inp.unwrap();
            if inp < 0 {
                println!("You cannot add a negative amount to your wallet.");
                return;
            }
            user.wallet += inp as f64;
            println!("Added {} to wallet. New balance: {}", inp, user.wallet);
        }
    }
}

pub fn can_pay(users: &mut Vec<User>, username: &str, f_price: f64) -> bool {
    for user in users {
        if user.username == username {
            if user.wallet < f_price as f64 {
                println!("You do not have enough money in your wallet to pay for this order.");
                return false;
            } else {
                user.wallet -= f_price as f64;
                println!("Payment successful! New wallet balance: {}", user.wallet);
                return true;
            }
        }
    }
    false
}

pub fn pay(
    users: &mut Vec<User>,
    owners: &mut Vec<Owner>,
    username: &str,
    order: &Order,
    admin: &mut Admin,
    t_price: f64,
) {
    for user in users {
        if user.username == username {
            user.wallet -= order.total_price;
        }
    }
    admin.wallet += order.total_price - t_price;
    'a: for rest in order.restaurants.iter() {
        for onwer in owners.iter_mut() {
            for owner_rest in onwer.restaurant.iter_mut() {
                if owner_rest == &rest.restaurant {
                    onwer.wallet += rest.price;
                    continue 'a;
                }
            }
        }
    }
}
