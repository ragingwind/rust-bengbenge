# bengbenge

> Simple inifity array for round-robin dns, beng, beng in rustlang. [Origin](https://github.com/ragingwind/node-bengbenge) is from node.

# rust

## Install

## Usese

```rust
use bengbenge::BengBenge;

fn main() {
    let mut bbe = BengBenge::new();

    bbe.append("192.168.0.1".to_string());
    bbe.append("192.168.0.2".to_string());

    println!("{}", bbe.next().unwrap());
    println!("{}", bbe.next().unwrap());
    println!("{}", bbe.next().unwrap());
    println!("{}", bbe.next().unwrap());
}
```

# node

## Install

```sh
npm install bengbenge
```

## Usese

```js
import Bengbenge from 'bengbenge';

const beng = new Bengbenge();

Array.from('12345').forEach(i => beng.append(`www${i}.vercel.com`));

redirect(beng.next());
```

# License

MIT @ Jimmy Moon