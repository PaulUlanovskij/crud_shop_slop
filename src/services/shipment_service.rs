use anyhow::Result;
use sea_orm::{ModelTrait, QueryOrder, TransactionTrait};
use sea_orm::{
    ActiveModelTrait, DatabaseConnection, EntityTrait, QueryFilter, 
    ColumnTrait, Set
};
use crate::db::{ShipmentItems, Shipments, products, shipments_items, shipments};
use crate::dtos::*;
use crate::error::AppError;
use crate::services::ProductService;

pub struct ShipmentService;

impl ShipmentService {
    pub async fn find_all(db: &DatabaseConnection) -> Result<Vec<shipments::Model>, AppError> {
        let shipments = Shipments::find()
            .order_by_desc(shipments::Column::ShipmentDate)
            .all(db)
            .await?;
        Ok(shipments)
    }

    pub async fn find_by_id(db: &DatabaseConnection, id: i32) -> Result<shipments::Model, AppError> {
        Shipments::find_by_id(id)
            .one(db)
            .await?
            .ok_or(AppError::NotFound)
    }

    pub async fn create(db: &DatabaseConnection, dto: ShipmentCreate) -> Result<shipments::Model, AppError> {
        let txn = db.begin().await?;
        
        // Create the shipment
        let shipment = shipments::ActiveModel {
            supplier_id: Set(dto.supplier_id),
            shipment_date: Set(dto.shipment_date),
            expected_delivery_date: Set(dto.expected_delivery_date),
            status: Set(dto.status.unwrap_or_else(|| "in_transit".to_string())),
            total_cost: Set(dto.total_cost),
            ..Default::default()
        };
        
        let shipment = shipment.insert(&txn).await?;
        
        // Create shipment items
        for item in dto.items {
            let shipment_item = shipments_items::ActiveModel {
                shipment_id: Set(shipment.shipment_id),
                product_id: Set(item.product_id),
                quantity: Set(item.quantity),
                unit_cost: Set(item.unit_cost),
                ..Default::default()
            };
            shipment_item.insert(&txn).await?;
            
            // Update product stock
            let product = ProductService::find_by_id(&db, item.product_id).await?;
            let new_stock = product.stock_quantity + item.quantity;
            
            let mut product: products::ActiveModel = product.into();
            product.stock_quantity = Set(new_stock);
            product.update(&txn).await?;
        }
        
        txn.commit().await?;
        Ok(shipment)
    }

    pub async fn update(db: &DatabaseConnection, id: i32, dto: ShipmentUpdate) -> Result<shipments::Model, AppError> {
        let shipment = Self::find_by_id(db, id).await?;
        
        let mut shipment: shipments::ActiveModel = shipment.into();
        
        if let Some(status) = dto.status {
            shipment.status = Set(status);
        }
        if let Some(expected_delivery_date) = dto.expected_delivery_date {
            shipment.expected_delivery_date = Set(expected_delivery_date);
        }
        if let Some(total_cost) = dto.total_cost {
            shipment.total_cost = Set(total_cost);
        }
        
        // Handle items update if provided
        if let Some(items) = dto.items {
            let txn = db.begin().await?;
            
            // Delete existing items
            ShipmentItems::delete_many()
                .filter(shipments_items::Column::ShipmentId.eq(id))
                .exec(&txn)
                .await?;
            
            // Add new items
            for item in items {
                let shipment_item = shipments_items::ActiveModel {
                    shipment_id: Set(id),
                    product_id: Set(item.product_id),
                    quantity: Set(item.quantity),
                    unit_cost: Set(item.unit_cost),
                    ..Default::default()
                };
                shipment_item.insert(&txn).await?;
            }
            
            txn.commit().await?;
        }
        
        Ok(shipment.update(db).await?)
    }

    pub async fn delete(db: &DatabaseConnection, id: i32) -> Result<(), AppError> {
        let shipment = Self::find_by_id(db, id).await?;
        let shipment: shipments::ActiveModel = shipment.into();
        shipment.delete(db).await?;
        Ok(())
    }

    pub async fn find_with_details(db: &DatabaseConnection, id: i32) -> Result<(shipments::Model, Vec<shipments_items::Model>), AppError> {
        let shipment = Self::find_by_id(db, id).await?;
        let items = shipment.find_related(ShipmentItems).all(db).await?;
        Ok((shipment, items))
    }
}
