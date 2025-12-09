use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, NaiveDate};

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryCreate {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryUpdate {
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryResponse {
    pub category_id: i32,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomerCreate {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub address: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomerUpdate {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomerResponse {
    pub customer_id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub registration_date: DateTime<Utc>,
    pub address: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SupplierCreate {
    pub company_name: String,
    pub contact_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SupplierUpdate {
    pub company_name: Option<String>,
    pub contact_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SupplierResponse {
    pub supplier_id: i32,
    pub company_name: String,
    pub contact_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductCreate {
    pub name: String,
    pub description: Option<String>,
    pub price: rust_decimal::Decimal,
    pub stock_quantity: i32,
    pub category_id: i32,
    pub supplier_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductUpdate {
    pub name: Option<String>,
    pub description: Option<String>,
    pub price: Option<rust_decimal::Decimal>,
    pub stock_quantity: Option<i32>,
    pub category_id: Option<i32>,
    pub supplier_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductResponse {
    pub product_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub price: rust_decimal::Decimal,
    pub stock_quantity: i32,
    pub category_id: i32,
    pub supplier_id: i32,
    pub category_name: Option<String>,
    pub supplier_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderItemCreate {
    pub product_id: i32,
    pub quantity: i32,
    pub unit_price: rust_decimal::Decimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderItemResponse {
    pub order_id: i32,
    pub product_id: i32,
    pub quantity: i32,
    pub unit_price: rust_decimal::Decimal,
    pub product_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderCreate {
    pub customer_id: i32,
    pub status: Option<String>,
    pub total_amount: rust_decimal::Decimal,
    pub shipping_address: String,
    pub items: Vec<OrderItemCreate>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderUpdate {
    pub status: Option<String>,
    pub total_amount: Option<rust_decimal::Decimal>,
    pub shipping_address: Option<String>,
    pub items: Option<Vec<OrderItemCreate>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderResponse {
    pub order_id: i32,
    pub customer_id: i32,
    pub order_date: DateTime<Utc>,
    pub status: String,
    pub total_amount: rust_decimal::Decimal,
    pub shipping_address: String,
    pub customer_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderDetailsResponse {
    pub order_id: i32,
    pub customer_id: i32,
    pub order_date: DateTime<Utc>,
    pub status: String,
    pub total_amount: rust_decimal::Decimal,
    pub shipping_address: String,
    pub customer_name: Option<String>,
    pub items: Vec<OrderItemResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShipmentItemCreate {
    pub product_id: i32,
    pub quantity: i32,
    pub unit_cost: rust_decimal::Decimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShipmentItemResponse {
    pub shipment_id: i32,
    pub product_id: i32,
    pub quantity: i32,
    pub unit_cost: rust_decimal::Decimal,
    pub product_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShipmentCreate {
    pub supplier_id: i32,
    pub shipment_date: NaiveDate,
    pub expected_delivery_date: NaiveDate,
    pub status: Option<String>,
    pub total_cost: rust_decimal::Decimal,
    pub items: Vec<ShipmentItemCreate>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShipmentUpdate {
    pub status: Option<String>,
    pub expected_delivery_date: Option<NaiveDate>,
    pub total_cost: Option<rust_decimal::Decimal>,
    pub items: Option<Vec<ShipmentItemCreate>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShipmentResponse {
    pub shipment_id: i32,
    pub supplier_id: i32,
    pub shipment_date: NaiveDate,
    pub expected_delivery_date: NaiveDate,
    pub status: String,
    pub total_cost: rust_decimal::Decimal,
    pub supplier_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShipmentDetailsResponse {
    pub shipment_id: i32,
    pub supplier_id: i32,
    pub shipment_date: NaiveDate,
    pub expected_delivery_date: NaiveDate,
    pub status: String,
    pub total_cost: rust_decimal::Decimal,
    pub supplier_name: Option<String>,
    pub items: Vec<ShipmentItemResponse>,
}
