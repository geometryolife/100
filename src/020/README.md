# Comprehensive case of data manipulation

## Run

```shell
cargo run data/config.toml
```

## Output

```shell
SQLite #Products=4.
SQLite #Sales=5.
PostgreSQL #Products=4.
PostgreSQL #Sales=5.
```

## Question

`if name.local_name == "product-id" =>` 这里为何不是 `if name.local_name == "product_id" =>`？
