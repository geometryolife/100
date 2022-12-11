# Simple Blockchain

```rust
pub fn serialize<T: ?Sized>(value: &T) -> Result<Vec<u8>> 
where
    T: Serialize,
```

- `serialize<T: ?Sized>` => `?` 编译时类型的大小不确定 => 传递动态大小类型
- `where` 泛型约束

```rust
pub fn deserialize<'a, T>(bytes: &'a [u8]) -> Result<T> 
where
    T: Deserialize<'a>,
```

- `deserialize<'a, T>` => 具备生命周期的泛型，参数传入的是引用

## Helix

- 编写 `coder.rs` 时，没有分析功能，并提示 => `file not included in module tree`
  - 需要在 `lib.rs` 添加 `pub mod coder`
