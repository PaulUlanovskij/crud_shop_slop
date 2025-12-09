use anyhow::Result;
use sea_orm::{
    ActiveModelTrait, DatabaseConnection, EntityTrait, QueryOrder, Set
};
use crate::db::{Categories, categories};
use crate::dtos::*;
use crate::error::AppError;


pub struct CategoryService;

impl CategoryService {
    pub async fn find_all(db: &DatabaseConnection) -> Result<Vec<categories::Model>, AppError> {
        let categories = Categories::find()
            .order_by_asc(categories::Column::Name)
            .all(db)
            .await?;
        Ok(categories)
    }

    pub async fn find_by_id(db: &DatabaseConnection, id: i32) -> Result<categories::Model, AppError> {
        Categories::find_by_id(id)
            .one(db)
            .await?
            .ok_or(AppError::NotFound)
    }

    pub async fn create(db: &DatabaseConnection, dto: CategoryCreate) -> Result<categories::Model, AppError> {
        let category = categories::ActiveModel {
            name: Set(dto.name),
            description: Set(dto.description),
            ..Default::default()
        };
        
        Ok(category.insert(db).await?)
    }

    pub async fn update(db: &DatabaseConnection, id: i32, dto: CategoryUpdate) -> Result<categories::Model, AppError> {
        let category = Self::find_by_id(db, id).await?;
        
        let mut category: categories::ActiveModel = category.into();
        
        if let Some(name) = dto.name {
            category.name = Set(name);
        }
        if let Some(description) = dto.description {
            category.description = Set(Some(description));
        }
        
        Ok(category.update(db).await?)
    }

    pub async fn delete(db: &DatabaseConnection, id: i32) -> Result<(), AppError> {
        let category = Self::find_by_id(db, id).await?;
        let category: categories::ActiveModel = category.into();
        category.delete(db).await?;
        Ok(())
    }
}

