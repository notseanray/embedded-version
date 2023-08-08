add in main.rs (toplevel)
```
include!(concat!(env!("OUT_DIR"), "/version.rs"));
```

add to build.rs
```
fn main() {
    embedded_version::version().expect("Failed to write version.rs file");
}
```
