A Rust implementation of [xxHash](http://code.google.com/p/xxhash/).

### Build:

    $ ./build.sh

You can choose the C compiler by setting `--cfg gcc` or `--cfg clang` in build.sh.

### Test:

    $ ./rust-hash --test --bench

    running 25 tests
    test xxhash::c::test_chunks ... ok
    test xxhash::c::test ... ok
    test xxhash::rust::test ... ok
    test xxhash::rust::test_chunks ... ok
    <snip benchmarks>
    
    test result: ok. 4 passed; 0 failed; 0 ignored; 21 measured

### Use:

```rust
let mut xxh: XXHState = XXHState::new(seed);
for chunk in v.chunks(64) {
    xxh.update(chunk);
}

let result = xxh.digest();
assert_eq!(result, expected);
```

### Benchmark (MB/s, bigger is better, x86_64):

Test    | SipHash | xxHash gcc | xxHash clang | xxHash Rust |
----|---:|----:|----:|----:|
7-byte chunks  | 166 |  277 |  224 |  **285** |
8-byte chunks  | 315 |  **479** |  309 |  **479** |
15-byte chunks | 260 |  **445** |  245 |  444 |
16-byte chunks | 389 |  788 | **1099** | 1034 |
32-byte chunks | 448 | 1273 | 1638 | **1641** |
64-byte chunks | 474 | 1868 | 2226 | **2303** |
oneshot        |  79 | 3480 | **3894** | 3855 |

SipHash is the Rust implementation in `std::hash`.

All tests are done processing a 64 kB uninitialized array. I've applied all optimizations everywhere that didn't cause the whole benchmark to get optimized out.

These numbers are just some benchmark results, nothing more.

