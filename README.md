A Rust implementation of [xxHash](http://code.google.com/p/xxhash/).

### Build:

    $ ./build.sh

### Test:

    $ build/rust-hash --test --bench

    running 25 tests
    test xxhash::c::test ... ok
    test xxhash::c::test_chunks ... ok
    test xxhash::rust::test ... ok
    test xxhash::rust::test_chunks ... ok
    test siphash::bench_chunks_15      ... bench:   1040392 ns/iter (+/- 3152) = 251 MB/s
    test siphash::bench_chunks_16      ... bench:    672582 ns/iter (+/- 13339) = 389 MB/s
    test siphash::bench_chunks_32      ... bench:    585660 ns/iter (+/- 1701) = 447 MB/s
    test siphash::bench_chunks_64      ... bench:    553517 ns/iter (+/- 1631) = 473 MB/s
    test siphash::bench_chunks_7       ... bench:   1659018 ns/iter (+/- 38440) = 157 MB/s
    test siphash::bench_chunks_8       ... bench:    822369 ns/iter (+/- 4674) = 318 MB/s
    test siphash::bench_oneshot        ... bench:   3312658 ns/iter (+/- 3850) = 78 MB/s
    test xxhash::c::bench_chunks_15    ... bench:    586070 ns/iter (+/- 545) = 447 MB/s
    test xxhash::c::bench_chunks_16    ... bench:    332730 ns/iter (+/- 2902) = 787 MB/s
    test xxhash::c::bench_chunks_32    ... bench:    214249 ns/iter (+/- 3270) = 1223 MB/s
    test xxhash::c::bench_chunks_64    ... bench:    146364 ns/iter (+/- 2266) = 1790 MB/s
    test xxhash::c::bench_chunks_7     ... bench:    938405 ns/iter (+/- 8357) = 279 MB/s
    test xxhash::c::bench_chunks_8     ... bench:    523084 ns/iter (+/- 2697) = 500 MB/s
    test xxhash::c::bench_oneshot      ... bench:     75159 ns/iter (+/- 86) = 3487 MB/s
    test xxhash::rust::bench_chunks_15 ... bench:    908980 ns/iter (+/- 1462) = 288 MB/s
    test xxhash::rust::bench_chunks_16 ... bench:    265509 ns/iter (+/- 852) = 987 MB/s
    test xxhash::rust::bench_chunks_32 ... bench:    171645 ns/iter (+/- 1802) = 1526 MB/s
    test xxhash::rust::bench_chunks_64 ... bench:    125406 ns/iter (+/- 556) = 2090 MB/s
    test xxhash::rust::bench_chunks_7  ... bench:   1152945 ns/iter (+/- 11663) = 227 MB/s
    test xxhash::rust::bench_chunks_8  ... bench:    815427 ns/iter (+/- 17007) = 321 MB/s
    test xxhash::rust::bench_oneshot   ... bench:     67931 ns/iter (+/- 152) = 3858 MB/s

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
