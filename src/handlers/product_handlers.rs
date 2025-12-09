use sea_orm::EntityTrait;
use actix_web::{web, HttpResponse};

use crate::{
    dtos::*,
    state::AppState,
    services::*,
    error::AppError,
    db::*,
};

pub async fn get_products(data: web::Data<AppState>) -> Result<HttpResponse, AppError> {
    let products = ProductService::find_all(&data.db).await?;
    
    let mut products_with_additional_data = Vec::new();
    for product in products {
        let category = Categories::find_by_id(product.category_id)
            .one(&data.db)
            .await?
            .unwrap_or_default();
        
        let supplier = Suppliers::find_by_id(product.supplier_id)
            .one(&data.db)
            .await?
            .unwrap_or_default();
        
        let response = ProductResponse {
            product_id: product.product_id,
            name: product.name,
            description: product.description,
            price: product.price,
            stock_quantity: product.stock_quantity,
            category_id: product.category_id,
            supplier_id: product.supplier_id,
            category_name: Some(category.name),
            supplier_name: Some(supplier.company_name),
        };
        products_with_additional_data.push(response);
    }
    
    Ok(HttpResponse::Ok().json(products_with_additional_data))
}

pub async fn get_product(
    data: web::Data<AppState>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let product_id = path.into_inner();
    let product = ProductService::find_by_id(&data.db, product_id).await?;
    
    let category = Categories::find_by_id(product.category_id)
        .one(&data.db)
        .await?
        .unwrap_or_default();
    
    let supplier = Suppliers::find_by_id(product.supplier_id)
        .one(&data.db)
        .await?
        .unwrap_or_default();
    
    let response = ProductResponse {
        product_id: product.product_id,
        name: product.name,
        description: product.description,
        price: product.price,
        stock_quantity: product.stock_quantity,
        category_id: product.category_id,
        supplier_id: product.supplier_id,
        category_name: Some(category.name),
        supplier_name: Some(supplier.company_name),
    };
    
    Ok(HttpResponse::Ok().json(response))
}

pub async fn create_product(
    data: web::Data<AppState>,
    dto: web::Json<ProductCreate>,
) -> Result<HttpResponse, AppError> {
    let product = ProductService::create(&data.db, dto.into_inner()).await?;
    
    let category = Categories::find_by_id(product.category_id)
        .one(&data.db)
        .await?
        .unwrap_or_default();
    
    let supplier = Suppliers::find_by_id(product.supplier_id)
        .one(&data.db)
        .await?
        .unwrap_or_default();
    
    let response = ProductResponse {
        product_id: product.product_id,
        name: product.name,
        description: product.description,
        price: product.price,
        stock_quantity: product.stock_quantity,
        category_id: product.category_id,
        supplier_id: product.supplier_id,
        category_name: Some(category.name),
        supplier_name: Some(supplier.company_name),
    };
    
    Ok(HttpResponse::Created().json(response))
}

pub async fn update_product(
    data: web::Data<AppState>,
    path: web::Path<i32>,
    dto: web::Json<ProductUpdate>,
) -> Result<HttpResponse, AppError> {
    let product_id = path.into_inner();
    let product = ProductService::update(&data.db, product_id, dto.into_inner()).await?;
    
    let category = Categories::find_by_id(product.category_id)
        .one(&data.db)
        .await?
        .unwrap_or_default();
    
    let supplier = Suppliers::find_by_id(product.supplier_id)
        .one(&data.db)
        .await?
        .unwrap_or_default();
    
    let response = ProductResponse {
        product_id: product.product_id,
        name: product.name,
        description: product.description,
        price: product.price,
        stock_quantity: product.stock_quantity,
        category_id: product.category_id,
        supplier_id: product.supplier_id,
        category_name: Some(category.name),
        supplier_name: Some(supplier.company_name),
    };
    
    Ok(HttpResponse::Ok().json(response))
}

pub async fn delete_product(
    data: web::Data<AppState>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let product_id = path.into_inner();
    ProductService::delete(&data.db, product_id).await?;
    Ok(HttpResponse::NoContent().finish())
}

