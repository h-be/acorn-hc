# hcid

Holochain base32 encoding scheme for keys, agents, identifiers, etc.

```rust
extern crate hcid;

fn main() {
    let enc = hcid::HcidEncoding::with_kind("hcs0").unwrap();
    let key = enc.encode(&[0; 32]).unwrap();
    assert_eq!("HcSciaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", key);
    let buffer = enc.decode(&key).unwrap();
    assert_eq!([0; 32].to_vec(), buffer);
}
```
