use sea_orm::EntityTrait;
use actix_web::{web, HttpResponse};

use crate::{
    dtos::*,
    state::AppState,
    services::*,
    error::AppError,
    db::*,
};

pub async fn get_shipments(data: web::Data<AppState>) -> Result<HttpResponse, AppError> {
    let shipments = ShipmentService::find_all(&data.db).await?;
    
    let mut shipments_with_additional_information = Vec::new();
    for shipment in shipments {
        let supplier = Suppliers::find_by_id(shipment.supplier_id)
            .one(&data.db)
            .await?
            .unwrap_or_default();
        
        let response = ShipmentResponse {
            shipment_id: shipment.shipment_id,
            supplier_id: shipment.supplier_id,
            shipment_date: shipment.shipment_date,
            expected_delivery_date: shipment.expected_delivery_date,
            status: shipment.status,
            total_cost: shipment.total_cost,
            supplier_name: Some(supplier.company_name),
        };
        shipments_with_additional_information.push(response);
    }
    
    Ok(HttpResponse::Ok().json(shipments_with_additional_information))
}

pub async fn get_shipment(
    data: web::Data<AppState>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let shipment_id = path.into_inner();
    let shipment = ShipmentService::find_by_id(&data.db, shipment_id).await?;
    
    let supplier = Suppliers::find_by_id(shipment.supplier_id)
        .one(&data.db)
        .await?
        .unwrap_or_default();
    
    let response = ShipmentResponse {
        shipment_id: shipment.shipment_id,
        supplier_id: shipment.supplier_id,
        shipment_date: shipment.shipment_date,
        expected_delivery_date: shipment.expected_delivery_date,
        status: shipment.status,
        total_cost: shipment.total_cost,
        supplier_name: Some(supplier.company_name),
    };
    
    Ok(HttpResponse::Ok().json(response))
}

pub async fn create_shipment(
    data: web::Data<AppState>,
    dto: web::Json<ShipmentCreate>,
) -> Result<HttpResponse, AppError> {
    let shipment = ShipmentService::create(&data.db, dto.into_inner()).await?;
    
    let supplier = Suppliers::find_by_id(shipment.supplier_id)
        .one(&data.db)
        .await?
        .unwrap_or_default();
    
    let response = ShipmentResponse {
        shipment_id: shipment.shipment_id,
        supplier_id: shipment.supplier_id,
        shipment_date: shipment.shipment_date,
        expected_delivery_date: shipment.expected_delivery_date,
        status: shipment.status,
        total_cost: shipment.total_cost,
        supplier_name: Some(supplier.company_name),
    };
    
    Ok(HttpResponse::Created().json(response))
}

pub async fn update_shipment(
    data: web::Data<AppState>,
    path: web::Path<i32>,
    dto: web::Json<ShipmentUpdate>,
) -> Result<HttpResponse, AppError> {
    let shipment_id = path.into_inner();
    let shipment = ShipmentService::update(&data.db, shipment_id, dto.into_inner()).await?;
    
    let supplier = Suppliers::find_by_id(shipment.supplier_id)
        .one(&data.db)
        .await?
        .unwrap_or_default();
    
    let response = ShipmentResponse {
        shipment_id: shipment.shipment_id,
        supplier_id: shipment.supplier_id,
        shipment_date: shipment.shipment_date,
        expected_delivery_date: shipment.expected_delivery_date,
        status: shipment.status,
        total_cost: shipment.total_cost,
        supplier_name: Some(supplier.company_name),
    };
    
    Ok(HttpResponse::Ok().json(response))
}

pub async fn delete_shipment(
    data: web::Data<AppState>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let shipment_id = path.into_inner();
    ShipmentService::delete(&data.db, shipment_id).await?;
    Ok(HttpResponse::NoContent().finish())
}

pub async fn get_shipment_details(
    data: web::Data<AppState>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let shipment_id = path.into_inner();
    let (shipment, items) = ShipmentService::find_with_details(&data.db, shipment_id).await?;
    
    let supplier = Suppliers::find_by_id(shipment.supplier_id)
        .one(&data.db)
        .await?
        .unwrap_or_default();
    
    let mut items_with_additional_information = Vec::new();
    for item in items {
        let product = Products::find_by_id(item.product_id)
            .one(&data.db)
            .await?
            .unwrap_or_default();
        
        let item_response = ShipmentItemResponse {
            shipment_id: item.shipment_id,
            product_id: item.product_id,
            quantity: item.quantity,
            unit_cost: item.unit_cost,
            product_name: Some(product.name),
        };
        items_with_additional_information.push(item_response);
    }
    
    let response = ShipmentDetailsResponse {
        shipment_id: shipment.shipment_id,
        supplier_id: shipment.supplier_id,
        shipment_date: shipment.shipment_date,
        expected_delivery_date: shipment.expected_delivery_date,
        status: shipment.status,
        total_cost: shipment.total_cost,
        supplier_name: Some(supplier.company_name),
        items: items_with_additional_information,
    };
    
    Ok(HttpResponse::Ok().json(response))
}
