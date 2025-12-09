use sea_orm::EntityTrait;
use actix_web::{web, HttpResponse};

use crate::{
    dtos::*,
    state::AppState,
    services::*,
    error::AppError,
};

pub async fn get_categories(data: web::Data<AppState>) -> Result<HttpResponse, AppError> {
    let categories = CategoryService::find_all(&data.db).await?;
    let response: Vec<CategoryResponse> = categories.into_iter().map(|c| CategoryResponse {
        category_id: c.category_id,
        name: c.name,
        description: c.description,
    }).collect();
    Ok(HttpResponse::Ok().json(response))
}

pub async fn get_category(
    data: web::Data<AppState>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let category_id = path.into_inner();
    let category = CategoryService::find_by_id(&data.db, category_id).await?;
    let response = CategoryResponse {
        category_id: category.category_id,
        name: category.name,
        description: category.description,
    };
    Ok(HttpResponse::Ok().json(response))
}

pub async fn create_category(
    data: web::Data<AppState>,
    dto: web::Json<CategoryCreate>,
) -> Result<HttpResponse, AppError> {
    let category = CategoryService::create(&data.db, dto.into_inner()).await?;
    let response = CategoryResponse {
        category_id: category.category_id,
        name: category.name,
        description: category.description,
    };
    Ok(HttpResponse::Created().json(response))
}

pub async fn update_category(
    data: web::Data<AppState>,
    path: web::Path<i32>,
    dto: web::Json<CategoryUpdate>,
) -> Result<HttpResponse, AppError> {
    let category_id = path.into_inner();
    let category = CategoryService::update(&data.db, category_id, dto.into_inner()).await?;
    let response = CategoryResponse {
        category_id: category.category_id,
        name: category.name,
        description: category.description,
    };
    Ok(HttpResponse::Ok().json(response))
}

pub async fn delete_category(
    data: web::Data<AppState>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let category_id = path.into_inner();
    CategoryService::delete(&data.db, category_id).await?;
    Ok(HttpResponse::NoContent().finish())
}

