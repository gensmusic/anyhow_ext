# anyhow_ext

An extension of [anyhow](https://github.com/dtolnay/anyhow). A drop-in replacement of anyhow.

# Features

- Attach file location info in error message
- Use `dot` to make a small backtrace.

# usage

## Work with anyhow!

```toml
[dependencies]
anyhow = "1"
anyhow_ext = "0.2"
```
Then use `anyhow_ext::Context` instead of `anyhow::Context`.

```rust
use anyhow::{anyhow, Result};
use anyhow_ext::Context;

fn foo() -> Result<()> {
    Err(anyhow!("an anyhow err")).context("wrap with file info")?;
    Ok(())
}
```

## drop-in replacement of anyhow

Since `anyhow_ext` re-exports all the thing in `anyhow` except for `Context`, you can use anyhow_ext as a drop-in replacement.

`Cargo.toml`
```toml
[dependencies]
anyhow_ext = "0.2"
```
Then
```rust
use anyhow_ext::{Result,anyhow,Context};

fn foo() -> Result<()> {
    Err(anyhow!("an anyhow err")).context("wrap with file info")?;
    Ok(())
}
```

## How localtion info is displayed

`println!("{}", err)` looks like

```plain
foo err at `src/bin/anyhow_ext.rs@6:11`
```

and `println!("{:?}", err)` looks like

```plain
  foo err at `src/bin/anyhow_ext.rs@6:11`

  Caused by:
  0: read_a_file err at `src/bin/anyhow_ext.rs@10:19`
  1: an io err at `src/bin/anyhow_ext.rs@15:55`
  2: No such file or directory (os error 2)
```

## `dot` to make a small backtrace

```rust
use anyhow_ext::{Context, Result};

fn foo() -> Result<()> {
    std::fs::read_to_string("file_not_exists.txt").dot()?;
    Ok(())
}

fn bar() -> Result<()> {
    foo().dot()?;
    Ok(())
}

fn main() {
    if let Err(err) = try_main().dot() {
        println!("{:?}", err);
    }
}

fn try_main() -> Result<()> {
    bar().dot()?;
    Ok(())
}
```
This will make a small backtrace.

```text
at `examples/dot.rs@14:34`

Caused by:
    0: at `examples/dot.rs@20:11`
    1: at `examples/dot.rs@9:11`
    2: at `examples/dot.rs@4:52`
    3: No such file or directory (os error 2)
```