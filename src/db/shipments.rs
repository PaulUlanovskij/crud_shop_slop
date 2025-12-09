use sea_orm::entity::prelude::*;
use chrono::NaiveDate;

#[derive(Default, Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "shipments")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub shipment_id: i32,
    pub supplier_id: i32,
    pub shipment_date: NaiveDate,
    pub expected_delivery_date: NaiveDate,
    pub status: String,
    pub total_cost: Decimal,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::suppliers::Entity",
        from = "Column::SupplierId",
        to = "super::suppliers::Column::SupplierId"
    )]
    Supplier,
    #[sea_orm(has_many = "super::shipments_items::Entity")]
    ShipmentItem, 
}

impl Related<super::suppliers::Entity> for Entity {
    fn to() -> RelationDef {
        super::suppliers::Relation::Shipment.def()
    }
}
impl Related<super::shipments_items::Entity> for Entity {
    fn to() -> RelationDef {
        super::shipments_items::Relation::Shipment.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
