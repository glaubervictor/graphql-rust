# GraphQL - Rust

For studies and improvements in the Rust language.

# Tech

- **Rust**: For building the app;
- **Async-Graphql**: For GraphQL;
- **Sea-ORM**: ORM database with integration Postgres;
- **Axum**: Web Framework.

# Migrating the Project

- Use `sea-orm-cli migrate up` after install cli.
- Use `sea-orm-cli generate entity --compact-format --tables person --output-dir ./trash` for create entity (ex. Person)
  after new migration created and updated in database.

### Ps

I am using authorization and authentication using JWT and based on user roles.

### Screen

<img width="1629" alt="image" src="https://github.com/user-attachments/assets/f28e52fe-3431-42bf-bde0-a446c30960a2">
