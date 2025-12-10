pub use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute_unprepared(
            "
-- Insert mock data for Categories
INSERT INTO Categories (name, description) VALUES
('Electronics', 'Electronic devices and accessories'),
('Clothing', 'Apparel and fashion items'),
('Books', 'Books, magazines, and publications'),
('Home & Garden', 'Home improvement and garden supplies'),
('Sports & Outdoors', 'Sporting goods and outdoor equipment'),
('Toys & Games', 'Toys, games, and entertainment'),
('Beauty & Health', 'Beauty products and health supplies'),
('Automotive', 'Auto parts and accessories'),
('Food & Beverages', 'Food items and beverages'),
('Office Supplies', 'Office equipment and stationery');

-- Insert mock data for Suppliers
INSERT INTO Suppliers (company_name, contact_name, email, phone, address) VALUES
('TechCorp Inc.', 'John Smith', 'john@techcorp.com', '+1-555-123-4567', '123 Tech St, San Francisco, CA'),
('FashionStyle Ltd', 'Emma Johnson', 'emma@fashionstyle.com', '+44-20-7123-4567', '45 Fashion Ave, London, UK'),
('BookWorld Publishers', 'Robert Chen', 'robert@bookworld.com', '+1-555-987-6543', '789 Library Rd, New York, NY'),
('HomeGoods Supply', 'Maria Garcia', 'maria@homegoods.com', '+1-555-456-7890', '321 Home St, Chicago, IL'),
('SportPro Equipment', 'David Wilson', 'david@sportpro.com', '+61-2-9876-5432', '12 Stadium Rd, Sydney, Australia'),
('ToyMasters Co.', 'Sarah Miller', 'sarah@toymasters.com', '+1-555-234-5678', '567 Playground Blvd, Los Angeles, CA'),
('HealthBeauty Ltd', 'Lisa Wang', 'lisa@healthbeauty.com', '+86-10-8765-4321', '89 Wellness St, Beijing, China'),
('AutoParts Direct', 'Mike Brown', 'mike@autoparts.com', '+1-555-345-6789', '234 Garage Rd, Detroit, MI'),
('FreshFoods Market', 'Anna Schmidt', 'anna@freshfoods.com', '+49-30-1234-5678', '56 Market St, Berlin, Germany'),
('OfficePlus Corp', 'Tom Davis', 'tom@officeplus.com', '+1-555-876-5432', '987 Business Ave, Seattle, WA');

-- Insert mock data for Customers
INSERT INTO Customers (first_name, last_name, email, phone, registration_date, address) VALUES
('Alice', 'Johnson', 'alice.johnson@email.com', '+1-555-111-2233', '2023-01-15 09:30:00', '123 Main St, Anytown, USA'),
('Bob', 'Williams', 'bob.williams@email.com', '+1-555-222-3344', '2023-02-20 14:15:00', '456 Oak Ave, Somewhere, USA'),
('Charlie', 'Brown', 'charlie.brown@email.com', '+1-555-333-4455', '2023-03-10 11:45:00', '789 Pine Rd, Nowhere, USA'),
('Diana', 'Miller', 'diana.miller@email.com', '+1-555-444-5566', '2023-04-05 16:20:00', '321 Elm St, Anywhere, USA'),
('Edward', 'Davis', 'edward.davis@email.com', '+1-555-555-6677', '2023-05-12 10:00:00', '654 Maple Dr, Everywhere, USA'),
('Fiona', 'Garcia', 'fiona.garcia@email.com', '+1-555-666-7788', '2023-06-18 13:30:00', '987 Birch Ln, Someplace, USA'),
('George', 'Martinez', 'george.martinez@email.com', '+1-555-777-8899', '2023-07-22 08:45:00', '147 Cedar Ave, Noplace, USA'),
('Hannah', 'Lee', 'hannah.lee@email.com', '+1-555-888-9900', '2023-08-30 12:15:00', '258 Spruce Blvd, Anyplace, USA'),
('Ian', 'Taylor', 'ian.taylor@email.com', '+1-555-999-0011', '2023-09-14 15:40:00', '369 Willow Way, Everyplace, USA'),
('Julia', 'Anderson', 'julia.anderson@email.com', '+1-555-000-1122', '2023-10-25 17:50:00', '741 Ash Ct, Thisplace, USA'),
('Kevin', 'Thomas', 'kevin.thomas@email.com', '+1-555-112-2334', '2023-11-03 09:10:00', '852 Poplar St, Thatplace, USA'),
('Laura', 'Jackson', 'laura.jackson@email.com', '+1-555-223-3445', '2023-12-08 14:25:00', '963 Fir Rd, Otherplace, USA');

-- Insert mock data for Products
INSERT INTO Products (name, description, price, stock_quantity, category_id, supplier_id) VALUES
('Smartphone X', 'Latest smartphone with advanced features', 799.99, 50, 1, 1),
('Laptop Pro', 'High-performance laptop for professionals', 1299.99, 30, 1, 1),
('Wireless Headphones', 'Noise-cancelling Bluetooth headphones', 199.99, 100, 1, 1),
('Men''s Casual Shirt', 'Comfortable cotton shirt for daily wear', 29.99, 200, 2, 2),
('Women''s Dress', 'Elegant evening dress', 89.99, 75, 2, 2),
('Programming Book', 'Complete guide to modern programming', 49.99, 150, 3, 3),
('Mystery Novel', 'Bestselling thriller novel', 14.99, 300, 3, 3),
('Garden Hose', '50ft durable garden hose', 39.99, 80, 4, 4),
('Lawn Mower', 'Electric lawn mower with bag', 249.99, 25, 4, 4),
('Basketball', 'Official size basketball', 24.99, 120, 5, 5),
('Tent 4-Person', 'Waterproof camping tent', 149.99, 40, 5, 5),
('Board Game', 'Family strategy board game', 34.99, 90, 6, 6),
('Facial Cream', 'Anti-aging moisturizing cream', 59.99, 150, 7, 7),
('Car Battery', '12V automotive battery', 129.99, 60, 8, 8),
('Coffee Beans', 'Premium Arabica coffee beans', 19.99, 200, 9, 9),
('Notebook Set', 'Pack of 5 premium notebooks', 12.99, 250, 10, 10);

-- Insert mock data for Orders
INSERT INTO Orders (customer_id, order_date, status, total_amount, shipping_address) VALUES
(1, '2024-01-10 10:30:00', 'delivered', 849.98, '123 Main St, Anytown, USA'),
(2, '2024-01-15 14:45:00', 'shipped', 1549.98, '456 Oak Ave, Somewhere, USA'),
(3, '2024-02-05 09:15:00', 'confirmed', 74.97, '789 Pine Rd, Nowhere, USA'),
(4, '2024-02-20 16:20:00', 'pending', 239.97, '321 Elm St, Anywhere, USA'),
(5, '2024-03-12 11:00:00', 'delivered', 199.99, '654 Maple Dr, Everywhere, USA'),
(6, '2024-03-25 13:45:00', 'cancelled', 89.99, '987 Birch Ln, Someplace, USA'),
(7, '2024-04-03 08:30:00', 'delivered', 149.99, '147 Cedar Ave, Noplace, USA'),
(8, '2024-04-18 12:15:00', 'shipped', 49.98, '258 Spruce Blvd, Anyplace, USA'),
(9, '2024-05-22 15:50:00', 'confirmed', 374.97, '369 Willow Way, Everyplace, USA'),
(10, '2024-06-07 17:30:00', 'delivered', 129.99, '741 Ash Ct, Thisplace, USA');

-- Insert mock data for OrderItems
INSERT INTO OrderItems (order_id, product_id, quantity, unit_price) VALUES
(1, 1, 1, 799.99),
(1, 4, 1, 29.99),
(2, 2, 1, 1299.99),
(2, 3, 1, 199.99),
(2, 6, 1, 49.99),
(3, 7, 5, 14.99),
(4, 5, 1, 89.99),
(4, 10, 2, 24.99),
(4, 12, 3, 34.99),
(5, 3, 1, 199.99),
(6, 5, 1, 89.99),
(7, 11, 1, 149.99),
(8, 16, 2, 12.99),
(8, 13, 1, 59.99),
(9, 9, 1, 249.99),
(9, 8, 1, 39.99),
(9, 15, 3, 19.99),
(10, 14, 1, 129.99);

-- Insert mock data for Shipments
INSERT INTO Shipments (supplier_id, shipment_date, expected_delivery_date, status, total_cost) VALUES
(1, '2024-01-05', '2024-01-12', 'delivered', 15000.00),
(2, '2024-01-10', '2024-01-20', 'delivered', 5000.00),
(3, '2024-02-01', '2024-02-15', 'delivered', 3000.00),
(4, '2024-02-15', '2024-02-25', 'delivered', 8000.00),
(5, '2024-03-01', '2024-03-10', 'delivered', 6000.00),
(6, '2024-03-10', '2024-03-20', 'in_transit', 4000.00),
(7, '2024-03-20', '2024-03-30', 'in_transit', 7000.00),
(8, '2024-04-05', '2024-04-15', 'cancelled', 2000.00),
(9, '2024-04-10', '2024-04-20', 'delivered', 9000.00),
(10, '2024-05-01', '2024-05-10', 'in_transit', 3500.00);

-- Insert mock data for ShipmentItems
INSERT INTO ShipmentItems (shipment_id, product_id, quantity, unit_cost) VALUES
(1, 1, 20, 500.00),
(1, 2, 10, 800.00),
(1, 3, 50, 120.00),
(2, 4, 100, 15.00),
(2, 5, 50, 40.00),
(3, 6, 60, 25.00),
(3, 7, 100, 8.00),
(4, 8, 100, 25.00),
(4, 9, 20, 150.00),
(5, 10, 150, 15.00),
(5, 11, 30, 90.00),
(6, 12, 100, 20.00),
(7, 13, 100, 35.00),
(8, 14, 15, 80.00),
(9, 15, 200, 10.00),
(9, 16, 100, 7.00),
(10, 16, 150, 7.00),
(10, 13, 50, 35.00);

-- Update product stock quantities based on shipments
UPDATE Products p
SET stock_quantity = p.stock_quantity + sub.total_quantity
FROM (
    SELECT si.product_id, SUM(si.quantity) as total_quantity
    FROM ShipmentItems si
    JOIN Shipments s ON si.shipment_id = s.shipment_id
    WHERE s.status != 'cancelled'
    GROUP BY si.product_id
) sub
WHERE p.product_id = sub.product_id;

-- Update product stock quantities based on completed orders
UPDATE Products p
SET stock_quantity = p.stock_quantity - sub.total_quantity
FROM (
    SELECT oi.product_id, SUM(oi.quantity) as total_quantity
    FROM OrderItems oi
    JOIN Orders o ON oi.order_id = o.order_id
    WHERE o.status IN ('delivered', 'shipped', 'confirmed')
    GROUP BY oi.product_id
) sub
WHERE p.product_id = sub.product_id;
;"
        ).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute_unprepared(
            "
DELETE FROM ShipmentItems;
DELETE FROM OrderItems;
DELETE FROM Shipments;
DELETE FROM Orders;
DELETE FROM Products;
DELETE FROM Customers;
DELETE FROM Suppliers;
DELETE FROM Categories;

-- Reset all sequences
ALTER SEQUENCE categories_category_id_seq RESTART WITH 1;
ALTER SEQUENCE suppliers_supplier_id_seq RESTART WITH 1;
ALTER SEQUENCE customers_customer_id_seq RESTART WITH 1;
ALTER SEQUENCE products_product_id_seq RESTART WITH 1;
ALTER SEQUENCE orders_order_id_seq RESTART WITH 1;
ALTER SEQUENCE shipments_shipment_id_seq RESTART WITH 1;
",
        )
        .await?;
        Ok(())
    }
}
