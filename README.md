`Iterator::max*` returns the last for equal keys,
this crate implements the return the first

```rust
use iter_first_max::IterFirstMaxExt as _;

let arr = [(0, 0), (1, 1), (2, 2), (2, 3), (2, 4)];
let last  = arr.iter().max_by_key(|n| n.0);
let first = arr.iter().first_max_by_key(|n| n.0);

assert_eq!(last,  Some(&(2, 4)));
assert_eq!(first, Some(&(2, 2)));
```
