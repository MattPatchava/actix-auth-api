# actix-auth-api

A secure authentication API built with Rust and Actix-Web.

## About

This project is a rewrite of an earlier authentication API I originally built using Node.js and Express.

Now rebuilt in Rust with Actix-Web, this version improves on performance, type safety, and modular structure — part of my transition into systems-level backend development.



## Features

- Register and login endpoints
- Password hashing with Argon2
- JWT access token issuance (HS256)
- PostgreSQL via SQLx
- Modular project structure for maintainability



## Routes

| Method | Path           | Description           |
| ------ | -------------- | --------------------- |
| POST   | /auth/register | Register a new user   |
| POST   | /auth/login    | Login and receive JWT |



## Tech Stack

- [Rust](https://www.rust-lang.org/)
- [Actix-Web](https://actix.rs/)
- [SQLx](https://docs.rs/sqlx)
- [jsonwebtoken](https://docs.rs/jsonwebtoken)
- [argon2](https://docs.rs/argon2)



## Getting Started

```bash
# Run the API locally (ensure a valid .env file exists)
cargo run
```

### Example .env

```
PORT=8080
DATABASE_URL=postgres://user:password@localhost/dbname
JWT_SECRET=your_jwt_secret
```



## TODO

- Add refresh token via httpOnly cookie
- Middleware for JWT validation
- Role-based access control



## Related Projects

- [Express version (Node.js)](https://github.com/MattPatchava/authentication-system/tree/main/backend)