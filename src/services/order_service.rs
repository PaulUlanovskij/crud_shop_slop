use anyhow::Result;
use sea_orm::{ModelTrait, QueryOrder, TransactionTrait};
use sea_orm::{
    ActiveModelTrait, DatabaseConnection, EntityTrait, QueryFilter, 
    ColumnTrait, Set
};
use chrono::Utc;
use crate::db::{self, OrderItems, Orders};
use crate::dtos::*;
use crate::error::AppError;
use crate::services::ProductService;

pub struct OrderService;

impl OrderService {
    pub async fn find_all(db: &DatabaseConnection) -> Result<Vec<db::orders::Model>, AppError> {
        let orders = Orders::find()
            .order_by_desc(db::orders::Column::OrderDate)
            .all(db)
            .await?;
        Ok(orders)
    }

    pub async fn find_by_id(db: &DatabaseConnection, id: i32) -> Result<db::orders::Model, AppError> {
        Orders::find_by_id(id)
            .one(db)
            .await?
            .ok_or(AppError::NotFound)
    }

    pub async fn create(db: &DatabaseConnection, dto: OrderCreate) -> Result<db::orders::Model, AppError> {
        let txn = db.begin().await?;
        
        // Create the order
        let order = db::orders::ActiveModel {
            customer_id: Set(dto.customer_id),
            order_date: Set(Utc::now()),
            status: Set(dto.status.unwrap_or_else(|| "pending".to_string())),
            total_amount: Set(dto.total_amount),
            shipping_address: Set(dto.shipping_address),
            ..Default::default()
        };
        
        let order = order.insert(&txn).await?;
        
        // Create order items
        for item in dto.items {
            let order_item = db::order_items::ActiveModel {
                order_id: Set(order.order_id),
                product_id: Set(item.product_id),
                quantity: Set(item.quantity),
                unit_price: Set(item.unit_price),
                ..Default::default()
            };
            order_item.insert(&txn).await?;
            
            // Update product stock
            let product = ProductService::find_by_id(&db, item.product_id).await?;
            let new_stock = product.stock_quantity - item.quantity;
            if new_stock < 0 {
                return Err(AppError::Validation("Insufficient stock".to_string()));
            }
            
            let mut product: db::products::ActiveModel = product.into();
            product.stock_quantity = Set(new_stock);
            product.update(&txn).await?;
        }
        
        txn.commit().await?;
        Ok(order)
    }

    pub async fn update(db: &DatabaseConnection, id: i32, dto: OrderUpdate) -> Result<db::orders::Model, AppError> {
        let order = Self::find_by_id(db, id).await?;
        
        let mut order: db::orders::ActiveModel = order.into();
        
        if let Some(status) = dto.status {
            order.status = Set(status);
        }
        if let Some(total_amount) = dto.total_amount {
            order.total_amount = Set(total_amount);
        }
        if let Some(shipping_address) = dto.shipping_address {
            order.shipping_address = Set(shipping_address);
        }
        
        // Handle items update if provided
        if let Some(items) = dto.items {
            let txn = db.begin().await?;
            
            // Delete existing items
            OrderItems::delete_many()
                .filter(db::order_items::Column::OrderId.eq(id))
                .exec(&txn)
                .await?;
            
            // Add new items
            for item in items {
                let order_item = db::order_items::ActiveModel {
                    order_id: Set(id),
                    product_id: Set(item.product_id),
                    quantity: Set(item.quantity),
                    unit_price: Set(item.unit_price),
                    ..Default::default()
                };
                order_item.insert(&txn).await?;
            }
            
            txn.commit().await?;
        }
        
        Ok(order.update(db).await?)
    }

    pub async fn delete(db: &DatabaseConnection, id: i32) -> Result<(), AppError> {
        let order = Self::find_by_id(db, id).await?;
        let order: db::orders::ActiveModel = order.into();
        order.delete(db).await?;
        Ok(())
    }

    pub async fn find_with_details(db: &DatabaseConnection, id: i32) -> Result<(db::orders::Model, Vec<db::order_items::Model>), AppError> {
        let order = Self::find_by_id(db, id).await?;
        let items = order.find_related(OrderItems).all(db).await?;
        Ok((order, items))
    }
}
