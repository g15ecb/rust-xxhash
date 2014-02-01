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
    test siphash::bench_chunks_15      ... bench:    256395 ns/iter (+/- 5826) = 255 MB/s
    test siphash::bench_chunks_16      ... bench:    166980 ns/iter (+/- 3279) = 392 MB/s
    test siphash::bench_chunks_32      ... bench:    146360 ns/iter (+/- 694) = 447 MB/s
    test siphash::bench_chunks_64      ... bench:    138619 ns/iter (+/- 263) = 472 MB/s
    test siphash::bench_chunks_7       ... bench:    398572 ns/iter (+/- 31296) = 164 MB/s
    test siphash::bench_chunks_8       ... bench:    206662 ns/iter (+/- 1137) = 317 MB/s
    test siphash::bench_oneshot        ... bench:    827907 ns/iter (+/- 1658) = 79 MB/s
    test xxhash::c::bench_chunks_15    ... bench:    588104 ns/iter (+/- 514) = 445 MB/s
    test xxhash::c::bench_chunks_16    ... bench:    333169 ns/iter (+/- 535) = 786 MB/s
    test xxhash::c::bench_chunks_32    ... bench:    205790 ns/iter (+/- 241) = 1273 MB/s
    test xxhash::c::bench_chunks_64    ... bench:    140199 ns/iter (+/- 182) = 1869 MB/s
    test xxhash::c::bench_chunks_7     ... bench:    946005 ns/iter (+/- 881) = 277 MB/s
    test xxhash::c::bench_chunks_8     ... bench:    546597 ns/iter (+/- 698) = 479 MB/s
    test xxhash::c::bench_oneshot      ... bench:     18802 ns/iter (+/- 24) = 3485 MB/s
    test xxhash::rust::bench_chunks_15 ... bench:    145384 ns/iter (+/- 2055) = 450 MB/s
    test xxhash::rust::bench_chunks_16 ... bench:     62816 ns/iter (+/- 248) = 1043 MB/s
    test xxhash::rust::bench_chunks_32 ... bench:     39764 ns/iter (+/- 242) = 1648 MB/s
    test xxhash::rust::bench_chunks_64 ... bench:     28215 ns/iter (+/- 809) = 2322 MB/s
    test xxhash::rust::bench_chunks_7  ... bench:    216114 ns/iter (+/- 247) = 303 MB/s
    test xxhash::rust::bench_chunks_8  ... bench:    136653 ns/iter (+/- 322) = 479 MB/s
    test xxhash::rust::bench_oneshot   ... bench:     17013 ns/iter (+/- 74) = 3852 MB/s

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
