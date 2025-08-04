# 🚗 Rust Vehicle API

A high-performance RESTful API for vehicle management built with modern Rust technologies.

## 🛠️ Tech Stack

- **🦀 Rust** - Systems programming language
- **⚡ Axum** - Fast and ergonomic web framework
- **🌊 SeaORM** - Async & dynamic ORM for Rust
- **🐘 PostgreSQL** - Robust relational database
- **🔐 JWT** - JSON Web Tokens for authentication
- **📦 Tokio** - Async runtime for Rust

## 🚀 Features

- ✅ User authentication and authorization
- ✅ JWT-based security
- ✅ RESTful API endpoints
- ✅ Database migrations
- ✅ CORS support
- ✅ Error handling middleware
- ✅ UUID primary keys
- ✅ Async/await throughout

## 🏗️ Project Structure

```
rust-vehicle-api/
├── 📁 src/
│   ├── 🎯 main.rs              # Application entry point
│   ├── 📁 controllers/         # Request handlers
│   ├── 📁 middlewares/         # Auth & other middleware
│   ├── 📁 models/              # Data models
│   ├── 📁 routes/              # Route definitions
│   └── 📁 utils/               # Utilities (JWT, errors)
├── 📁 entity/                  # SeaORM entities
├── 📁 migration/               # Database migrations
├── 📄 Cargo.toml              # Rust dependencies
└── 📄 .env                    # Environment variables
```

## 🔧 Setup & Installation

### Prerequisites

- Rust (latest stable version)
- PostgreSQL
- Sea-ORM CLI

### 1. Clone the repository

```bash
git clone <your-repo-url>
cd rust-vehicle-api
```

### 2. Install dependencies

```bash
cargo build
```

### 3. Install SeaORM CLI

```bash
cargo install sea-orm-cli
```

### 4. Setup database

Create a PostgreSQL database and update your `.env` file:

```env
DATABASE_URL=postgres://username:password@localhost:5432/rust_vehicle_api
JWT_SECRET=your-super-secure-secret-key-here
JWT_EXPIRATION=86400
SERVER_HOST=0.0.0.0
SERVER_PORT=5000
```

### 5. Run migrations

```bash
sea-orm-cli migrate fresh
```

### 6. Start the server

```bash
cargo run
```

The API will be available at `http://localhost:5000`

## 📡 API Endpoints

### Authentication

| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/auth/register` | Register a new user |
| POST | `/auth/login` | Login user |

### Users

| Method | Endpoint | Description | Auth Required |
|--------|----------|-------------|---------------|
| GET | `/users` | Get all users | ✅ |
| GET | `/users/:id` | Get user by ID | ✅ |
| PUT | `/users/:id` | Update user | ✅ |
| DELETE | `/users/:id` | Delete user | ✅ |

### Health Check

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/` | API status |
| GET | `/test` | Test endpoint |

## 🔐 Authentication

This API uses JWT (JSON Web Tokens) for authentication. Include the token in the Authorization header:

```
Authorization: Bearer <your-jwt-token>
```

## 🗃️ Database Schema

### Users Table

| Column | Type | Constraints |
|--------|------|-------------|
| id | UUID | Primary Key, Default: gen_random_uuid() |
| name | VARCHAR | NOT NULL |
| email | VARCHAR | UNIQUE, NOT NULL |
| created_at | TIMESTAMPTZ | NOT NULL, Default: CURRENT_TIMESTAMP |
| updated_at | TIMESTAMPTZ | NOT NULL, Default: CURRENT_TIMESTAMP |

## 🐛 Troubleshooting

### Common Issues

1. **UUID Auto-increment Error**
   ```
   not implemented: Uuid doesn't support auto increment
   ```
   **Solution**: Use `gen_random_uuid()` as default value instead of auto-increment.

2. **Database Authentication Failed**
   ```
   password authentication failed for user "username"
   ```
   **Solution**: Update DATABASE_URL in `.env` with correct credentials.

3. **Migration Issues**
   ```bash
   # Reset migrations
   sea-orm-cli migrate fresh
   
   # Check migration status
   sea-orm-cli migrate status
   ```

## 🔨 Development

### Generate new migration

```bash
sea-orm-cli migrate generate create_vehicles_table
```

### Generate entities from database

```bash
sea-orm-cli generate entity -o entity/src
```

### Run tests

```bash
cargo test
```

### Format code

```bash
cargo fmt
```

### Lint code

```bash
cargo clippy
```

## 📝 Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `DATABASE_URL` | PostgreSQL connection string | Required |
| `JWT_SECRET` | Secret key for JWT tokens | Required |
| `JWT_EXPIRATION` | Token expiration time (seconds) | 86400 |
| `SERVER_HOST` | Server bind address | 0.0.0.0 |
| `SERVER_PORT` | Server port | 5000 |

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with ❤️ using Rust
- Powered by the amazing Rust ecosystem
- SeaORM for making database operations enjoyable
- Axum for the fantastic web framework

---

⭐ **Star this repo if you find it helpful!**
