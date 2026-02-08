# caesar
A rough Rust implementation of the Caesar cipher.

This code is probably inefficient and incomplete, but is something I'm coding just for fun so I can learn both mathematical cryptography and some Rust fundamentals.

## Usage
For now, Cargo should be enough:

```bash
cargo run -- <KEY> <MESSAGE>
```

For example:

```bash
cargo run -- 16 Ave Caesar morituri te salutant
```

gives

```plaintext
QLUSQUIQHCEHYJKHYJUIQBKJQDJ
```

## Future improvements
[ ] Error handling
[ ] Key could be negative for quicker decryption
[ ] Unicode support
[ ] Automated unit tests

