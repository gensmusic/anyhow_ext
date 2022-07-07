# anyhow_ext

An extension of [anyhow](https://github.com/dtolnay/anyhow). A drop-in replacement of anyhow.

# Features

- Attach file location info in error message

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

# usage

Work with anyhow!

```toml
[dependencies]
anyhow = "1"
anyhow_ext = "0.1"
```
Then use `anyhow_ext::Context` instead of `anyhow::Context`.


```rust
use anyhow::{Result,bail};
use anyhow_ext::Context;

fn foo() -> Result<()> {
    bail("an anyhow err").context("wrap with file info")?;
    Ok(())
}
```

Since `anyhow_ext` re-exports all the thing in `anyhow` except for `Context`, you can use anyhow_ext as a drop-in replacement.

```toml
[dependencies]
anyhow_ext = "0.1"
```

```rust
use anyhow_ext::{Result,bail Context};

fn foo() -> Result<()> {
    bail("an anyhow err").context("wrap with file info")?;
    Ok(())
}
```