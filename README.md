A Rust implementation of [xxHash](http://code.google.com/p/xxhash/).

### Build:

    $ ./build.sh

### Test:

    # schedtool -a 0x1 -R -p 40 -e ./rust-hash --test --bench

    running 25 tests
    test xxhash::c::test_chunks ... ok
    test xxhash::c::test ... ok
    test xxhash::rust::test ... ok
    test xxhash::rust::test_chunks ... ok
    test siphash::chunks_07      ... bench:    393825 ns/iter (+/- 20849) = 166 MB/s
    test siphash::chunks_08      ... bench:    207668 ns/iter (+/- 467) = 315 MB/s
    test siphash::chunks_15      ... bench:    251233 ns/iter (+/- 309) = 260 MB/s
    test siphash::chunks_16      ... bench:    168221 ns/iter (+/- 261) = 389 MB/s
    test siphash::chunks_32      ... bench:    146158 ns/iter (+/- 239) = 448 MB/s
    test siphash::chunks_64      ... bench:    138239 ns/iter (+/- 214) = 474 MB/s
    test siphash::oneshot        ... bench:    828240 ns/iter (+/- 3987) = 79 MB/s
    test xxhash::c::chunks_07    ... bench:    946256 ns/iter (+/- 970) = 276 MB/s
    test xxhash::c::chunks_08    ... bench:    546679 ns/iter (+/- 502) = 479 MB/s
    test xxhash::c::chunks_15    ... bench:    588238 ns/iter (+/- 586) = 445 MB/s
    test xxhash::c::chunks_16    ... bench:    358061 ns/iter (+/- 373) = 731 MB/s
    test xxhash::c::chunks_32    ... bench:    205717 ns/iter (+/- 278) = 1274 MB/s
    test xxhash::c::chunks_64    ... bench:    144501 ns/iter (+/- 158) = 1814 MB/s
    test xxhash::c::oneshot      ... bench:     18825 ns/iter (+/- 18) = 3481 MB/s
    test xxhash::rust::chunks_07 ... bench:    213686 ns/iter (+/- 1169) = 306 MB/s
    test xxhash::rust::chunks_08 ... bench:    135593 ns/iter (+/- 245) = 483 MB/s
    test xxhash::rust::chunks_15 ... bench:    146063 ns/iter (+/- 3296) = 448 MB/s
    test xxhash::rust::chunks_16 ... bench:     63434 ns/iter (+/- 332) = 1033 MB/s
    test xxhash::rust::chunks_32 ... bench:     40166 ns/iter (+/- 173) = 1631 MB/s
    test xxhash::rust::chunks_64 ... bench:     29232 ns/iter (+/- 72) = 2241 MB/s
    test xxhash::rust::oneshot   ... bench:     17006 ns/iter (+/- 14) = 3853 MB/s

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
