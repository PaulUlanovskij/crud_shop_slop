use anyhow::Result;
use sea_orm::{
    ActiveModelTrait, DatabaseConnection, EntityTrait, QueryOrder, Set
};
use crate::db::{Products, products};
use crate::dtos::*;
use crate::error::AppError;

pub struct ProductService;

impl ProductService {
    pub async fn find_all(db: &DatabaseConnection) -> Result<Vec<products::Model>, AppError> {
        let products = Products::find()
            .order_by_asc(products::Column::Name)
            .all(db)
            .await?;
        Ok(products)
    }

    pub async fn find_by_id(db: &DatabaseConnection, id: i32) -> Result<products::Model, AppError> {
        Products::find_by_id(id)
            .one(db)
            .await?
            .ok_or(AppError::NotFound)
    }

    pub async fn create(db: &DatabaseConnection, dto: ProductCreate) -> Result<products::Model, AppError> {
        let product = products::ActiveModel {
            name: Set(dto.name),
            description: Set(dto.description),
            price: Set(dto.price),
            stock_quantity: Set(dto.stock_quantity),
            category_id: Set(dto.category_id),
            supplier_id: Set(dto.supplier_id),
            ..Default::default()
        };
        
        Ok(product.insert(db).await?)
    }

    pub async fn update(db: &DatabaseConnection, id: i32, dto: ProductUpdate) -> Result<products::Model, AppError> {
        let product = Self::find_by_id(db, id).await?;
        
        let mut product: products::ActiveModel = product.into();
        
        if let Some(name) = dto.name {
            product.name = Set(name);
        }
        if let Some(description) = dto.description {
            product.description = Set(Some(description));
        }
        if let Some(price) = dto.price {
            product.price = Set(price);
        }
        if let Some(stock_quantity) = dto.stock_quantity {
            product.stock_quantity = Set(stock_quantity);
        }
        if let Some(category_id) = dto.category_id {
            product.category_id = Set(category_id);
        }
        if let Some(supplier_id) = dto.supplier_id {
            product.supplier_id = Set(supplier_id);
        }
        
        Ok(product.update(db).await?)
    }

    pub async fn delete(db: &DatabaseConnection, id: i32) -> Result<(), AppError> {
        let product = Self::find_by_id(db, id).await?;
        let product: products::ActiveModel = product.into();
        product.delete(db).await?;
        Ok(())
    }

    pub async fn update_stock(db: &DatabaseConnection, id: i32, new_quantity: i32) -> Result<products::Model, AppError> {
        let product = Self::find_by_id(db, id).await?;
        let mut product: products::ActiveModel = product.into();
        product.stock_quantity = Set(new_quantity);
        Ok(product.update(db).await?)
    }
}
