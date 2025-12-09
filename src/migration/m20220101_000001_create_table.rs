pub use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute_unprepared(
            "
CREATE TABLE Categories (
    category_id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL UNIQUE,
    description TEXT
);

CREATE TABLE Suppliers (
    supplier_id SERIAL PRIMARY KEY,
    company_name VARCHAR(150) NOT NULL,
    contact_name VARCHAR(100),
    email VARCHAR(100) CHECK (email ~* '^[A-Za-z0-9._+%-]+@[A-Za-z0-9.-]+[.][A-Za-z]+$'),
    phone VARCHAR(20),
    address TEXT
);

CREATE TABLE Customers (
    customer_id SERIAL PRIMARY KEY,
    first_name VARCHAR(50) NOT NULL,
    last_name VARCHAR(50) NOT NULL,
    email VARCHAR(100) NOT NULL UNIQUE CHECK (email ~* '^[A-Za-z0-9._+%-]+@[A-Za-z0-9.-]+[.][A-Za-z]+$'),
    phone VARCHAR(20),
    registration_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    address TEXT
);

CREATE TABLE Products (
    product_id SERIAL PRIMARY KEY,
    name VARCHAR(200) NOT NULL,
    description TEXT,
    price DECIMAL(10,2) NOT NULL CHECK (price >= 0),
    stock_quantity INT NOT NULL DEFAULT 0 CHECK (stock_quantity >= 0),
    category_id INT NOT NULL,
    supplier_id INT NOT NULL,
    CONSTRAINT fk_product_category FOREIGN KEY (category_id)
        REFERENCES Categories (category_id)
        ON DELETE RESTRICT,
    CONSTRAINT fk_product_supplier FOREIGN KEY (supplier_id)
        REFERENCES Suppliers (supplier_id)
        ON DELETE RESTRICT
);

CREATE TABLE Orders (
    order_id SERIAL PRIMARY KEY,
    customer_id INT NOT NULL,
    order_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'pending' CHECK (status IN ('pending', 'confirmed', 'shipped', 'delivered', 'cancelled')),
    total_amount DECIMAL(10,2) NOT NULL CHECK (total_amount >= 0),
    shipping_address TEXT NOT NULL,
    CONSTRAINT fk_order_customer FOREIGN KEY (customer_id)
        REFERENCES Customers (customer_id)
        ON DELETE CASCADE
);

CREATE TABLE OrderItems (
    order_id INT NOT NULL,
    product_id INT NOT NULL,
    quantity INT NOT NULL CHECK (quantity > 0),
    unit_price DECIMAL(10,2) NOT NULL CHECK (unit_price >= 0),
    PRIMARY KEY (order_id, product_id),
    CONSTRAINT fk_orderitem_order FOREIGN KEY (order_id)
        REFERENCES Orders (order_id)
        ON DELETE CASCADE,
    CONSTRAINT fk_orderitem_product FOREIGN KEY (product_id)
        REFERENCES Products (product_id)
        ON DELETE RESTRICT
);

CREATE TABLE Shipments (
    shipment_id SERIAL PRIMARY KEY,
    supplier_id INT NOT NULL,
    shipment_date DATE NOT NULL,
    expected_delivery_date DATE NOT NULL CHECK (expected_delivery_date >= shipment_date),
    status VARCHAR(20) NOT NULL DEFAULT 'in_transit' CHECK (status IN ('in_transit', 'delivered', 'cancelled')),
    total_cost DECIMAL(10,2) NOT NULL CHECK (total_cost >= 0),
    CONSTRAINT fk_shipment_supplier FOREIGN KEY (supplier_id)
        REFERENCES Suppliers (supplier_id)
        ON DELETE CASCADE
);

CREATE TABLE ShipmentItems (
    shipment_id INT NOT NULL,
    product_id INT NOT NULL,
    quantity INT NOT NULL CHECK (quantity > 0),
    unit_cost DECIMAL(10,2) NOT NULL CHECK (unit_cost >= 0),
    PRIMARY KEY (shipment_id, product_id),
    CONSTRAINT fk_shipmentitem_shipment FOREIGN KEY (shipment_id)
        REFERENCES Shipments (shipment_id)
        ON DELETE CASCADE,
    CONSTRAINT fk_shipmentitem_product FOREIGN KEY (product_id)
        REFERENCES Products (product_id)
        ON DELETE RESTRICT
);"
        ).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute_unprepared(
            "
DROP TABLE IF EXISTS ShipmentItems CASCADE;
DROP TABLE IF EXISTS OrderItems CASCADE;
DROP TABLE IF EXISTS Shipments CASCADE;
DROP TABLE IF EXISTS Orders CASCADE;
DROP TABLE IF EXISTS Products CASCADE;
DROP TABLE IF EXISTS Customers CASCADE;
DROP TABLE IF EXISTS Suppliers CASCADE;
DROP TABLE IF EXISTS Categories CASCADE;
",
        )
        .await?;
        Ok(())
    }
}
