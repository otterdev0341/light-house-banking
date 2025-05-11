# 📘 Use Case Functions with DTO Structures

---

## 👜 Asset Management

| Function | Description |
|---------|-------------|
| `createAsset(CreateAssetDto)` | Create a new asset using: <br>**CreateAssetDto** `{ name: str, asset_type_id: int }` |
| `getAllAssets()` | Retrieve all assets. Returns: List of **ResAssetDto** |
| `updateAsset(id, UpdateAssetDto)` | Update asset by ID with: <br>**UpdateAssetDto** `{ name?: str, asset_type_id?: int }` |
| `deleteAsset(id)` | Delete asset if no linked transactions. |

---

## 🧾 Expense Management

| Function | Description |
|---------|-------------|
| `createExpense(CreateExpenseDto)` | Create a new expense item using: <br>**CreateExpenseDto** `{ description: str, expense_type_id: int }` |
| `getAllExpenses()` | Get all defined expense items. Returns: List of **ResExpenseDto** |
| `updateExpense(id, UpdateExpenseDto)` | Update an expense with: <br>**UpdateExpenseDto** `{ description?: str, expense_type_id?: int }` |
| `deleteExpense(id)` | Delete expense if safe to do so. |

---

## 💰 Transaction Management

| Function | Description |
|---------|-------------|
| `recordIncome(CreateTransactionDto)` | Record income using:<br>**CreateTransactionDto** `{ transaction_type: 'Income', amount: Decimal, asset_id: int, contact_id?: int, note?: str }` |
| `recordPayment(CreateTransactionDto)` | Record payment using:<br>**CreateTransactionDto** `{ transaction_type: 'Payment', amount: Decimal, asset_id: int, expense_id: int, contact_id?: int, note?: str }` |
| `getTransactions()` | Get all transactions (income + payments). Returns: List of **ResTransactionDto** |
| `getIncomeTransactions()` | Get only income transactions. |
| `getPaymentTransactions()` | Get only payment transactions. |
| `getTransactionsByMonth(month: str)` | Get all transactions in a given month (format: `'YYYY-MM'`). |

---

## 🔁 Fund Transfer

| Function | Description |
|---------|-------------|
| `transferFund(TransferFundDto)` | Move funds between assets using:<br>**TransferFundDto** `{ source_asset_id: int, destination_asset_id: int, amount: Decimal, note?: str }` |

---

## 👥 Contact Management

| Function | Description |
|---------|-------------|
| `createContact(CreateContactDto)` | Add a contact using:<br>**CreateContactDto** `{ name: str, business_name: str, phone: str, description?: str, contact_type_id: int }` |
| `getAllContacts()` | Get all contacts. Returns: List of **ResContactDto** |
| `updateContact(id, UpdateContactDto)` | Update contact using:<br>**UpdateContactDto** `{ name?, business_name?, phone?, description?, contact_type_id? }` |
| `deleteContact(id)` | Delete or deactivate a contact. |

---

## 🧾 Customer Type Management

| Function | Description |
|---------|-------------|
| `getCustomerTypes()` | Get contact types (e.g., Customer, Vendor). Returns: List of **ResContactTypeDto** |
| `createCustomer(CreateContactDto)` | Shortcut to create contact with customer type. |
| `createVendor(CreateContactDto)` | Shortcut to create contact with vendor type. |

---

## 📊 Summary & Reports

| Function | Description |
|---------|-------------|
| `getMonthlySummary(month: str)` | Returns monthly summary grouped by asset type and expense type.<br>Input: `month = 'YYYY-MM'` |

---

## 📦 DTO Summary

<details>
<summary>🔍 Expand to view DTO details</summary>

### 🔹 Asset DTOs
- **CreateAssetDto**: `{ name: str, asset_type_id: int }`
- **UpdateAssetDto**: `{ name?: str, asset_type_id?: int }`
- **ResAssetDto**: `{ id, name, asset_type_id, created_at?, updated_at? }`

### 🔹 Expense DTOs
- **CreateExpenseDto**: `{ description: str, expense_type_id: int }`
- **UpdateExpenseDto**: `{ description?: str, expense_type_id?: int }`
- **ResExpenseDto**: `{ id, description, expense_type_id, created_at?, updated_at? }`

### 🔹 Contact DTOs
- **CreateContactDto**: `{ name, business_name, phone, description?, contact_type_id }`
- **UpdateContactDto**: `{ name?, business_name?, phone?, description?, contact_type_id? }`
- **ResContactDto**: `{ id, name, business_name, phone, description?, contact_type_id, created_at?, updated_at? }`

### 🔹 Transaction DTOs
- **CreateTransactionDto**: `{ transaction_type: 'Income' | 'Payment', amount, asset_id, expense_id?, contact_id?, note? }`
- **UpdateTransactionDto**: Same fields, all optional.
- **ResTransactionDto**: `{ id, transaction_type, amount, asset_id, expense_id?, contact_id?, note?, created_at?, updated_at? }`

### 🔹 Fund Transfer DTO
- **TransferFundDto**: `{ source_asset_id, destination_asset_id, amount, note? }`

### 🔹 Contact Type DTOs
- **ResContactTypeDto**: `{ id, name, created_at?, updated_at? }`

</details>
