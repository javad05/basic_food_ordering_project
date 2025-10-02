use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub password: String,
    pub role: String,
    pub wallet: f64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Owner {
    pub username: String,
    pub password: String,
    pub role: String,
    pub wallet: f64,
    pub restaurant: Vec<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Admin {
    pub username: String,
    pub password: String,
    pub role: String,
    pub wallet: f64,
    pub profit: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    pub username: String,
    pub restaurants: Vec<OrderItem>,
    pub total_price: f64,
    pub status: String,
    pub datetime: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct OrderItem {
    pub restaurant: String,
    pub items: Vec<SelectedItem>,
    pub price: f64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SelectedItem {
    pub name: String,
    pub quantity: u32,
}
#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct Restaurant {
    pub name: String,
    pub owner: String,
    pub category: String,
    pub menu: Vec<Item>,
}
#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct Item {
    pub name: String,
    pub price: f64,
    pub description: String,
}
