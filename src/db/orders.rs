use sea_orm::entity::prelude::*;
use chrono::{DateTime, Utc};

#[derive(Default, Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "orders")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub order_id: i32,
    pub customer_id: i32,
    pub order_date: DateTime<Utc>,
    pub status: String,
    pub total_amount: Decimal,
    pub shipping_address: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::customers::Entity",
        from = "Column::CustomerId",
        to = "super::customers::Column::CustomerId"
    )]
    Customer,
    #[sea_orm(has_many = "super::order_items::Entity")]
    OrderItem,
}

impl Related<super::customers::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Customer.def()
    }
}
impl Related<super::order_items::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::OrderItem.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
