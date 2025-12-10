use sea_orm::entity::prelude::*;

#[derive(Default, Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "shipmentitems")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub shipment_id: i32,
    #[sea_orm(primary_key)]
    pub product_id: i32,
    pub quantity: i32,
    pub unit_cost: Decimal,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::shipments::Entity",
        from = "Column::ShipmentId",
        to = "super::shipments::Column::ShipmentId"
    )]
    Shipment,
    #[sea_orm(
        belongs_to = "super::products::Entity",
        from = "Column::ProductId",
        to = "super::products::Column::ProductId"
    )]
    Product,
}

impl Related<super::products::Entity> for Entity {
    fn to() -> RelationDef {
        super::products::Relation::ShipmentItem.def()
    }
}

impl Related<super::shipments::Entity> for Entity {
    fn to() -> RelationDef {
        super::shipments::Relation::ShipmentItem.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
