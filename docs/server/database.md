# Database management

As a first step install the `diesel` cli tool using

```sh
cargo install diesel_cli --no-default-features --features sqlite
```

## Modifying the schema

To modify the schema, modify *server/src/schema.rs* with your schema modifications.

Create a migration using

``` sh
diesel migration generate --diff-schema <migration-name>
```

> Generally migrations will be included in the server binary
