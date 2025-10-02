use crate::models::{Item, Order, Owner, Restaurant};

pub fn show_owner_restaurants(restaurants: &Vec<Restaurant>, username: &str) -> bool {
    let mut index = 1;
    let mut have_rest = false;
    for rest in restaurants {
        if rest.owner == username {
            println!(
                "{}. name: {} category: {} count of items: {}",
                index,
                rest.name,
                rest.category,
                rest.menu.len()
            );
            have_rest = true;
            index += 1;
        }
    }
    have_rest
}
pub fn check_restaurant_name(restaurants: &Vec<Restaurant>, restaurant_name: &str) -> bool {
    for rest in restaurants {
        if rest.name == restaurant_name {
            return false;
        }
    }
    true
}
pub fn check_item_name(items: &Vec<Item>, item_name: &str) -> bool {
    for item in items {
        if item.name == item_name {
            return false;
        }
    }
    true
}
pub fn show_all_owner_order(orders: &Vec<Order>, is_paid: bool, owner: &Owner) {
    let paid = if is_paid {
        "paid".to_string()
    } else {
        "unpaid".to_string()
    };
    for rest in owner.restaurant.iter() {
        println!("{}:", rest);
        let mut have_order = false;
        for order in orders.iter() {
            if order.status == paid {
                for o in &order.restaurants {
                    if &o.restaurant == rest {
                        println!("user: {}", order.username);
                        for it in &o.items {
                            println!("{}", it.name);
                            println!("quantity: {}", it.quantity);
                        }
                        println!("price: {}", o.price);
                        have_order = true;
                    }
                }
            }
        }
        if !have_order {
            println!("{}", "have not any order");
        }
        println!("your wallet: {}", owner.wallet);
    }
}

pub fn find_owner(owners: &Vec<Owner>, name: &str) -> Option<Owner> {
    for owner in owners {
        if owner.username == name {
            return Some(Owner {
                username: owner.username.clone(),
                password: owner.password.clone(),
                role: "owner".to_string(),
                wallet: owner.wallet,
                restaurant: owner.restaurant.clone(),
            });
        }
    }
    return None;
}

pub fn change_owner_restaurant_name(
    owners: &mut Vec<Owner>,
    username: &str,
    index: usize,
    new_name: &str,
) {
    for owner in owners {
        if owner.username == username {
            owner.restaurant[index] = new_name.to_string();
            println!("changed to {}", new_name.to_string());
            return;
        }
    }
}
