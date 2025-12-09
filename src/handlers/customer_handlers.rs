use sea_orm::EntityTrait;
use actix_web::{web, HttpResponse};

use crate::{
    dtos::*,
    state::AppState,
    services::*,
    error::AppError,
};

pub async fn get_customers(data: web::Data<AppState>) -> Result<HttpResponse, AppError> {
    let customers = CustomerService::find_all(&data.db).await?;
    let response: Vec<CustomerResponse> = customers.into_iter().map(|c| CustomerResponse {
        customer_id: c.customer_id,
        first_name: c.first_name,
        last_name: c.last_name,
        email: c.email,
        phone: c.phone,
        registration_date: c.registration_date,
        address: c.address,
    }).collect();
    Ok(HttpResponse::Ok().json(response))
}

pub async fn get_customer(
    data: web::Data<AppState>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let customer_id = path.into_inner();
    let customer = CustomerService::find_by_id(&data.db, customer_id).await?;
    let response = CustomerResponse {
        customer_id: customer.customer_id,
        first_name: customer.first_name,
        last_name: customer.last_name,
        email: customer.email,
        phone: customer.phone,
        registration_date: customer.registration_date,
        address: customer.address,
    };
    Ok(HttpResponse::Ok().json(response))
}

pub async fn create_customer(
    data: web::Data<AppState>,
    dto: web::Json<CustomerCreate>,
) -> Result<HttpResponse, AppError> {
    let customer = CustomerService::create(&data.db, dto.into_inner()).await?;
    let response = CustomerResponse {
        customer_id: customer.customer_id,
        first_name: customer.first_name,
        last_name: customer.last_name,
        email: customer.email,
        phone: customer.phone,
        registration_date: customer.registration_date,
        address: customer.address,
    };
    Ok(HttpResponse::Created().json(response))
}

pub async fn update_customer(
    data: web::Data<AppState>,
    path: web::Path<i32>,
    dto: web::Json<CustomerUpdate>,
) -> Result<HttpResponse, AppError> {
    let customer_id = path.into_inner();
    let customer = CustomerService::update(&data.db, customer_id, dto.into_inner()).await?;
    let response = CustomerResponse {
        customer_id: customer.customer_id,
        first_name: customer.first_name,
        last_name: customer.last_name,
        email: customer.email,
        phone: customer.phone,
        registration_date: customer.registration_date,
        address: customer.address,
    };
    Ok(HttpResponse::Ok().json(response))
}

pub async fn delete_customer(
    data: web::Data<AppState>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let customer_id = path.into_inner();
    CustomerService::delete(&data.db, customer_id).await?;
    Ok(HttpResponse::NoContent().finish())
}

pub async fn get_customer_orders(
    data: web::Data<AppState>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let customer_id = path.into_inner();
    let (customer, orders) = CustomerService::find_with_orders(&data.db, customer_id).await?;
    
    let customer_response = CustomerResponse {
        customer_id: customer.customer_id,
        first_name: customer.first_name.clone(),
        last_name: customer.last_name.clone(),
        email: customer.email,
        phone: customer.phone,
        registration_date: customer.registration_date,
        address: customer.address,
    };
    
    let orders_response: Vec<OrderResponse> = orders.into_iter().map(|o| OrderResponse {
        order_id: o.order_id,
        customer_id: o.customer_id,
        order_date: o.order_date,
        status: o.status,
        total_amount: o.total_amount,
        shipping_address: o.shipping_address,
        customer_name: Some(format!("{} {}", customer.first_name, customer.last_name)),
    }).collect();
    
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "customer": customer_response,
        "orders": orders_response
    })))
}

