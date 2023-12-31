# What should happen

All cases should produce:

```txt
hello world!
```

# What will happen

## Overall:

- [x] ok: `cargo run -p user`
- [x] ok: `cargo run -p user --features wrapper`
- [x] ok: `cargo run -p container`
- [x] ok: `cargo run -p container --features wrapper`

 ## Details
#### ✓ `cargo run -p user`

Will produce:

```txt
```

#### ✓ `cargo run -p user --features wrapper`

Will produce:

```txt
hello world!
```

#### ✓ `cargo run -p container`

Will produce:

```txt
hello world!
```

#### ✓ `cargo run -p container --features wrapper`

Will produce:

```txt
hello world!
hello world!
```

