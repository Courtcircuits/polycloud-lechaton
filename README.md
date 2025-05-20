# Le Chaton

## Cryptographic benchmarks

0. Install the [hyperfine](https://github.com/sharkdp/hyperfine) CLI.

1. Build both symmetric and asymmetric examples :

```bash
cargo build --bin asymmetric
cargo build --bin symmetric
```

2. Then launch the benchmarks

```bash
hyperfine './target/debug/asymmetric'
hyperfine './target/debug/symmetric'
```
