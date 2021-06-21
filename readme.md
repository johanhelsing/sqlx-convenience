# sqlx-convenience

Convenience stuff to reduce the amount of boiler-plate when doing common stuff with sqlx.

## insert! macro

```rust
insert!("account", id, email, created_at, updated_at)
```

Expands into:

```rust
query!("INSERT INTO account (id, email, created_at, updated_at) VALUES ($1, $2, $3, $4)",
    id, email, created_at, updated_at);
```

Saves you the work of typing the arguments twice and meticulously maintaining
the order and number of `$1` etc.

