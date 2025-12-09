// API Configuration
const API_BASE_URL = 'http://localhost:8000/api';
let currentEditingId = null;
let currentEditingType = null;

// DOM Elements
const tabs = document.querySelectorAll('.tab');
const tabContents = document.querySelectorAll('.tab-content');
const notification = document.getElementById('notification');

// Initialize the application
document.addEventListener('DOMContentLoaded', function() {
  // Set up tab switching
  tabs.forEach(tab => {
    tab.addEventListener('click', () => {
      const tabId = tab.getAttribute('data-tab');

      // Update active tab
      tabs.forEach(t => t.classList.remove('active'));
      tab.classList.add('active');

      // Update active content
      tabContents.forEach(content => content.classList.remove('active'));
      document.getElementById(`${tabId}-tab`).classList.add('active');

      // Load data for the active tab
      loadTabData(tabId);
    });
  });

  // Set up product form handlers
  document.getElementById('product-create-btn').addEventListener('click', createProduct);
  document.getElementById('product-update-btn').addEventListener('click', updateProduct);
  document.getElementById('product-cancel-btn').addEventListener('click', cancelEdit);

  // Set up category form handlers
  document.getElementById('category-create-btn').addEventListener('click', createCategory);
  document.getElementById('category-update-btn').addEventListener('click', updateCategory);
  document.getElementById('category-cancel-btn').addEventListener('click', cancelEditCategory);

  // Set up customer form handlers
  document.getElementById('customer-create-btn').addEventListener('click', createCustomer);
  document.getElementById('customer-update-btn').addEventListener('click', updateCustomer);
  document.getElementById('customer-cancel-btn').addEventListener('click', cancelEditCustomer);

  // Set up supplier form handlers
  document.getElementById('supplier-create-btn').addEventListener('click', createSupplier);
  document.getElementById('supplier-update-btn').addEventListener('click', updateSupplier);
  document.getElementById('supplier-cancel-btn').addEventListener('click', cancelEditSupplier);

  // Set up order form handlers
  document.getElementById('order-create-btn').addEventListener('click', createOrder);
  document.getElementById('order-update-btn').addEventListener('click', updateOrder);
  document.getElementById('order-cancel-btn').addEventListener('click', cancelEditOrder);

  // Set up shipment form handlers
  document.getElementById('shipment-create-btn').addEventListener('click', createShipment);
  document.getElementById('shipment-update-btn').addEventListener('click', updateShipment);
  document.getElementById('shipment-cancel-btn').addEventListener('click', cancelEditShipment);

  // Set current date for shipment forms
  document.getElementById('shipment-date').valueAsDate = new Date();
  const expectedDate = new Date();
  expectedDate.setDate(expectedDate.getDate() + 7);
  document.getElementById('shipment-expected-date').valueAsDate = expectedDate;

  // Load initial data for the active tab (products)
  loadTabData('products');
});

// Load data based on the active tab
async function loadTabData(tabId) {
  switch(tabId) {
    case 'products':
      await loadProducts();
      await loadCategoriesForDropdown();
      await loadSuppliersForDropdown();
      break;
    case 'categories':
      await loadCategories();
      break;
    case 'customers':
      await loadCustomers();
      break;
    case 'suppliers':
      await loadSuppliers();
      break;
    case 'orders':
      await loadOrders();
      await loadCustomersForDropdown();
      await loadProductsForOrderItems();
      break;
    case 'shipments':
      await loadShipments();
      await loadSuppliersForDropdown();
      await loadProductsForShipmentItems();
      break;
  }
}

// ========== PRODUCT FUNCTIONS ==========
  async function loadProducts() {
    try {
      showLoading('products-table-body');
      const response = await fetch(`${API_BASE_URL}/products`);
      const products = await response.json();

      const tableBody = document.getElementById('products-table-body');
      tableBody.innerHTML = '';

      if (products.length === 0) {
        tableBody.innerHTML = `
          <tr>
          <td colspan="7" class="empty-state">
          <i>📦</i>
          <p>No products found</p>
          </td>
          </tr>
          `;
        return;
      }

      products.forEach(product => {
        const row = document.createElement('tr');
        row.innerHTML = `
          <td>${product.product_id}</td>
          <td>${product.name}</td>
          <td>$${parseFloat(product.price).toFixed(2)}</td>
          <td>${product.stock_quantity}</td>
          <td>${product.category_name || 'N/A'}</td>
          <td>${product.supplier_name || 'N/A'}</td>
          <td>
          <div class="action-buttons">
          <button class="btn btn-warning btn-sm" onclick="editProduct(${product.product_id})">Edit</button>
          <button class="btn btn-danger btn-sm" onclick="deleteProduct(${product.product_id})">Delete</button>
          </div>
          </td>
          `;
        tableBody.appendChild(row);
      });
    } catch (error) {
      console.error('Error loading products:', error);
      showNotification('Failed to load products', 'error');
    }
  }

async function createProduct() {
  const name = document.getElementById('product-name').value;
  const description = document.getElementById('product-description').value;
  const price = document.getElementById('product-price').value;
  const stock = document.getElementById('product-stock').value;
  const categoryId = document.getElementById('product-category').value;
  const supplierId = document.getElementById('product-supplier').value;

  if (!name || !price || !stock || !categoryId || !supplierId) {
    showNotification('Please fill all required fields', 'error');
    return;
  }

  try {
    const response = await fetch(`${API_BASE_URL}/products`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        name,
        description,
        price: parseFloat(price),
        stock_quantity: parseInt(stock),
        category_id: parseInt(categoryId),
        supplier_id: parseInt(supplierId)
      })
    });

    if (response.ok) {
      showNotification('Product created successfully', 'success');
      resetProductForm();
      await loadProducts();
    } else {
      showNotification('Failed to create product', 'error');
    }
  } catch (error) {
    console.error('Error creating product:', error);
    showNotification('Failed to create product', 'error');
  }
}

async function editProduct(id) {
  try {
    const response = await fetch(`${API_BASE_URL}/products/${id}`);
    const product = await response.json();

    document.getElementById('product-name').value = product.name;
    document.getElementById('product-description').value = product.description || '';
    document.getElementById('product-price').value = product.price;
    document.getElementById('product-stock').value = product.stock_quantity;
    document.getElementById('product-category').value = product.category_id;
    document.getElementById('product-supplier').value = product.supplier_id;

    document.getElementById('product-create-btn').style.display = 'none';
    document.getElementById('product-update-btn').style.display = 'inline-block';
    document.getElementById('product-cancel-btn').style.display = 'inline-block';

    currentEditingId = id;
    currentEditingType = 'product';
  } catch (error) {
    console.error('Error loading product for edit:', error);
    showNotification('Failed to load product for editing', 'error');
  }
}

async function updateProduct() {
  if (!currentEditingId) return;

  const name = document.getElementById('product-name').value;
  const description = document.getElementById('product-description').value;
  const price = document.getElementById('product-price').value;
  const stock = document.getElementById('product-stock').value;
  const categoryId = document.getElementById('product-category').value;
  const supplierId = document.getElementById('product-supplier').value;

  if (!name || !price || !stock || !categoryId || !supplierId) {
    showNotification('Please fill all required fields', 'error');
    return;
  }

  try {
    const response = await fetch(`${API_BASE_URL}/products/${currentEditingId}`, {
      method: 'PUT',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        name,
        description,
        price: parseFloat(price),
        stock_quantity: parseInt(stock),
        category_id: parseInt(categoryId),
        supplier_id: parseInt(supplierId)
      })
    });

    if (response.ok) {
      showNotification('Product updated successfully', 'success');
      resetProductForm();
      await loadProducts();
    } else {
      showNotification('Failed to update product', 'error');
    }
  } catch (error) {
    console.error('Error updating product:', error);
    showNotification('Failed to update product', 'error');
  }
}

async function deleteProduct(id) {
  if (!confirm('Are you sure you want to delete this product?')) {
    return;
  }

  try {
    const response = await fetch(`${API_BASE_URL}/products/${id}`, {
      method: 'DELETE'
    });

    if (response.ok) {
      showNotification('Product deleted successfully', 'success');
      await loadProducts();
    } else {
      showNotification('Failed to delete product', 'error');
    }
  } catch (error) {
    console.error('Error deleting product:', error);
    showNotification('Failed to delete product', 'error');
  }
}

function cancelEdit() {
  resetProductForm();
}

function resetProductForm() {
  document.getElementById('product-name').value = '';
  document.getElementById('product-description').value = '';
  document.getElementById('product-price').value = '';
  document.getElementById('product-stock').value = '';
  document.getElementById('product-category').value = '';
  document.getElementById('product-supplier').value = '';

  document.getElementById('product-create-btn').style.display = 'inline-block';
  document.getElementById('product-update-btn').style.display = 'none';
  document.getElementById('product-cancel-btn').style.display = 'none';

  currentEditingId = null;
  currentEditingType = null;
}

// ========== CATEGORY FUNCTIONS ==========
  async function loadCategories() {
    try {
      showLoading('categories-table-body');
      const response = await fetch(`${API_BASE_URL}/categories`);
      const categories = await response.json();

      const tableBody = document.getElementById('categories-table-body');
      tableBody.innerHTML = '';

      if (categories.length === 0) {
        tableBody.innerHTML = `
          <tr>
          <td colspan="4" class="empty-state">
          <i>📂</i>
          <p>No categories found</p>
          </td>
          </tr>
          `;
        return;
      }

      categories.forEach(category => {
        const row = document.createElement('tr');
        row.innerHTML = `
          <td>${category.category_id}</td>
          <td>${category.name}</td>
          <td>${category.description || 'N/A'}</td>
          <td>
          <div class="action-buttons">
          <button class="btn btn-warning btn-sm" onclick="editCategory(${category.category_id})">Edit</button>
          <button class="btn btn-danger btn-sm" onclick="deleteCategory(${category.category_id})">Delete</button>
          </div>
          </td>
          `;
        tableBody.appendChild(row);
      });
    } catch (error) {
      console.error('Error loading categories:', error);
      showNotification('Failed to load categories', 'error');
    }
  }

async function loadCategoriesForDropdown() {
  try {
    const response = await fetch(`${API_BASE_URL}/categories`);
    const categories = await response.json();

    const dropdown = document.getElementById('product-category');
    dropdown.innerHTML = '<option value="">Select a category</option>';

    categories.forEach(category => {
      const option = document.createElement('option');
      option.value = category.category_id;
      option.textContent = category.name;
      dropdown.appendChild(option);
    });
  } catch (error) {
    console.error('Error loading categories for dropdown:', error);
  }
}

async function createCategory() {
  const name = document.getElementById('category-name').value;
  const description = document.getElementById('category-description').value;

  if (!name) {
    showNotification('Category name is required', 'error');
    return;
  }

  try {
    const response = await fetch(`${API_BASE_URL}/categories`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        name,
        description
      })
    });

    if (response.ok) {
      showNotification('Category created successfully', 'success');
      resetCategoryForm();
      await loadCategories();
      await loadCategoriesForDropdown(); // Refresh dropdown in products tab
    } else {
      showNotification('Failed to create category', 'error');
    }
  } catch (error) {
    console.error('Error creating category:', error);
    showNotification('Failed to create category', 'error');
  }
}

async function editCategory(id) {
  try {
    const response = await fetch(`${API_BASE_URL}/categories/${id}`);
    const category = await response.json();

    document.getElementById('category-name').value = category.name;
    document.getElementById('category-description').value = category.description || '';

    document.getElementById('category-create-btn').style.display = 'none';
    document.getElementById('category-update-btn').style.display = 'inline-block';
    document.getElementById('category-cancel-btn').style.display = 'inline-block';

    currentEditingId = id;
    currentEditingType = 'category';
  } catch (error) {
    console.error('Error loading category for edit:', error);
    showNotification('Failed to load category for editing', 'error');
  }
}

async function updateCategory() {
  if (!currentEditingId) return;

  const name = document.getElementById('category-name').value;
  const description = document.getElementById('category-description').value;

  if (!name) {
    showNotification('Category name is required', 'error');
    return;
  }

  try {
    const response = await fetch(`${API_BASE_URL}/categories/${currentEditingId}`, {
      method: 'PUT',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        name,
        description
      })
    });

    if (response.ok) {
      showNotification('Category updated successfully', 'success');
      resetCategoryForm();
      await loadCategories();
      await loadCategoriesForDropdown(); // Refresh dropdown in products tab
    } else {
      showNotification('Failed to update category', 'error');
    }
  } catch (error) {
    console.error('Error updating category:', error);
    showNotification('Failed to update category', 'error');
  }
}

async function deleteCategory(id) {
  if (!confirm('Are you sure you want to delete this category?')) {
    return;
  }

  try {
    const response = await fetch(`${API_BASE_URL}/categories/${id}`, {
      method: 'DELETE'
    });

    if (response.ok) {
      showNotification('Category deleted successfully', 'success');
      await loadCategories();
      await loadCategoriesForDropdown(); // Refresh dropdown in products tab
    } else {
      showNotification('Failed to delete category', 'error');
    }
  } catch (error) {
    console.error('Error deleting category:', error);
    showNotification('Failed to delete category', 'error');
  }
}

function cancelEditCategory() {
  resetCategoryForm();
}

function resetCategoryForm() {
  document.getElementById('category-name').value = '';
  document.getElementById('category-description').value = '';

  document.getElementById('category-create-btn').style.display = 'inline-block';
  document.getElementById('category-update-btn').style.display = 'none';
  document.getElementById('category-cancel-btn').style.display = 'none';

  currentEditingId = null;
  currentEditingType = null;
}

// ========== CUSTOMER FUNCTIONS ==========
  async function loadCustomers() {
    try {
      showLoading('customers-table-body');
      const response = await fetch(`${API_BASE_URL}/customers`);
      const customers = await response.json();

      const tableBody = document.getElementById('customers-table-body');
      tableBody.innerHTML = '';

      if (customers.length === 0) {
        tableBody.innerHTML = `
          <tr>
          <td colspan="6" class="empty-state">
          <i>👥</i>
          <p>No customers found</p>
          </td>
          </tr>
          `;
        return;
      }

      customers.forEach(customer => {
        const row = document.createElement('tr');
        const registrationDate = new Date(customer.registration_date).toLocaleDateString();
        row.innerHTML = `
          <td>${customer.customer_id}</td>
          <td>${customer.first_name} ${customer.last_name}</td>
          <td>${customer.email}</td>
          <td>${customer.phone || 'N/A'}</td>
          <td>${registrationDate}</td>
          <td>
          <div class="action-buttons">
          <button class="btn btn-warning btn-sm" onclick="editCustomer(${customer.customer_id})">Edit</button>
          <button class="btn btn-danger btn-sm" onclick="deleteCustomer(${customer.customer_id})">Delete</button>
          <button class="btn btn-primary btn-sm" onclick="viewCustomerDetails(${customer.customer_id})">Details</button>
          </div>
          </td>
          `;
        tableBody.appendChild(row);
      });
    } catch (error) {
      console.error('Error loading customers:', error);
      showNotification('Failed to load customers', 'error');
    }
  }

async function loadCustomersForDropdown() {
  try {
    const response = await fetch(`${API_BASE_URL}/customers`);
    const customers = await response.json();

    const dropdown = document.getElementById('order-customer');
    dropdown.innerHTML = '<option value="">Select a customer</option>';

    customers.forEach(customer => {
      const option = document.createElement('option');
      option.value = customer.customer_id;
      option.textContent = `${customer.first_name} ${customer.last_name} (${customer.email})`;
      dropdown.appendChild(option);
    });
  } catch (error) {
    console.error('Error loading customers for dropdown:', error);
  }
}

async function createCustomer() {
  const firstName = document.getElementById('customer-first-name').value;
  const lastName = document.getElementById('customer-last-name').value;
  const email = document.getElementById('customer-email').value;
  const phone = document.getElementById('customer-phone').value;
  const address = document.getElementById('customer-address').value;

  if (!firstName || !lastName || !email) {
    showNotification('Please fill all required fields', 'error');
    return;
  }

  try {
    const response = await fetch(`${API_BASE_URL}/customers`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        first_name: firstName,
        last_name: lastName,
        email,
        phone,
        address
      })
    });

    if (response.ok) {
      showNotification('Customer created successfully', 'success');
      resetCustomerForm();
      await loadCustomers();
      await loadCustomersForDropdown(); // Refresh dropdown in orders tab
    } else {
      showNotification('Failed to create customer', 'error');
    }
  } catch (error) {
    console.error('Error creating customer:', error);
    showNotification('Failed to create customer', 'error');
  }
}

async function editCustomer(id) {
  try {
    const response = await fetch(`${API_BASE_URL}/customers/${id}`);
    const customer = await response.json();

    document.getElementById('customer-first-name').value = customer.first_name;
    document.getElementById('customer-last-name').value = customer.last_name;
    document.getElementById('customer-email').value = customer.email;
    document.getElementById('customer-phone').value = customer.phone || '';
    document.getElementById('customer-address').value = customer.address || '';

    document.getElementById('customer-create-btn').style.display = 'none';
    document.getElementById('customer-update-btn').style.display = 'inline-block';
    document.getElementById('customer-cancel-btn').style.display = 'inline-block';

    currentEditingId = id;
    currentEditingType = 'customer';
  } catch (error) {
    console.error('Error loading customer for edit:', error);
    showNotification('Failed to load customer for editing', 'error');
  }
}

async function updateCustomer() {
  if (!currentEditingId) return;

  const firstName = document.getElementById('customer-first-name').value;
  const lastName = document.getElementById('customer-last-name').value;
  const email = document.getElementById('customer-email').value;
  const phone = document.getElementById('customer-phone').value;
  const address = document.getElementById('customer-address').value;

  if (!firstName || !lastName || !email) {
    showNotification('Please fill all required fields', 'error');
    return;
  }

  try {
    const response = await fetch(`${API_BASE_URL}/customers/${currentEditingId}`, {
      method: 'PUT',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        first_name: firstName,
        last_name: lastName,
        email,
        phone,
        address
      })
    });

    if (response.ok) {
      showNotification('Customer updated successfully', 'success');
      resetCustomerForm();
      await loadCustomers();
      await loadCustomersForDropdown(); // Refresh dropdown in orders tab
    } else {
      showNotification('Failed to update customer', 'error');
    }
  } catch (error) {
    console.error('Error updating customer:', error);
    showNotification('Failed to update customer', 'error');
  }
}

async function deleteCustomer(id) {
  if (!confirm('Are you sure you want to delete this customer?')) {
    return;
  }

  try {
    const response = await fetch(`${API_BASE_URL}/customers/${id}`, {
      method: 'DELETE'
    });

    if (response.ok) {
      showNotification('Customer deleted successfully', 'success');
      await loadCustomers();
      await loadCustomersForDropdown(); // Refresh dropdown in orders tab
    } else {
      showNotification('Failed to delete customer', 'error');
    }
  } catch (error) {
    console.error('Error deleting customer:', error);
    showNotification('Failed to delete customer', 'error');
  }
}

async function viewCustomerDetails(id) {
  try {
    const response = await fetch(`${API_BASE_URL}/customers/${id}/orders`);
    const customerWithOrders = await response.json();

    // Create a modal or expand row to show details
    alert(`Customer Details:\nName: ${customerWithOrders.first_name} ${customerWithOrders.last_name}\nEmail: ${customerWithOrders.email}\nPhone: ${customerWithOrders.phone || 'N/A'}\nTotal Orders: ${customerWithOrders.orders ? customerWithOrders.orders.length : 0}`);
  } catch (error) {
    console.error('Error loading customer details:', error);
    showNotification('Failed to load customer details', 'error');
  }
}

function cancelEditCustomer() {
  resetCustomerForm();
}

function resetCustomerForm() {
  document.getElementById('customer-first-name').value = '';
  document.getElementById('customer-last-name').value = '';
  document.getElementById('customer-email').value = '';
  document.getElementById('customer-phone').value = '';
  document.getElementById('customer-address').value = '';

  document.getElementById('customer-create-btn').style.display = 'inline-block';
  document.getElementById('customer-update-btn').style.display = 'none';
  document.getElementById('customer-cancel-btn').style.display = 'none';

  currentEditingId = null;
  currentEditingType = null;
}

// ========== SUPPLIER FUNCTIONS ==========
  async function loadSuppliers() {
    try {
      showLoading('suppliers-table-body');
      const response = await fetch(`${API_BASE_URL}/suppliers`);
      const suppliers = await response.json();

      const tableBody = document.getElementById('suppliers-table-body');
      tableBody.innerHTML = '';

      if (suppliers.length === 0) {
        tableBody.innerHTML = `
          <tr>
          <td colspan="6" class="empty-state">
          <i>🏢</i>
          <p>No suppliers found</p>
          </td>
          </tr>
          `;
        return;
      }

      suppliers.forEach(supplier => {
        const row = document.createElement('tr');
        row.innerHTML = `
          <td>${supplier.supplier_id}</td>
          <td>${supplier.company_name}</td>
          <td>${supplier.contact_name || 'N/A'}</td>
          <td>${supplier.email || 'N/A'}</td>
          <td>${supplier.phone || 'N/A'}</td>
          <td>
          <div class="action-buttons">
          <button class="btn btn-warning btn-sm" onclick="editSupplier(${supplier.supplier_id})">Edit</button>
          <button class="btn btn-danger btn-sm" onclick="deleteSupplier(${supplier.supplier_id})">Delete</button>
          <button class="btn btn-primary btn-sm" onclick="viewSupplierDetails(${supplier.supplier_id})">Details</button>
          </div>
          </td>
          `;
        tableBody.appendChild(row);
      });
    } catch (error) {
      console.error('Error loading suppliers:', error);
      showNotification('Failed to load suppliers', 'error');
    }
  }

async function loadSuppliersForDropdown() {
  try {
    const response = await fetch(`${API_BASE_URL}/suppliers`);
    const suppliers = await response.json();

    // For product form
    const productDropdown = document.getElementById('product-supplier');
    productDropdown.innerHTML = '<option value="">Select a supplier</option>';

    // For shipment form
    const shipmentDropdown = document.getElementById('shipment-supplier');
    shipmentDropdown.innerHTML = '<option value="">Select a supplier</option>';

    suppliers.forEach(supplier => {
      const productOption = document.createElement('option');
      productOption.value = supplier.supplier_id;
      productOption.textContent = supplier.company_name;
      productDropdown.appendChild(productOption);

      const shipmentOption = document.createElement('option');
      shipmentOption.value = supplier.supplier_id;
      shipmentOption.textContent = supplier.company_name;
      shipmentDropdown.appendChild(shipmentOption);
    });
  } catch (error) {
    console.error('Error loading suppliers for dropdown:', error);
  }
}

async function createSupplier() {
  const companyName = document.getElementById('supplier-company-name').value;
  const contactName = document.getElementById('supplier-contact-name').value;
  const email = document.getElementById('supplier-email').value;
  const phone = document.getElementById('supplier-phone').value;
  const address = document.getElementById('supplier-address').value;

  if (!companyName) {
    showNotification('Company name is required', 'error');
    return;
  }

  try {
    const response = await fetch(`${API_BASE_URL}/suppliers`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        company_name: companyName,
        contact_name: contactName,
        email,
        phone,
        address
      })
    });

    if (response.ok) {
      showNotification('Supplier created successfully', 'success');
      resetSupplierForm();
      await loadSuppliers();
      await loadSuppliersForDropdown(); // Refresh dropdowns
    } else {
      showNotification('Failed to create supplier', 'error');
    }
  } catch (error) {
    console.error('Error creating supplier:', error);
    showNotification('Failed to create supplier', 'error');
  }
}

async function editSupplier(id) {
  try {
    const response = await fetch(`${API_BASE_URL}/suppliers/${id}`);
    const supplier = await response.json();

    document.getElementById('supplier-company-name').value = supplier.company_name;
    document.getElementById('supplier-contact-name').value = supplier.contact_name || '';
    document.getElementById('supplier-email').value = supplier.email || '';
    document.getElementById('supplier-phone').value = supplier.phone || '';
    document.getElementById('supplier-address').value = supplier.address || '';

    document.getElementById('supplier-create-btn').style.display = 'none';
    document.getElementById('supplier-update-btn').style.display = 'inline-block';
    document.getElementById('supplier-cancel-btn').style.display = 'inline-block';

    currentEditingId = id;
    currentEditingType = 'supplier';
  } catch (error) {
    console.error('Error loading supplier for edit:', error);
    showNotification('Failed to load supplier for editing', 'error');
  }
}

async function updateSupplier() {
  if (!currentEditingId) return;

  const companyName = document.getElementById('supplier-company-name').value;
  const contactName = document.getElementById('supplier-contact-name').value;
  const email = document.getElementById('supplier-email').value;
  const phone = document.getElementById('supplier-phone').value;
  const address = document.getElementById('supplier-address').value;

  if (!companyName) {
    showNotification('Company name is required', 'error');
    return;
  }

  try {
    const response = await fetch(`${API_BASE_URL}/suppliers/${currentEditingId}`, {
      method: 'PUT',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        company_name: companyName,
        contact_name: contactName,
        email,
        phone,
        address
      })
    });

    if (response.ok) {
      showNotification('Supplier updated successfully', 'success');
      resetSupplierForm();
      await loadSuppliers();
      await loadSuppliersForDropdown(); // Refresh dropdowns
    } else {
      showNotification('Failed to update supplier', 'error');
    }
  } catch (error) {
    console.error('Error updating supplier:', error);
    showNotification('Failed to update supplier', 'error');
  }
}

async function deleteSupplier(id) {
  if (!confirm('Are you sure you want to delete this supplier?')) {
    return;
  }

  try {
    const response = await fetch(`${API_BASE_URL}/suppliers/${id}`, {
      method: 'DELETE'
    });

    if (response.ok) {
      showNotification('Supplier deleted successfully', 'success');
      await loadSuppliers();
      await loadSuppliersForDropdown(); // Refresh dropdowns
    } else {
      showNotification('Failed to delete supplier', 'error');
    }
  } catch (error) {
    console.error('Error deleting supplier:', error);
    showNotification('Failed to delete supplier', 'error');
  }
}

async function viewSupplierDetails(id) {
  try {
    const response = await fetch(`${API_BASE_URL}/suppliers/${id}/products`);
    const supplierWithProducts = await response.json();

    // Create a modal or expand row to show details
    alert(`Supplier Details:\nCompany: ${supplierWithProducts.company_name}\nContact: ${supplierWithProducts.contact_name || 'N/A'}\nEmail: ${supplierWithProducts.email || 'N/A'}\nTotal Products: ${supplierWithProducts.products ? supplierWithProducts.products.length : 0}`);
  } catch (error) {
    console.error('Error loading supplier details:', error);
    showNotification('Failed to load supplier details', 'error');
  }
}

function cancelEditSupplier() {
  resetSupplierForm();
}

function resetSupplierForm() {
  document.getElementById('supplier-company-name').value = '';
  document.getElementById('supplier-contact-name').value = '';
  document.getElementById('supplier-email').value = '';
  document.getElementById('supplier-phone').value = '';
  document.getElementById('supplier-address').value = '';

  document.getElementById('supplier-create-btn').style.display = 'inline-block';
  document.getElementById('supplier-update-btn').style.display = 'none';
  document.getElementById('supplier-cancel-btn').style.display = 'none';

  currentEditingId = null;
  currentEditingType = null;
}

// ========== ORDER FUNCTIONS ==========
  async function loadOrders() {
    try {
      showLoading('orders-table-body');
      const response = await fetch(`${API_BASE_URL}/orders`);
      const orders = await response.json();

      const tableBody = document.getElementById('orders-table-body');
      tableBody.innerHTML = '';

      if (orders.length === 0) {
        tableBody.innerHTML = `
          <tr>
          <td colspan="6" class="empty-state">
          <i>📋</i>
          <p>No orders found</p>
          </td>
          </tr>
          `;
        return;
      }

      orders.forEach(order => {
        const row = document.createElement('tr');
        const orderDate = new Date(order.order_date).toLocaleDateString();
        const statusClass = `status-${order.status}`;
        row.innerHTML = `
          <td>${order.order_id}</td>
          <td>${order.customer_name || 'N/A'}</td>
          <td>${orderDate}</td>
          <td><span class="status-badge ${statusClass}">${order.status}</span></td>
          <td>$${parseFloat(order.total_amount).toFixed(2)}</td>
          <td>
          <div class="action-buttons">
          <button class="btn btn-warning btn-sm" onclick="editOrder(${order.order_id})">Edit</button>
          <button class="btn btn-danger btn-sm" onclick="deleteOrder(${order.order_id})">Delete</button>
          <button class="btn btn-primary btn-sm" onclick="viewOrderDetails(${order.order_id})">Details</button>
          </div>
          </td>
          `;
        tableBody.appendChild(row);
      });
    } catch (error) {
      console.error('Error loading orders:', error);
      showNotification('Failed to load orders', 'error');
    }
  }

async function loadProductsForOrderItems() {
  try {
    const response = await fetch(`${API_BASE_URL}/products`);
    const products = await response.json();

    const dropdowns = document.querySelectorAll('.order-item-product');
    dropdowns.forEach(dropdown => {
      dropdown.innerHTML = '<option value="">Select a product</option>';
      products.forEach(product => {
        const option = document.createElement('option');
        option.value = product.product_id;
        option.textContent = `${product.name} - $${parseFloat(product.price).toFixed(2)}`;
        option.setAttribute('data-price', product.price);
        dropdown.appendChild(option);
      });
    });

    // Add event listeners to update price when product is selected
    document.querySelectorAll('.order-item-product').forEach(select => {
      select.addEventListener('change', function() {
        const selectedOption = this.options[this.selectedIndex];
        const price = selectedOption.getAttribute('data-price');
        const quantityInput = this.closest('.item-row').querySelector('.order-item-quantity');
        const priceInput = this.closest('.item-row').querySelector('.order-item-price');

        if (price) {
          priceInput.value = price;
          calculateOrderTotal();
        }
      });
    });

    // Add event listeners to recalculate total when quantity changes
    document.querySelectorAll('.order-item-quantity').forEach(input => {
      input.addEventListener('input', calculateOrderTotal);
    });
  } catch (error) {
    console.error('Error loading products for order items:', error);
  }
}

function addOrderItem() {
  const container = document.getElementById('order-items-container');
  const newItemRow = document.createElement('div');
  newItemRow.className = 'item-row';
  newItemRow.innerHTML = `
    <div class="form-group">
    <label>Product</label>
    <select class="order-item-product">
    <option value="">Select a product</option>
    </select>
    </div>
    <div class="form-group">
    <label>Quantity</label>
    <input type="number" class="order-item-quantity" min="1" value="1">
    </div>
    <div class="form-group">
    <label>Unit Price</label>
    <input type="number" class="order-item-price" step="0.01" min="0" placeholder="0.00">
    </div>
    <div class="item-actions">
    <button type="button" class="btn btn-danger btn-sm" onclick="removeOrderItem(this)">-</button>
    </div>
    `;
  container.appendChild(newItemRow);

  // Load products for the new dropdown
  loadProductsForOrderItems();
}

function removeOrderItem(button) {
  const itemRow = button.closest('.item-row');
  // Don't remove the first item row
  if (document.querySelectorAll('.item-row').length > 1) {
    itemRow.remove();
    calculateOrderTotal();
  }
}

function calculateOrderTotal() {
  let total = 0;
  document.querySelectorAll('.item-row').forEach(row => {
    const quantity = parseFloat(row.querySelector('.order-item-quantity').value) || 0;
    const price = parseFloat(row.querySelector('.order-item-price').value) || 0;
    total += quantity * price;
  });
  document.getElementById('order-total-amount').value = total.toFixed(2);
}

async function createOrder() {
  const customerId = document.getElementById('order-customer').value;
  const status = document.getElementById('order-status').value;
  const totalAmount = document.getElementById('order-total-amount').value;
  const shippingAddress = document.getElementById('order-shipping-address').value;

  if (!customerId || !totalAmount || !shippingAddress) {
    showNotification('Please fill all required fields', 'error');
    return;
  }

  // Collect order items
  const orderItems = [];
  document.querySelectorAll('.item-row').forEach(row => {
    const productId = row.querySelector('.order-item-product').value;
    const quantity = row.querySelector('.order-item-quantity').value;
    const unitPrice = row.querySelector('.order-item-price').value;

    if (productId && quantity && unitPrice) {
      orderItems.push({
        product_id: parseInt(productId),
        quantity: parseInt(quantity),
        unit_price: parseFloat(unitPrice)
      });
    }
  });

  if (orderItems.length === 0) {
    showNotification('Please add at least one order item', 'error');
    return;
  }

  try {
    const response = await fetch(`${API_BASE_URL}/orders`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        customer_id: parseInt(customerId),
        status,
        total_amount: parseFloat(totalAmount),
        shipping_address: shippingAddress,
        items: orderItems
      })
    });

    if (response.ok) {
      showNotification('Order created successfully', 'success');
      resetOrderForm();
      await loadOrders();
    } else {
      showNotification('Failed to create order', 'error');
    }
  } catch (error) {
    console.error('Error creating order:', error);
    showNotification('Failed to create order', 'error');
  }
}

async function editOrder(id) {
  try {
    const response = await fetch(`${API_BASE_URL}/orders/${id}`);
    const order = await response.json();

    document.getElementById('order-customer').value = order.customer_id;
    document.getElementById('order-status').value = order.status;
    document.getElementById('order-total-amount').value = order.total_amount;
    document.getElementById('order-shipping-address').value = order.shipping_address;

    // Clear existing items
    const container = document.getElementById('order-items-container');
    container.innerHTML = '';

    // Add items
    if (order.items && order.items.length > 0) {
      order.items.forEach((item, index) => {
        const newItemRow = document.createElement('div');
        newItemRow.className = 'item-row';
        newItemRow.innerHTML = `
          <div class="form-group">
          <label>Product</label>
          <select class="order-item-product">
          <option value="">Select a product</option>
          </select>
          </div>
          <div class="form-group">
          <label>Quantity</label>
          <input type="number" class="order-item-quantity" min="1" value="${item.quantity}">
          </div>
          <div class="form-group">
          <label>Unit Price</label>
          <input type="number" class="order-item-price" step="0.01" min="0" value="${item.unit_price}">
          </div>
          <div class="item-actions">
          ${index === 0 ? 
              '<button type="button" class="btn btn-success btn-sm" onclick="addOrderItem()">+</button>' : 
              '<button type="button" class="btn btn-danger btn-sm" onclick="removeOrderItem(this)">-</button>'
          }
          </div>
          `;
        container.appendChild(newItemRow);

        // Set the product after dropdown is populated
        setTimeout(() => {
          const select = newItemRow.querySelector('.order-item-product');
          if (select) {
            select.value = item.product_id;
          }
        }, 100);
      });
    } else {
      addOrderItem(); // Add at least one empty item row
    }

    document.getElementById('order-create-btn').style.display = 'none';
    document.getElementById('order-update-btn').style.display = 'inline-block';
    document.getElementById('order-cancel-btn').style.display = 'inline-block';

    currentEditingId = id;
    currentEditingType = 'order';

    // Load products for dropdowns
    await loadProductsForOrderItems();
  } catch (error) {
    console.error('Error loading order for edit:', error);
    showNotification('Failed to load order for editing', 'error');
  }
}

async function updateOrder() {
  if (!currentEditingId) return;

  const customerId = document.getElementById('order-customer').value;
  const status = document.getElementById('order-status').value;
  const totalAmount = document.getElementById('order-total-amount').value;
  const shippingAddress = document.getElementById('order-shipping-address').value;

  if (!customerId || !totalAmount || !shippingAddress) {
    showNotification('Please fill all required fields', 'error');
    return;
  }

  // Collect order items
  const orderItems = [];
  document.querySelectorAll('.item-row').forEach(row => {
    const productId = row.querySelector('.order-item-product').value;
    const quantity = row.querySelector('.order-item-quantity').value;
    const unitPrice = row.querySelector('.order-item-price').value;

    if (productId && quantity && unitPrice) {
      orderItems.push({
        product_id: parseInt(productId),
        quantity: parseInt(quantity),
        unit_price: parseFloat(unitPrice)
      });
    }
  });

  if (orderItems.length === 0) {
    showNotification('Please add at least one order item', 'error');
    return;
  }

  try {
    const response = await fetch(`${API_BASE_URL}/orders/${currentEditingId}`, {
      method: 'PUT',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        customer_id: parseInt(customerId),
        status,
        total_amount: parseFloat(totalAmount),
        shipping_address: shippingAddress,
        items: orderItems
      })
    });

    if (response.ok) {
      showNotification('Order updated successfully', 'success');
      resetOrderForm();
      await loadOrders();
    } else {
      showNotification('Failed to update order', 'error');
    }
  } catch (error) {
    console.error('Error updating order:', error);
    showNotification('Failed to update order', 'error');
  }
}

async function deleteOrder(id) {
  if (!confirm('Are you sure you want to delete this order?')) {
    return;
  }

  try {
    const response = await fetch(`${API_BASE_URL}/orders/${id}`, {
      method: 'DELETE'
    });

    if (response.ok) {
      showNotification('Order deleted successfully', 'success');
      await loadOrders();
    } else {
      showNotification('Failed to delete order', 'error');
    }
  } catch (error) {
    console.error('Error deleting order:', error);
    showNotification('Failed to delete order', 'error');
  }
}

async function viewOrderDetails(id) {
  try {
    const response = await fetch(`${API_BASE_URL}/orders/${id}/details`);
    const orderDetails = await response.json();

    let itemsText = '';
    if (orderDetails.items && orderDetails.items.length > 0) {
      itemsText = orderDetails.items.map(item => 
        `  - ${item.product_name}: ${item.quantity} x $${parseFloat(item.unit_price).toFixed(2)}`
      ).join('\n');
    }

    alert(`Order Details:\nOrder ID: ${orderDetails.order_id}\nCustomer: ${orderDetails.customer_name}\nStatus: ${orderDetails.status}\nTotal: $${parseFloat(orderDetails.total_amount).toFixed(2)}\nShipping: ${orderDetails.shipping_address}\n\nItems:\n${itemsText}`);
  } catch (error) {
    console.error('Error loading order details:', error);
    showNotification('Failed to load order details', 'error');
  }
}

function cancelEditOrder() {
  resetOrderForm();
}

function resetOrderForm() {
  document.getElementById('order-customer').value = '';
  document.getElementById('order-status').value = 'pending';
  document.getElementById('order-total-amount').value = '';
  document.getElementById('order-shipping-address').value = '';

  // Reset items to one empty row
  const container = document.getElementById('order-items-container');
  container.innerHTML = '';
  addOrderItem();

  document.getElementById('order-create-btn').style.display = 'inline-block';
  document.getElementById('order-update-btn').style.display = 'none';
  document.getElementById('order-cancel-btn').style.display = 'none';

  currentEditingId = null;
  currentEditingType = null;
}

// ========== SHIPMENT FUNCTIONS ==========
  async function loadShipments() {
    try {
      showLoading('shipments-table-body');
      const response = await fetch(`${API_BASE_URL}/shipments`);
      const shipments = await response.json();

      const tableBody = document.getElementById('shipments-table-body');
      tableBody.innerHTML = '';

      if (shipments.length === 0) {
        tableBody.innerHTML = `
          <tr>
          <td colspan="7" class="empty-state">
          <i>🚚</i>
          <p>No shipments found</p>
          </td>
          </tr>
          `;
        return;
      }

      shipments.forEach(shipment => {
        const row = document.createElement('tr');
        const shipmentDate = new Date(shipment.shipment_date).toLocaleDateString();
        const expectedDate = new Date(shipment.expected_delivery_date).toLocaleDateString();
        const statusClass = `status-${shipment.status.replace('_', '-')}`;
        row.innerHTML = `
          <td>${shipment.shipment_id}</td>
          <td>${shipment.supplier_name || 'N/A'}</td>
          <td>${shipmentDate}</td>
          <td>${expectedDate}</td>
          <td><span class="status-badge ${statusClass}">${shipment.status}</span></td>
          <td>$${parseFloat(shipment.total_cost).toFixed(2)}</td>
          <td>
          <div class="action-buttons">
          <button class="btn btn-warning btn-sm" onclick="editShipment(${shipment.shipment_id})">Edit</button>
          <button class="btn btn-danger btn-sm" onclick="deleteShipment(${shipment.shipment_id})">Delete</button>
          <button class="btn btn-primary btn-sm" onclick="viewShipmentDetails(${shipment.shipment_id})">Details</button>
          </div>
          </td>
          `;
        tableBody.appendChild(row);
      });
    } catch (error) {
      console.error('Error loading shipments:', error);
      showNotification('Failed to load shipments', 'error');
    }
  }

async function loadProductsForShipmentItems() {
  try {
    const response = await fetch(`${API_BASE_URL}/products`);
    const products = await response.json();

    const dropdowns = document.querySelectorAll('.shipment-item-product');
    dropdowns.forEach(dropdown => {
      dropdown.innerHTML = '<option value="">Select a product</option>';
      products.forEach(product => {
        const option = document.createElement('option');
        option.value = product.product_id;
        option.textContent = `${product.name} - Stock: ${product.stock_quantity}`;
        dropdown.appendChild(option);
      });
    });
  } catch (error) {
    console.error('Error loading products for shipment items:', error);
  }
}

function addShipmentItem() {
  const container = document.getElementById('shipment-items-container');
  const newItemRow = document.createElement('div');
  newItemRow.className = 'item-row';
  newItemRow.innerHTML = `
    <div class="form-group">
    <label>Product</label>
    <select class="shipment-item-product">
    <option value="">Select a product</option>
    </select>
    </div>
    <div class="form-group">
    <label>Quantity</label>
    <input type="number" class="shipment-item-quantity" min="1" value="1">
    </div>
    <div class="form-group">
    <label>Unit Cost</label>
    <input type="number" class="shipment-item-cost" step="0.01" min="0" placeholder="0.00">
    </div>
    <div class="item-actions">
    <button type="button" class="btn btn-danger btn-sm" onclick="removeShipmentItem(this)">-</button>
    </div>
    `;
  container.appendChild(newItemRow);

  // Load products for the new dropdown
  loadProductsForShipmentItems();
}

function removeShipmentItem(button) {
  const itemRow = button.closest('.item-row');
  // Don't remove the first item row
  if (document.querySelectorAll('.item-row').length > 1) {
    itemRow.remove();
    calculateShipmentTotal();
  }
}

function calculateShipmentTotal() {
  let total = 0;
  document.querySelectorAll('.item-row').forEach(row => {
    const quantity = parseFloat(row.querySelector('.shipment-item-quantity').value) || 0;
    const cost = parseFloat(row.querySelector('.shipment-item-cost').value) || 0;
    total += quantity * cost;
  });
  document.getElementById('shipment-total-cost').value = total.toFixed(2);
}

async function createShipment() {
  const supplierId = document.getElementById('shipment-supplier').value;
  const status = document.getElementById('shipment-status').value;
  const shipmentDate = document.getElementById('shipment-date').value;
  const expectedDate = document.getElementById('shipment-expected-date').value;
  const totalCost = document.getElementById('shipment-total-cost').value;

  if (!supplierId || !shipmentDate || !expectedDate || !totalCost) {
    showNotification('Please fill all required fields', 'error');
    return;
  }

  if (new Date(expectedDate) < new Date(shipmentDate)) {
    showNotification('Expected delivery date cannot be before shipment date', 'error');
    return;
  }

  // Collect shipment items
  const shipmentItems = [];
  document.querySelectorAll('.item-row').forEach(row => {
    const productId = row.querySelector('.shipment-item-product').value;
    const quantity = row.querySelector('.shipment-item-quantity').value;
    const unitCost = row.querySelector('.shipment-item-cost').value;

    if (productId && quantity && unitCost) {
      shipmentItems.push({
        product_id: parseInt(productId),
        quantity: parseInt(quantity),
        unit_cost: parseFloat(unitCost)
      });
    }
  });

  if (shipmentItems.length === 0) {
    showNotification('Please add at least one shipment item', 'error');
    return;
  }

  try {
    const response = await fetch(`${API_BASE_URL}/shipments`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        supplier_id: parseInt(supplierId),
        status,
        shipment_date: shipmentDate,
        expected_delivery_date: expectedDate,
        total_cost: parseFloat(totalCost),
        items: shipmentItems
      })
    });

    if (response.ok) {
      showNotification('Shipment created successfully', 'success');
      resetShipmentForm();
      await loadShipments();
    } else {
      showNotification('Failed to create shipment', 'error');
    }
  } catch (error) {
    console.error('Error creating shipment:', error);
    showNotification('Failed to create shipment', 'error');
  }
}

async function editShipment(id) {
  try {
    const response = await fetch(`${API_BASE_URL}/shipments/${id}`);
    const shipment = await response.json();

    document.getElementById('shipment-supplier').value = shipment.supplier_id;
    document.getElementById('shipment-status').value = shipment.status;
    document.getElementById('shipment-date').value = shipment.shipment_date;
    document.getElementById('shipment-expected-date').value = shipment.expected_delivery_date;
    document.getElementById('shipment-total-cost').value = shipment.total_cost;

    // Clear existing items
    const container = document.getElementById('shipment-items-container');
    container.innerHTML = '';

    // Add items
    if (shipment.items && shipment.items.length > 0) {
      shipment.items.forEach((item, index) => {
        const newItemRow = document.createElement('div');
        newItemRow.className = 'item-row';
        newItemRow.innerHTML = `
          <div class="form-group">
          <label>Product</label>
          <select class="shipment-item-product">
          <option value="">Select a product</option>
          </select>
          </div>
          <div class="form-group">
          <label>Quantity</label>
          <input type="number" class="shipment-item-quantity" min="1" value="${item.quantity}">
          </div>
          <div class="form-group">
          <label>Unit Cost</label>
          <input type="number" class="shipment-item-cost" step="0.01" min="0" value="${item.unit_cost}">
          </div>
          <div class="item-actions">
          ${index === 0 ? 
              '<button type="button" class="btn btn-success btn-sm" onclick="addShipmentItem()">+</button>' : 
              '<button type="button" class="btn btn-danger btn-sm" onclick="removeShipmentItem(this)">-</button>'
          }
          </div>
          `;
        container.appendChild(newItemRow);

        // Set the product after dropdown is populated
        setTimeout(() => {
          const select = newItemRow.querySelector('.shipment-item-product');
          if (select) {
            select.value = item.product_id;
          }
        }, 100);
      });
    } else {
      addShipmentItem(); // Add at least one empty item row
    }

    document.getElementById('shipment-create-btn').style.display = 'none';
    document.getElementById('shipment-update-btn').style.display = 'inline-block';
    document.getElementById('shipment-cancel-btn').style.display = 'inline-block';

    currentEditingId = id;
    currentEditingType = 'shipment';

    // Load products for dropdowns
    await loadProductsForShipmentItems();
  } catch (error) {
    console.error('Error loading shipment for edit:', error);
    showNotification('Failed to load shipment for editing', 'error');
  }
}

async function updateShipment() {
  if (!currentEditingId) return;

  const supplierId = document.getElementById('shipment-supplier').value;
  const status = document.getElementById('shipment-status').value;
  const shipmentDate = document.getElementById('shipment-date').value;
  const expectedDate = document.getElementById('shipment-expected-date').value;
  const totalCost = document.getElementById('shipment-total-cost').value;

  if (!supplierId || !shipmentDate || !expectedDate || !totalCost) {
    showNotification('Please fill all required fields', 'error');
    return;
  }

  if (new Date(expectedDate) < new Date(shipmentDate)) {
    showNotification('Expected delivery date cannot be before shipment date', 'error');
    return;
  }

  // Collect shipment items
  const shipmentItems = [];
  document.querySelectorAll('.item-row').forEach(row => {
    const productId = row.querySelector('.shipment-item-product').value;
    const quantity = row.querySelector('.shipment-item-quantity').value;
    const unitCost = row.querySelector('.shipment-item-cost').value;

    if (productId && quantity && unitCost) {
      shipmentItems.push({
        product_id: parseInt(productId),
        quantity: parseInt(quantity),
        unit_cost: parseFloat(unitCost)
      });
    }
  });

  if (shipmentItems.length === 0) {
    showNotification('Please add at least one shipment item', 'error');
    return;
  }

  try {
    const response = await fetch(`${API_BASE_URL}/shipments/${currentEditingId}`, {
      method: 'PUT',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        supplier_id: parseInt(supplierId),
        status,
        shipment_date: shipmentDate,
        expected_delivery_date: expectedDate,
        total_cost: parseFloat(totalCost),
        items: shipmentItems
      })
    });

    if (response.ok) {
      showNotification('Shipment updated successfully', 'success');
      resetShipmentForm();
      await loadShipments();
    } else {
      showNotification('Failed to update shipment', 'error');
    }
  } catch (error) {
    console.error('Error updating shipment:', error);
    showNotification('Failed to update shipment', 'error');
  }
}

async function deleteShipment(id) {
  if (!confirm('Are you sure you want to delete this shipment?')) {
    return;
  }

  try {
    const response = await fetch(`${API_BASE_URL}/shipments/${id}`, {
      method: 'DELETE'
    });

    if (response.ok) {
      showNotification('Shipment deleted successfully', 'success');
      await loadShipments();
    } else {
      showNotification('Failed to delete shipment', 'error');
    }
  } catch (error) {
    console.error('Error deleting shipment:', error);
    showNotification('Failed to delete shipment', 'error');
  }
}

async function viewShipmentDetails(id) {
  try {
    const response = await fetch(`${API_BASE_URL}/shipments/${id}/details`);
    const shipmentDetails = await response.json();

    let itemsText = '';
    if (shipmentDetails.items && shipmentDetails.items.length > 0) {
      itemsText = shipmentDetails.items.map(item => 
        `  - ${item.product_name}: ${item.quantity} x $${parseFloat(item.unit_cost).toFixed(2)}`
      ).join('\n');
    }

    alert(`Shipment Details:\nShipment ID: ${shipmentDetails.shipment_id}\nSupplier: ${shipmentDetails.supplier_name}\nStatus: ${shipmentDetails.status}\nTotal Cost: $${parseFloat(shipmentDetails.total_cost).toFixed(2)}\nShipment Date: ${shipmentDetails.shipment_date}\nExpected Delivery: ${shipmentDetails.expected_delivery_date}\n\nItems:\n${itemsText}`);
  } catch (error) {
    console.error('Error loading shipment details:', error);
    showNotification('Failed to load shipment details', 'error');
  }
}

function cancelEditShipment() {
  resetShipmentForm();
}

function resetShipmentForm() {
  document.getElementById('shipment-supplier').value = '';
  document.getElementById('shipment-status').value = 'in_transit';

  // Set current date and expected date (7 days from now)
  const today = new Date();
  document.getElementById('shipment-date').valueAsDate = today;
  const expectedDate = new Date();
  expectedDate.setDate(today.getDate() + 7);
  document.getElementById('shipment-expected-date').valueAsDate = expectedDate;

  document.getElementById('shipment-total-cost').value = '';

  // Reset items to one empty row
  const container = document.getElementById('shipment-items-container');
  container.innerHTML = '';
  addShipmentItem();

  document.getElementById('shipment-create-btn').style.display = 'inline-block';
  document.getElementById('shipment-update-btn').style.display = 'none';
  document.getElementById('shipment-cancel-btn').style.display = 'none';

  currentEditingId = null;
  currentEditingType = null;
}

// ========== HELPER FUNCTIONS ==========
  function showLoading(tableBodyId) {
    const tableBody = document.getElementById(tableBodyId);
    tableBody.innerHTML = `
      <tr>
      <td colspan="10" style="text-align: center; padding: 2rem;">
      <div class="loading"></div>
      <p>Loading...</p>
      </td>
      </tr>
      `;
  }

// Notification function
function showNotification(message, type) {
  notification.textContent = message;
  notification.className = `notification ${type}`;
  notification.classList.add('show');

  setTimeout(() => {
    notification.classList.remove('show');
  }, 3000);
}

