# if_debug
A handy rust macro to exclude debugging code from your release builds.

## Example

```rust
fn main() {
  if_debug!(println!("We're debugging!"));
}
```

## Install

Add the following line to your `Cargo.toml` file under `[dependencies]`:

```toml
if_debug = { git = "https://github.com/Humandoodlebug/if_debug" }
```
