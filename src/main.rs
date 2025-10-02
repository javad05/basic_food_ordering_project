pub mod handlers;
pub mod models;
mod users_fn {
    pub mod admin;
    pub mod general;
    pub mod owner;
    pub mod user;
}

use handlers::{get_data, read_i32, read_string, update_data};
use models::{Item, Order, OrderItem, Restaurant, SelectedItem};
use users_fn::{
    admin::{show_all_orders, show_all_owners, show_all_restaurants, show_all_users},
    general::{
        search_items, show_all_items, show_restaurant_menu, show_restaurants,
        show_searched_restaurants, sign_in, sign_up,
    },
    owner::{
        check_item_name, check_restaurant_name, find_owner, show_all_owner_order,
        show_owner_restaurants,
    },
    user::{add_mony_to_wallet, calculate_price, can_pay, pay, show_all_user_orders},
};

use crate::users_fn::owner::change_owner_restaurant_name;

fn main() {
    let (mut orders, mut restaurants, mut users, mut owners, mut admin) = get_data();
    println!("{}", "Welcome to the Food Delivery System!");
    'm: loop {
        let (username, role);
        println!("{}", "1.Sign in 2.Sign up 3.Exit");
        let input = read_i32();
        if input == None {
            continue;
        }
        let input = input.unwrap();
        match input {
            1 => {
                let result = sign_in(&users, &owners, &admin);
                match result {
                    Some(u) => {
                        (role, username) = u;
                    }
                    None => {
                        continue;
                    }
                }
            }
            2 => {
                let user_op = sign_up(&mut users, &mut owners);
                match user_op {
                    Some(u) => {
                        (role, username) = u;
                    }
                    None => {
                        continue;
                    }
                }
            }
            3 => {
                break 'm;
            }
            _ => {
                // (username, role) = ("".to_string(), "".to_string());
                println!("{}", "just 1 or 2!");
                continue;
            }
        }

        let mut u_orders = Order {
            username: username.clone(),
            restaurants: Vec::new(),
            total_price: 0.0,
            status: "pending".to_string(),
            datetime: chrono::Local::now().to_rfc3339(),
        };
        match role.to_string().as_str() {
            "user" => 'a: loop {
                println!(
                    "{}",
                    "1.show restaurants list 2.search restaurants 3.show all items 4.search items 5.show orders and pay 6.show last orders 7.add mony to wallet 8.sign out 0.exit"
                );
                let inp = read_i32();
                if inp == None {
                    continue;
                }
                match inp.unwrap() {
                    1 => {
                        println!("{}", "Select a restaurant from the list below:");
                        show_restaurants(&restaurants);
                        let inp1 = read_i32();
                        if inp1 == None {
                            continue;
                        };
                        let inp1 = inp1.unwrap() - 1;

                        u_orders.restaurants.push(OrderItem {
                            restaurant: restaurants[inp1 as usize].name.clone(),
                            items: Vec::new(),
                            price: 0.0,
                        });

                        loop {
                            println!("{}", "for order, enter the item id(0 is back):");
                            show_restaurant_menu(&(restaurants[inp1 as usize]));
                            let inp2 = read_i32();
                            if inp2 == None {
                                continue 'a;
                            }
                            let inp2 = inp2.unwrap() - 1;
                            if inp2 == -1 {
                                continue 'a;
                            }
                            if inp2 > restaurants[inp1 as usize].menu.len() as i32 {
                                println!("{}", "item not found");
                                continue;
                            }
                            println!("{}", "Enter the quantity of the item:");
                            let quantity = read_i32();
                            if quantity == None {
                                continue;
                            }
                            let quantity = quantity.unwrap();
                            if quantity <= 0 {
                                println!("{}", "quantity must be greater than 0");
                                continue;
                            }
                            let last_index = u_orders.restaurants.len() - 1;
                            u_orders.restaurants[last_index].items.push(SelectedItem {
                                name: restaurants[inp1 as usize].menu[inp2 as usize].name.clone(),
                                quantity: quantity as u32,
                            });

                            u_orders.restaurants[last_index].price +=
                                restaurants[inp1 as usize].menu[inp2 as usize].price
                                    * quantity as f64;
                            println!("{}", "item added to order");
                        }
                    }
                    2 => {
                        println!("{}", "Enter the restaurant name to search:");
                        let search_name = read_string();

                        println!("{}", "Select a restaurant from the list below:");
                        let searched_resturant =
                            show_searched_restaurants(&restaurants, search_name.as_str());
                        let index1 = read_i32();
                        if index1 == None {
                            continue;
                        }
                        let index1 = index1.unwrap() - 1;
                        if index1 <= -1 || index1 > searched_resturant.len() as i32 {
                            println!("{}", "(out of range)");
                            continue;
                        }
                        u_orders.restaurants.push(OrderItem {
                            restaurant: searched_resturant[index1 as usize].name.clone(),
                            items: Vec::new(),
                            price: 0.0,
                        });
                        loop {
                            println!("{}", "for order, enter the item id(0 is back):");
                            show_restaurant_menu(&(restaurants[index1 as usize]));
                            let index2 = read_i32();
                            if index2 == None {
                                continue;
                            }
                            let index2 = index2.unwrap() - 1;
                            if index2 == -1 {
                                continue 'a;
                            }
                            if index2 > restaurants[index1 as usize].menu.len() as i32 {
                                println!("{}", "item not found");
                                continue;
                            }
                            println!("{}", "Enter the quantity of the item:");
                            let quantity = read_i32();
                            if quantity == None {
                                continue;
                            }
                            let quantity = quantity.unwrap();
                            if quantity <= 0 {
                                println!("{}", "quantity must be greater than 0");
                                continue;
                            }
                            let last_index = u_orders.restaurants.len() - 1;
                            u_orders.restaurants[last_index].items.push(SelectedItem {
                                name: searched_resturant[index1 as usize].menu[index2 as usize]
                                    .name
                                    .clone(),
                                quantity: quantity as u32,
                            });
                            u_orders.restaurants[last_index].price +=
                                searched_resturant[index1 as usize].menu[index2 as usize].price
                                    * quantity as f64;
                            println!("{}", "item added to order");
                        }
                    }
                    3 => {
                        println!("{}", "select your item (example: 1.3 , 0.0 is to back)");
                        show_all_items(&restaurants);
                        let inp = read_string();
                        let inp = inp
                            .split('.')
                            .collect::<Vec<&str>>()
                            .iter()
                            .map(|x| x.parse::<usize>())
                            .collect::<Result<Vec<usize>, _>>();

                        if inp.is_err() {
                            println!("{}", "invalid input");
                            continue;
                        }
                        let inp = inp.unwrap();
                        if inp[0] == 0 && inp[0] == 0 {
                            continue;
                        }
                        let (i, j) = (inp[0] - 1, inp[1] - 1);

                        if i > restaurants.len() || j > restaurants[i].menu.len() {
                            println!("{}", "out of range");
                            continue;
                        }
                        u_orders.restaurants.push(OrderItem {
                            restaurant: restaurants[i].name.clone(),
                            items: Vec::new(),
                            price: 0.0,
                        });
                        println!("{}", "Enter the quantity of the item:");
                        let quantity = read_i32();
                        if quantity == None {
                            continue;
                        }
                        let quantity = quantity.unwrap();
                        if quantity <= 0 {
                            println!("{}", "quantity must be greater than 0");
                            continue;
                        }
                        let last_index = u_orders.restaurants.len() - 1;
                        u_orders.restaurants[last_index].items.push(SelectedItem {
                            name: restaurants[i].menu[j].name.clone(),
                            quantity: quantity as u32,
                        });
                        u_orders.restaurants[last_index].price +=
                            restaurants[i].menu[j].price * quantity as f64;
                        println!("{}", "item added to order");
                    }
                    4 => {
                        println!("{}", "Enter the item name to search:");
                        let search_name = read_string();

                        let (items, r_names) = search_items(&restaurants, &search_name);

                        println!("{}", "Select a restaurant from the list below:");
                        for (i, item) in items.iter().enumerate() {
                            println!(
                                "{}. Name: {}, Price: {}, Description: {}",
                                i + 1,
                                item.name,
                                item.price,
                                item.description
                            );
                        }
                        let index = read_i32();
                        if index == None {
                            continue;
                        }
                        let index = index.unwrap() - 1;
                        if index < 0 || index >= items.len() as i32 {
                            println!("{}", "(out of range)");
                            continue;
                        }
                        println!("{}", "Enter the quantity of the item:");
                        let quantity = read_i32();
                        if quantity == None {
                            continue;
                        }
                        let quantity = quantity.unwrap();
                        if quantity <= 0 {
                            println!("{}", "quantity must be greater than 0");
                            continue;
                        }

                        u_orders.restaurants.push(OrderItem {
                            restaurant: r_names[index as usize].clone(),
                            items: Vec::new(),
                            price: 0.0,
                        });
                        let last_index = u_orders.restaurants.len() - 1;
                        u_orders.restaurants[last_index].items.push(SelectedItem {
                            name: items[index as usize].name.clone(),
                            quantity: quantity as u32,
                        });
                        u_orders.restaurants[last_index].price +=
                            items[index as usize].price * quantity as f64;
                        println!("{}", "item added to order");
                    }
                    5 => {
                        if u_orders.restaurants.is_empty() {
                            println!("{}", "You have no orders");
                            continue;
                        }
                        println!("{}", "Your current orders:");
                        for (i, order) in u_orders.restaurants.iter().enumerate() {
                            println!(
                                "{}. Restaurant: {}, Items: {:?}, Price: {}",
                                i + 1,
                                order.restaurant,
                                order.items,
                                order.price
                            );
                        }
                        let (t, p, f) = calculate_price(&u_orders, admin[0].profit);
                        println!("Total price: {}\nProfit: {}\nFinal price: {}", t, p, f);
                        println!("{}", "Do you want to confirm your order? (yes/no)");
                        let input = read_string();
                        match input.as_str() {
                            "yes" | "Yes" | "YES" => {
                                let is_pay = can_pay(&mut users, &username, f);
                                if is_pay {
                                    u_orders.status = "paid".to_string();
                                    u_orders.total_price = f;
                                    pay(
                                        &mut users,
                                        &mut owners,
                                        &username,
                                        &u_orders,
                                        &mut admin[0],
                                        t,
                                    );
                                    orders.push(u_orders);
                                    u_orders = Order {
                                        username: username.clone(),
                                        restaurants: Vec::new(),
                                        total_price: 0.0,
                                        status: "pending".to_string(),
                                        datetime: chrono::Local::now().to_rfc3339(),
                                    };
                                    println!("{}", "Current order is clean!");
                                } else {
                                    println!(
                                        "{}",
                                        "Not enough stock\nYour Do you want to contuinue your order? (yes/no)"
                                    );
                                    let continue_input = read_string();
                                    match continue_input.as_str() {
                                        "yes" | "Yes" | "YES" => {
                                            println!("{}", "You can continue your order.");
                                        }
                                        "no" | "No" | "NO" => {
                                            println!("{}", "Order cancelled.");
                                            u_orders.total_price = f;
                                            orders.push(u_orders);
                                            u_orders = Order {
                                                username: username.clone(),
                                                restaurants: Vec::new(),
                                                total_price: 0.0,
                                                status: "pending".to_string(),
                                                datetime: chrono::Local::now().to_rfc3339(),
                                            };
                                            println!("{}", "Current order is clean!");
                                        }
                                        _ => {
                                            println!("{}", "Invalid input. Order not confirmed.");
                                        }
                                    }
                                }
                            }

                            _ => {
                                println!("{}", "continue")
                            }
                        }
                    }
                    6 => {
                        show_all_user_orders(&orders);
                    }
                    7 => {
                        add_mony_to_wallet(&mut users, &username);
                    }
                    8 => {
                        println!("{}", "You are signed out");
                        break;
                    }
                    0 => {
                        break 'm;
                    }
                    _ => println!("{}", "just 0-8!"),
                }
            },
            "owner" => loop {
                println!(
                    "{}",
                    "1.show and edit my restaurants 2.add new restaurant 3.show list of orders 4.sign out 0.exit"
                );
                let inp = read_i32();
                if inp == None {
                    continue;
                }
                match inp.unwrap() {
                    1 => {
                        let have_rest = show_owner_restaurants(&restaurants, &username);
                        if !have_rest {
                            println!("{}", "you don't have any reataurnt");
                            continue;
                        }
                        println!("{}", "1.edit 2.remove 0.back");
                        let flag = read_i32();
                        if flag == None {
                            continue;
                        }
                        let flag = flag.unwrap();
                        if flag == 0 {
                            continue;
                        }

                        println!("{}", "select a restaurant");
                        show_owner_restaurants(&restaurants, &username);
                        let i = read_i32();
                        if i == None {
                            continue;
                        }
                        let i = (i.unwrap() - 1) as usize;
                        if flag == 2 {
                            println!("{} is removed", restaurants[i].name);
                            restaurants.remove(i);
                            continue;
                        }
                        println!("{}", "edit 1.name 2.items 0.back");
                        let f = read_i32();
                        if f == None || f.unwrap() == 0 {
                            continue;
                        }
                        let f = f.unwrap();
                        if f == 1 {
                            println!("{}", "enter new name");
                            let new_r_name = read_string();
                            restaurants[i].name = new_r_name.clone();
                            change_owner_restaurant_name(&mut owners, &username, i, &new_r_name);
                            let mut on = find_owner(&owners, &username).unwrap();

                            on.restaurant[i] = new_r_name.clone();
                            println!("restaurant name changed to {}", new_r_name);
                            continue;
                        }
                        if restaurants[i].name.is_empty() {
                            println!("{}", "there is no any item!");
                            continue;
                        }
                        for (i, rest) in restaurants[i].menu.iter().enumerate() {
                            println!(
                                "{}. name: {} price: {} description: {}",
                                i + 1,
                                rest.name,
                                rest.price,
                                rest.description
                            )
                        }
                        println!("{}", "1.edit item 2.remove item 0.back");
                        let flag2 = read_i32();
                        if flag2 == None {
                            continue;
                        }
                        let flag2 = flag2.unwrap();
                        if flag2 == 0 {
                            continue;
                        }
                        println!("{}", "select an item");
                        for (i, rest) in restaurants[i].menu.iter().enumerate() {
                            println!(
                                "{}. name: {} price: {} description: {}",
                                i + 1,
                                rest.name,
                                rest.price,
                                rest.description
                            )
                        }
                        let j = read_i32();
                        if j == None {
                            continue;
                        }
                        let j = (j.unwrap() - 1) as usize;
                        if flag2 == 2 {
                            println!("{} is removed", restaurants[i].menu[j].name);
                            restaurants[i].menu.remove(j);
                            continue;
                        }
                        println!("{}", "edit: 1.name 2.price 3.description 0.back");
                        let flag3 = read_i32();
                        if flag3 == None {
                            continue;
                        }
                        let flag3 = flag3.unwrap();
                        match flag3 {
                            1 => {
                                println!("{}", "enter new name:");
                                let new_name = read_string();
                                restaurants[i].menu[j].name = new_name.clone();
                                println!("name changed to {}!", new_name);
                            }
                            2 => {
                                println!("{}", "enter new price(like: 32.5):");
                                let new_price = read_string().parse::<f64>();
                                if new_price.is_err() {
                                    println!("{}", "it's not a number");
                                    continue;
                                }
                                let new_price = new_price.unwrap();
                                restaurants[i].menu[j].price = new_price;
                                println!("price changed to {}!", new_price);
                            }
                            3 => {
                                println!("{}", "enter new description:");
                                let new_desc = read_string();
                                restaurants[i].menu[j].description = new_desc.clone();
                                println!("dexcription changed to {}1", new_desc);
                            }
                            0 => {
                                continue;
                            }
                            _ => {
                                println!("{}", "just 0-3!");
                                continue;
                            }
                        }
                    }
                    2 => {
                        println!("{}", "enter restaurant name:");
                        let r_name = read_string();
                        if !check_restaurant_name(&restaurants, &r_name) {
                            println!("{}", "this name is repetitive");
                            continue;
                        }
                        println!("{}", "enter restaurant category:");
                        let cat = read_string();
                        let mut items = Vec::new();
                        loop {
                            println!("{}", "1.add item 0.finish");
                            let input = read_i32();
                            if input == None {
                                continue;
                            }
                            let input = input.unwrap();
                            if input == 0 {
                                break;
                            }
                            println!("{}", "enter item name:");
                            let item_name = read_string();
                            if !check_item_name(&items, &item_name) {
                                println!("{}", "this name is repetitive");
                                continue;
                            }
                            println!("{}", "enter item price(like 320.5):");
                            let price = read_string();
                            let price = price.parse::<f64>();
                            if price.is_err() {
                                println!("{}", "enput is not valid");
                                continue;
                            }
                            let price = price.unwrap();
                            println!("{}", "enter item describtion:");
                            let des = read_string();
                            items.push(Item {
                                name: item_name,
                                price,
                                description: des,
                            });
                            println!("{}", "item added successfuly");
                        }
                        restaurants.push(Restaurant {
                            name: r_name.clone(),
                            owner: username.clone(),
                            category: cat,
                            menu: items,
                        });
                        let mut on = find_owner(&owners, &username).unwrap();
                        on.restaurant.push(r_name);
                        println!("{}", "restaurant added successfuly");
                    }
                    3 => {
                        println!("{}", "1.paids 2.unpaids 0.back");
                        let input = read_i32();
                        if input == None {
                            continue;
                        }
                        if input.unwrap() > 2 || input.unwrap() < 1 {
                            println!("{}", "select bitwin 1, 2 or 0!");
                            continue;
                        }
                        let is_paid = match input.unwrap() {
                            1 => true,
                            2 => false,
                            _ => true,
                        };
                        let own = find_owner(&owners, &username);
                        show_all_owner_order(&orders, is_paid, &own.unwrap());
                    }
                    4 => {
                        println!("{}", "You are signed out");
                        break;
                    }
                    0 => {
                        break 'm;
                    }
                    _ => println!("{}", "just 0-4!"),
                }
            },
            "admin" => loop {
                println!(
                    "{}",
                    "1.show all users 2.show all restaurant  3.show all orders 4.show wallet and change profit 5.sign out 0.exit"
                );
                let inp = read_i32();
                if inp == None {
                    continue;
                }
                match inp.unwrap() {
                    1 => {
                        println!("{}", "1.users 2.owners 0.back");
                        let input = read_i32();
                        if input == None {
                            continue;
                        }
                        let input = input.unwrap();
                        if input == 1 {
                            show_all_users(&users);
                        } else if input == 2 {
                            show_all_owners(&owners);
                        } else {
                            println!("{}", "just 0-2");
                            continue;
                        }
                        println!("{}", "1.delet 2.edit wallet 0.back");
                        let input2 = read_i32();
                        if input2 == None {
                            continue;
                        }
                        let input2 = input2.unwrap();
                        if input2 == 0 {
                            continue;
                        }
                        println!("{}", "select an index");
                        let index = read_i32();
                        if index == None {
                            continue;
                        }
                        let index = index.unwrap() - 1;
                        if input == 1 {
                            if index < 0 || index as usize > users.len() {
                                println!("{}", "out of range");
                                continue;
                            }
                            show_all_users(&users);
                        } else {
                            if index < 0 || index as usize > owners.len() {
                                println!("{}", "out of range");
                                continue;
                            }
                            show_all_owners(&owners);
                        }
                        if input2 == 1 {
                            if input == 1 {
                                users.remove(index as usize);
                            } else {
                                for (i, rest) in restaurants.clone().iter().enumerate() {
                                    if rest.name == owners[index as usize].username {
                                        restaurants.remove(i);
                                    }
                                }
                                owners.remove(index as usize);
                            }
                            println!("{}", "account deleted");
                            continue;
                        }
                        println!("{}", "enter new Inventory(like: 4123123.2):");
                        let new_inv = read_string();
                        let new_inv = new_inv.parse::<f64>();
                        if new_inv.is_err() {
                            println!("{}", "invalid input");
                            continue;
                        }
                        let new_inv = new_inv.unwrap();
                        if input == 1 {
                            users[index as usize].wallet = new_inv;
                        } else {
                            owners[index as usize].wallet = new_inv;
                        }
                        println!("{}", "inventory updated");
                    }
                    2 => {
                        show_all_restaurants(&restaurants);
                        println!("1.delet 0.back");
                        let input = read_i32();
                        if input == None {
                            continue;
                        }
                        match input.unwrap() {
                            1 => {
                                println!("{}", "select an index");
                                let index = read_i32();
                                if index == None {
                                    continue;
                                }
                                if index.unwrap() < 1 || index.unwrap() > restaurants.len() as i32 {
                                    println!("{}", "out of range");
                                    continue;
                                }
                                let index = index.unwrap() as usize - 1;
                                restaurants.remove(index);
                                println!("{}", "restaurant removed");
                            }
                            0 => continue,
                            _ => println!("{}", "invalid input"),
                        }
                    }
                    3 => {
                        show_all_orders(&orders);
                    }
                    4 => {
                        println!("wallet: {} profit: {}", admin[0].wallet,admin[0].profit);
                        println!("{}", "1.change profit 0.back");
                        let input = read_i32();
                        if input == None {
                            continue;
                        }
                        match input.unwrap() {
                            1 => {
                                println!("{}", "enter new profit(like: 0.30):");
                                let new_f = read_string();
                                let new_f = new_f.parse::<f64>();
                                if new_f.is_err() {
                                    println!("{}", "invalid input");
                                    continue;
                                }
                                admin[0].profit = new_f.unwrap();
                                println!("{}", "profit changed");
                            }
                            2 => {
                                continue;
                            }
                            _ => {
                                println!("{}", "out of range")
                            }
                        }
                    }
                    5 => {
                        println!("{}", "You are signed out");
                        break;
                    }
                    0 => {
                        break 'm;
                    }
                    _ => println!("{}", "just 0-7!"),
                }
            }
            _ => {
                println!("{}", "YOU ARE OUT OF PROGRAM")
            }
        }
    }
    update_data(orders, restaurants, users, owners, admin);
    println!("{}", "Goodbye");
    // println!("Users: {:#?}", users);
    // println!("Owners: {:#?}", owners);
    // println!("Admin: {:#?}", admin);
    // println!("Orders: {:#?}", orders);
    // println!("Restaurants: {:#?}", restaurants);
    // update_data(orders, restaurants, users, owners, admin);
    // users.push(models::User {
    //     username: "test_user".to_string(),
    //     password: "test_password".to_string(),
    //     role: "user".to_string(),
    //     wallet: 100,
    // });
    // restaurants.push(models::Restaurant {
    //     name: "test_restaurant".to_string(),
    //     owner: "test_owner".to_string(),
    //     category: "test_category".to_string(),
    //     menu: vec![models::Item {
    //         name: "test_item".to_string(),
    //         price: 10.0,
    //         description: "test_description".to_string(),
    //     }],
    // });
    // owners.push(models::Owner {
    //     username: "test_owner".to_string(),
    //     password: "test_password".to_string(),
    //     role: "owner".to_string(),
    //     wallet: 100,
    //     restaurant: vec!["test_restaurant".to_string()],
    // });
    // orders.push(models::Order {
    //     username: "test_user".to_string(),
    //     restaurants: vec![models::OrderItem {
    //         restaurant: "test_restaurant".to_string(),
    //         items: vec![models::SelectedItem {
    //             name: "test_item".to_string(),
    //             quantity: 1,
    //         }],
    //         price: 10.0,
    //     }],
    //     total_price: 10.0,
    //     status: "pending".to_string(),
    //     datetime: "2023-10-01T12:00:00Z".to_string(),
    // });
    // admin.push(models::Admin {
    //     username: "test_admin".to_string(),
    //     password: "test_password".to_string(),
    //     role: "admin".to_string(),
    //     wallet: 100,
    //     profit: 0.0,
    // });
    // update_data(orders, restaurants, users, owners, admin);
}
