# 🚗 Rust Vehicle API

A modern, high-performance RESTful API built with Rust for vehicle management systems. This API provides comprehensive CRUD operations for user management with plans to expand to vehicle-specific functionalities.

## 🚀 Tech Stack

- **🦀 Rust** - Systems programming language focused on safety and performance
- **⚡ Axum** - Ergonomic and modular web framework for Rust
- **🌊 SeaORM** - Modern async ORM for Rust
- **🐘 PostgreSQL** - Advanced open-source relational database
- **🔧 Sea-ORM CLI** - Database migration and entity generation tools

## ✨ Features

- **Fast & Safe**: Built with Rust for maximum performance and memory safety
- **Async/Await**: Fully asynchronous request handling
- **Database Migrations**: Automated database schema management
- **CORS Enabled**: Cross-origin resource sharing support
- **RESTful Design**: Clean and intuitive API endpoints
- **Error Handling**: Comprehensive error responses
- **UUID Support**: Secure unique identifiers for all entities

## 📋 Current Endpoints

### 🏠 General
- `GET /` - Welcome message and API status
- `GET /test` - Health check endpoint

### 👥 User Management
- `GET /users` - Retrieve all users
- `POST /users-create` - Create a new user
- `PATCH /users-update` - Update an existing user
- `DELETE /users-delete/{uuid}` - Delete a user by UUID

## 🛠️ Installation & Setup

### Prerequisites

- **Rust** (latest stable version)
- **PostgreSQL** (version 12 or higher)
- **Git**

### 1. Clone the Repository

```bash
git clone https://github.com/kavinda-100/rust-vehicle-api.git
cd rust-vehicle-api
```

### 2. Database Setup

Create a PostgreSQL database:

```sql
CREATE DATABASE rust_vehicle_api;
```

### 3. Environment Configuration

Create a `.env` file in the project root:

```env
DATABASE_URL=postgresql://username:password@localhost:5432/rust_vehicle_api
```

### 4. Install Dependencies

```bash
cargo build
```

### 5. Run Database Migrations

```bash
# Install SeaORM CLI if not already installed
cargo install sea-orm-cli

# Run migrations
sea-orm-cli migrate up
```

### 6. Start the Server

```bash
cargo run
```

The API will be available at `http://localhost:5000`

## 📖 API Usage Examples

### Create a User

```bash
curl -X POST http://localhost:5000/users-create \
  -H "Content-Type: application/json" \
  -d '{
    "name": "John Doe",
    "email": "john.doe@example.com"
  }'
```

### Get All Users

```bash
curl -X GET http://localhost:5000/users
```

### Update a User

```bash
curl -X PATCH http://localhost:5000/users-update \
  -H "Content-Type: application/json" \
  -d '{
    "id": "uuid-here",
    "name": "John Smith",
    "email": "john.smith@example.com"
  }'
```

### Delete a User

```bash
curl -X DELETE http://localhost:5000/users-delete/{uuid}
```

## 📁 Project Structure

```
rust-vehicle-api/
├── 📁 src/
│   ├── 📄 main.rs              # Application entry point
│   ├── 📁 controllers/         # Request handlers
│   │   └── 📄 user_controller.rs
│   ├── 📁 models/              # Data models
│   │   └── 📄 user_model.rs
│   └── 📁 routes/              # Route definitions
│       └── 📄 user_route.rs
├── 📁 entity/                  # Database entities (SeaORM)
│   ├── 📄 Cargo.toml
│   └── 📁 src/
│       ├── 📄 mod.rs
│       ├── 📄 prelude.rs
│       └── 📄 user.rs
├── 📁 migration/               # Database migrations
│   ├── 📄 Cargo.toml
│   └── 📁 src/
│       ├── 📄 lib.rs
│       ├── 📄 main.rs
│       └── 📄 m20220101_000001_create_table.rs
├── 📄 Cargo.toml               # Project dependencies
├── 📄 .env                     # Environment variables
├── 📄 VehicelAPI.http          # HTTP request examples
└── 📄 README.md                # Project documentation
```

## 🗄️ Database Schema

### Users Table

| Column     | Type      | Description                    |
|------------|-----------|--------------------------------|
| id         | UUID      | Primary key (auto-generated)  |
| name       | VARCHAR   | User's full name               |
| email      | VARCHAR   | User's email (unique)          |
| created_at | TIMESTAMP | Record creation time           |
| updated_at | TIMESTAMP | Last modification time         |

## 🔧 Development

### Running Migrations

```bash
# Create a new migration
sea-orm-cli migrate generate create_vehicle_table

# Apply pending migrations
sea-orm-cli migrate up

# Rollback last migration
sea-orm-cli migrate down

# Reset database (drop all tables and reapply migrations)
sea-orm-cli migrate fresh
```

### Code Generation

Generate entities from database schema:

```bash
sea-orm-cli generate entity -o entity/src
```

### Testing

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture
```

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 👨‍💻 Author

**Your Name**
- GitHub: [@kavinda](https://github.com/kavinda-100)
- Email: your.email@example.com

## 🙏 Acknowledgments

- [Axum](https://github.com/tokio-rs/axum) - Web framework
- [SeaORM](https://github.com/SeaQL/sea-orm) - ORM for Rust
- [Tokio](https://github.com/tokio-rs/tokio) - Async runtime
- [Serde](https://github.com/serde-rs/serde) - Serialization framework

---

⭐ **Star this repository if you find it helpful!**
