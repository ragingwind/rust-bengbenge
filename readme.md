# bengbenge

> Simple inifity array for round-robin dns, beng, beng in rustlang. [Origin](https://github.com/ragingwind/node-bengbenge) is from node.

# Usese

```rust
let mut bbe = BengBenge::new();

bbe.append("192.168.0.1".to_string());
bbe.append("192.168.0.2".to_string());
bbe.append("192.168.0.3".to_string());
bbe.append("192.168.0.4".to_string());

let dns = bbe.next();
```

# License

MIT @ Jimmy Moon