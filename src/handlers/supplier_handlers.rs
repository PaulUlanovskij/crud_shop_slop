use sea_orm::EntityTrait;
use actix_web::{web, HttpResponse};

use crate::{
    dtos::*,
    state::AppState,
    services::*,
    error::AppError,
};

pub async fn get_suppliers(data: web::Data<AppState>) -> Result<HttpResponse, AppError> {
    let suppliers = SupplierService::find_all(&data.db).await?;
    let response: Vec<SupplierResponse> = suppliers.into_iter().map(|s| SupplierResponse {
        supplier_id: s.supplier_id,
        company_name: s.company_name,
        contact_name: s.contact_name,
        email: s.email,
        phone: s.phone,
        address: s.address,
    }).collect();
    Ok(HttpResponse::Ok().json(response))
}

pub async fn get_supplier(
    data: web::Data<AppState>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let supplier_id = path.into_inner();
    let supplier = SupplierService::find_by_id(&data.db, supplier_id).await?;
    let response = SupplierResponse {
        supplier_id: supplier.supplier_id,
        company_name: supplier.company_name,
        contact_name: supplier.contact_name,
        email: supplier.email,
        phone: supplier.phone,
        address: supplier.address,
    };
    Ok(HttpResponse::Ok().json(response))
}

pub async fn create_supplier(
    data: web::Data<AppState>,
    dto: web::Json<SupplierCreate>,
) -> Result<HttpResponse, AppError> {
    let supplier = SupplierService::create(&data.db, dto.into_inner()).await?;
    let response = SupplierResponse {
        supplier_id: supplier.supplier_id,
        company_name: supplier.company_name,
        contact_name: supplier.contact_name,
        email: supplier.email,
        phone: supplier.phone,
        address: supplier.address,
    };
    Ok(HttpResponse::Created().json(response))
}

pub async fn update_supplier(
    data: web::Data<AppState>,
    path: web::Path<i32>,
    dto: web::Json<SupplierUpdate>,
) -> Result<HttpResponse, AppError> {
    let supplier_id = path.into_inner();
    let supplier = SupplierService::update(&data.db, supplier_id, dto.into_inner()).await?;
    let response = SupplierResponse {
        supplier_id: supplier.supplier_id,
        company_name: supplier.company_name,
        contact_name: supplier.contact_name,
        email: supplier.email,
        phone: supplier.phone,
        address: supplier.address,
    };
    Ok(HttpResponse::Ok().json(response))
}

pub async fn delete_supplier(
    data: web::Data<AppState>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let supplier_id = path.into_inner();
    SupplierService::delete(&data.db, supplier_id).await?;
    Ok(HttpResponse::NoContent().finish())
}

pub async fn get_supplier_products(
    data: web::Data<AppState>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let supplier_id = path.into_inner();
    let (supplier, products) = SupplierService::find_with_products(&data.db, supplier_id).await?;
    
    let supplier_response = SupplierResponse {
        supplier_id: supplier.supplier_id,
        company_name: supplier.company_name.clone(),
        contact_name: supplier.contact_name,
        email: supplier.email,
        phone: supplier.phone,
        address: supplier.address,
    };
    
    let products_response: Vec<ProductResponse> = products.into_iter().map(|p| ProductResponse {
        product_id: p.product_id,
        name: p.name,
        description: p.description,
        price: p.price,
        stock_quantity: p.stock_quantity,
        category_id: p.category_id,
        supplier_id: p.supplier_id,
        category_name: None,
        supplier_name: Some(supplier.company_name.clone()),
    }).collect();
    
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "supplier": supplier_response,
        "products": products_response
    })))
}
