use sea_orm::entity::prelude::*;

#[derive(Default, Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "order_items")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub order_id: i32,
    #[sea_orm(primary_key)]
    pub product_id: i32,
    pub quantity: i32,
    pub unit_price: Decimal,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::orders::Entity",
        from = "Column::OrderId",
        to = "super::orders::Column::OrderId"
    )]
    Order,
    #[sea_orm(
        belongs_to = "super::products::Entity",
        from = "Column::ProductId",
        to = "super::products::Column::ProductId"
    )]
    Product,
}

impl Related<super::products::Entity> for Entity {
    fn to() -> RelationDef {
        super::products::Relation::OrderItem.def()
    }
}
impl Related<super::orders::Entity> for Entity {
    fn to() -> RelationDef {
        super::orders::Relation::OrderItem.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
