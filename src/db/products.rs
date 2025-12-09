use sea_orm::entity::prelude::*;

#[derive(Default, Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "products")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub product_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub price: Decimal,
    pub stock_quantity: i32,
    pub category_id: i32,
    pub supplier_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::categories::Entity",
        from = "Column::CategoryId",
        to = "super::categories::Column::CategoryId"
    )]
    Category,
    #[sea_orm(
        belongs_to = "super::suppliers::Entity",
        from = "Column::SupplierId",
        to = "super::suppliers::Column::SupplierId"
    )]
    Supplier,
    #[sea_orm(has_many = "super::order_items::Entity")]
    OrderItem,
    #[sea_orm(has_many = "super::shipments_items::Entity")]
    ShipmentItem, 
}

impl Related<super::categories::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Category.def()
    }
}

impl Related<super::suppliers::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Supplier.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
