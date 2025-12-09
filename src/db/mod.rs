pub mod categories;
pub mod suppliers;
pub mod customers;
pub mod products;
pub mod orders;
pub mod order_items;
pub mod shipments;
pub mod shipments_items;

pub use categories::Entity as Categories;
pub use suppliers::Entity as Suppliers;
pub use customers::Entity as Customers;
pub use products::Entity as Products;
pub use orders::Entity as Orders;
pub use order_items::Entity as OrderItems;
pub use shipments::Entity as Shipments;
pub use shipments_items::Entity as ShipmentItems;

use sea_orm::{Database, DatabaseConnection, DbErr};
use crate::migration::{Migrator, MigratorTrait}; 

pub async fn connect_database(connection_string: impl AsRef<str>) -> Result<DatabaseConnection, DbErr> {
    Database::connect(connection_string.as_ref()).await
}

pub async fn migrate(connection: &DatabaseConnection) -> Result<(), DbErr> {
    Migrator::up(connection, None).await?;
    Ok(())
}

pub fn connection_string<A: AsRef<str>>(
    user: impl AsRef<str>,
    password: impl AsRef<str>,
    host: impl AsRef<str>,
    port: u16,
    db_name: Option<A>,
) -> String {
    let mut base =
    format!(
        "postgres://{}:{}@{}:{}",
        user.as_ref(),
        password.as_ref(),
        host.as_ref(),
        port,
    );
    if let Some(db_name) = db_name {
        base.push_str(&format!("/{}", db_name.as_ref()));
    }
    base
}
