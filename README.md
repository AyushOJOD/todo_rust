# 🦀 Rust Todo API with Actix Web

A learning project to build a backend Todo API in Rust using Actix Web, gradually extended to support users and authentication.

## 🚀 Features

- [x] Add, update, delete todos
- [x] In-memory storage using `Mutex<HashMap>`
- [x] RESTful endpoints for todos

## 📂 Project Structure

```bash
src/
├── models/
│   └── todo.rs         # Todo data structures
├── routes/
│   └── todos.rs        # All todo route handlers
├── state.rs            # Shared application state
├── lib.rs              # Route configuration
└── main.rs             # Server setup and entry point
```
