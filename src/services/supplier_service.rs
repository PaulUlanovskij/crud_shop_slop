use anyhow::Result;
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryOrder 
};
use crate::db::{Products, Suppliers, products, suppliers};
use crate::dtos::*;
use crate::error::AppError;


pub struct SupplierService;

impl SupplierService {
    pub async fn find_all(db: &DatabaseConnection) -> Result<Vec<suppliers::Model>, AppError> {
        let suppliers = Suppliers::find()
            .order_by_asc(suppliers::Column::CompanyName)
            .all(db)
            .await?;
        Ok(suppliers)
    }

    pub async fn find_by_id(db: &DatabaseConnection, id: i32) -> Result<suppliers::Model, AppError> {
        Suppliers::find_by_id(id)
            .one(db)
            .await?
            .ok_or(AppError::NotFound)
    }

    pub async fn create(db: &DatabaseConnection, dto: SupplierCreate) -> Result<suppliers::Model, AppError> {
        let supplier = suppliers::ActiveModel {
            company_name: Set(dto.company_name),
            contact_name: Set(dto.contact_name),
            email: Set(dto.email),
            phone: Set(dto.phone),
            address: Set(dto.address),
            ..Default::default()
        };
        
        Ok(supplier.insert(db).await?)
    }

    pub async fn update(db: &DatabaseConnection, id: i32, dto: SupplierUpdate) -> Result<suppliers::Model, AppError> {
        let supplier = Self::find_by_id(db, id).await?;
        
        let mut supplier: suppliers::ActiveModel = supplier.into();
        
        if let Some(company_name) = dto.company_name {
            supplier.company_name = Set(company_name);
        }
        if let Some(contact_name) = dto.contact_name {
            supplier.contact_name = Set(Some(contact_name));
        }
        if let Some(email) = dto.email {
            supplier.email = Set(Some(email));
        }
        if let Some(phone) = dto.phone {
            supplier.phone = Set(Some(phone));
        }
        if let Some(address) = dto.address {
            supplier.address = Set(Some(address));
        }
        
        Ok(supplier.update(db).await?)
    }

    pub async fn delete(db: &DatabaseConnection, id: i32) -> Result<(), AppError> {
        let supplier = Self::find_by_id(db, id).await?;
        let supplier: suppliers::ActiveModel = supplier.into();
        supplier.delete(db).await?;
        Ok(())
    }

    pub async fn find_with_products(db: &DatabaseConnection, id: i32) -> Result<(suppliers::Model, Vec<products::Model>), AppError> {
        let supplier = Self::find_by_id(db, id).await?;
        let products = supplier.find_related(Products).all(db).await?;
        Ok((supplier, products))
    }
}

