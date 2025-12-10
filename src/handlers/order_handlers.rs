use sea_orm::EntityTrait;
use actix_web::{web, HttpResponse};

use crate::{
    dtos::*,
    state::AppState,
    services::*,
    error::AppError,
    db::*,
};

pub async fn get_orders(data: web::Data<AppState>) -> Result<HttpResponse, AppError> {
    let orders = OrderService::find_all(&data.db).await?;
    
    let mut orders_with_additional_information = Vec::new();
    for order in orders {
        let customer = Customers::find_by_id(order.customer_id)
            .one(&data.db)
            .await?
            .unwrap_or_default();
        
        let response = OrderResponse {
            order_id: order.order_id,
            customer_id: order.customer_id,
            order_date: order.order_date,
            status: order.status,
            total_amount: order.total_amount,
            shipping_address: order.shipping_address,
            customer_name: Some(format!("{} {}", customer.first_name, customer.last_name)),
        };
        orders_with_additional_information.push(response);
    }
    
    Ok(HttpResponse::Ok().json(orders_with_additional_information))
}

pub async fn get_order(
    data: web::Data<AppState>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let order_id = path.into_inner();
    let order = OrderService::find_by_id(&data.db, order_id).await?;
    
    let customer = Customers::find_by_id(order.customer_id)
        .one(&data.db)
        .await?
        .unwrap_or_default();
    
    let response = OrderResponse {
        order_id: order.order_id,
        customer_id: order.customer_id,
        order_date: order.order_date,
        status: order.status,
        total_amount: order.total_amount,
        shipping_address: order.shipping_address,
        customer_name: Some(format!("{} {}", customer.first_name, customer.last_name)),
    };
    
    Ok(HttpResponse::Ok().json(response))
}

pub async fn create_order(
    data: web::Data<AppState>,
    dto: web::Json<OrderCreate>,
) -> Result<HttpResponse, AppError> {
    let order = OrderService::create(&data.db, dto.into_inner()).await?;
    
    let customer = Customers::find_by_id(order.customer_id)
        .one(&data.db)
        .await?
        .unwrap_or_default();
    
    let response = OrderResponse {
        order_id: order.order_id,
        customer_id: order.customer_id,
        order_date: order.order_date,
        status: order.status,
        total_amount: order.total_amount,
        shipping_address: order.shipping_address,
        customer_name: Some(format!("{} {}", customer.first_name, customer.last_name)),
    };
    
    Ok(HttpResponse::Created().json(response))
}

pub async fn update_order(
    data: web::Data<AppState>,
    path: web::Path<i32>,
    dto: web::Json<OrderUpdate>,
) -> Result<HttpResponse, AppError> {
    let order_id = path.into_inner();
    let order = OrderService::update(&data.db, order_id, dto.into_inner()).await?;
    
    let customer = Customers::find_by_id(order.customer_id)
        .one(&data.db)
        .await?
        .unwrap_or_default();
    
    let response = OrderResponse {
        order_id: order.order_id,
        customer_id: order.customer_id,
        order_date: order.order_date,
        status: order.status,
        total_amount: order.total_amount,
        shipping_address: order.shipping_address,
        customer_name: Some(format!("{} {}", customer.first_name, customer.last_name)),
    };
    
    Ok(HttpResponse::Ok().json(response))
}

pub async fn delete_order(
    data: web::Data<AppState>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let order_id = path.into_inner();
    OrderService::delete(&data.db, order_id).await?;
    Ok(HttpResponse::NoContent().finish())
}

pub async fn get_order_details(
    data: web::Data<AppState>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let order_id = path.into_inner();
    
    // Get order with items
    let (order, items) = OrderService::find_with_details(&data.db, order_id).await?;
    
    // Get customer separately (just needed fields)
    let customer = Customers::find_by_id(order.customer_id)
        .one(&data.db)
        .await?
        .ok_or_else(|| AppError::NotFound)?;
    
    // Build item responses WITHOUT full product objects
    let mut item_responses = Vec::new();
    for item in items {
        let product = Products::find_by_id(item.product_id)
            .one(&data.db)
            .await?
            .ok_or_else(|| AppError::NotFound)?;
        
        item_responses.push(OrderItemResponse {
            product_id: item.product_id,
            order_id: order.order_id,
            product_name: Some(product.name),
            quantity: item.quantity,
            unit_price: item.unit_price,   // Price at order time
        });
    }
    
    let response = OrderDetailsResponse {
        order_id: order.order_id,
        customer_id: order.customer_id,
        order_date: order.order_date,
        status: order.status,
        total_amount: order.total_amount,
        shipping_address: order.shipping_address,
        customer_name: format!("{} {}", customer.first_name, customer.last_name).into(),
        items: item_responses,
    };
    
    Ok(HttpResponse::Ok().json(response))
}

