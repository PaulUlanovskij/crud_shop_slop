use anyhow::Result;
use sea_orm::{ModelTrait};
use sea_orm::{
    ActiveModelTrait, DatabaseConnection, EntityTrait, QueryOrder, Set
};
use chrono::Utc;
use crate::db::{Customers, Orders, customers, orders};
use crate::dtos::*;
use crate::error::AppError;

pub struct CustomerService;

impl CustomerService {
    pub async fn find_all(db: &DatabaseConnection) -> Result<Vec<customers::Model>, AppError> {
        let customers = Customers::find()
            .order_by_asc(customers::Column::LastName)
            .order_by_asc(customers::Column::FirstName)
            .all(db)
            .await?;
        Ok(customers)
    }

    pub async fn find_by_id(db: &DatabaseConnection, id: i32) -> Result<customers::Model, AppError> {
        Customers::find_by_id(id)
            .one(db)
            .await?
            .ok_or(AppError::NotFound)
    }

    pub async fn create(db: &DatabaseConnection, dto: CustomerCreate) -> Result<customers::Model, AppError> {
        let customer = customers::ActiveModel {
            first_name: Set(dto.first_name),
            last_name: Set(dto.last_name),
            email: Set(dto.email),
            phone: Set(dto.phone),
            address: Set(dto.address),
            registration_date: Set(Utc::now()),
            ..Default::default()
        };
        
        Ok(customer.insert(db).await?)
    }

    pub async fn update(db: &DatabaseConnection, id: i32, dto: CustomerUpdate) -> Result<customers::Model, AppError> {
        let customer = Self::find_by_id(db, id).await?;
        
        let mut customer: customers::ActiveModel = customer.into();
        
        if let Some(first_name) = dto.first_name {
            customer.first_name = Set(first_name);
        }
        if let Some(last_name) = dto.last_name {
            customer.last_name = Set(last_name);
        }
        if let Some(email) = dto.email {
            customer.email = Set(email);
        }
        if let Some(phone) = dto.phone {
            customer.phone = Set(Some(phone));
        }
        if let Some(address) = dto.address {
            customer.address = Set(Some(address));
        }
        
        Ok(customer.update(db).await?)
    }

    pub async fn delete(db: &DatabaseConnection, id: i32) -> Result<(), AppError> {
        let customer = Self::find_by_id(db, id).await?;
        let customer: customers::ActiveModel = customer.into();
        customer.delete(db).await?;
        Ok(())
    }

    pub async fn find_with_orders(db: &DatabaseConnection, id: i32) -> Result<(customers::Model, Vec<orders::Model>), AppError> {
        let customer = Self::find_by_id(db, id).await?;
        let orders = customer.find_related(Orders).all(db).await?;
        Ok((customer, orders))
    }
}
