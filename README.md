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
    test siphash::chunks_07      ... bench:    393825 ns/iter (+/- 20849) = 166 MB/s
    test siphash::chunks_08      ... bench:    207668 ns/iter (+/- 467) = 315 MB/s
    test siphash::chunks_15      ... bench:    251233 ns/iter (+/- 309) = 260 MB/s
    test siphash::chunks_16      ... bench:    168221 ns/iter (+/- 261) = 389 MB/s
    test siphash::chunks_32      ... bench:    146158 ns/iter (+/- 239) = 448 MB/s
    test siphash::chunks_64      ... bench:    138239 ns/iter (+/- 214) = 474 MB/s
    test siphash::oneshot        ... bench:    828240 ns/iter (+/- 3987) = 79 MB/s
    test xxhash::gcc::chunks_07  ... bench:    945996 ns/iter (+/- 1221) = 277 MB/s
    test xxhash::gcc::chunks_08  ... bench:    546551 ns/iter (+/- 701) = 479 MB/s
    test xxhash::gcc::chunks_15  ... bench:    588597 ns/iter (+/- 3227) = 445 MB/s
    test xxhash::gcc::chunks_16  ... bench:    332412 ns/iter (+/- 814) = 788 MB/s
    test xxhash::gcc::chunks_32  ... bench:    205771 ns/iter (+/- 435) = 1273 MB/s
    test xxhash::gcc::chunks_64  ... bench:    140300 ns/iter (+/- 308) = 1868 MB/s
    test xxhash::gcc::oneshot    ... bench:     18832 ns/iter (+/- 25) = 3480 MB/s
    test xxhash::clang::chunks_07 ... bench:   1167677 ns/iter (+/- 9781) = 224 MB/s
    test xxhash::clang::chunks_08 ... bench:    845786 ns/iter (+/- 2362) = 309 MB/s
    test xxhash::clang::chunks_15 ... bench:   1067566 ns/iter (+/- 2612) = 245 MB/s
    test xxhash::clang::chunks_16 ... bench:    238343 ns/iter (+/- 2607) = 1099 MB/s
    test xxhash::clang::chunks_32 ... bench:    160018 ns/iter (+/- 935) = 1638 MB/s
    test xxhash::clang::chunks_64 ... bench:    117727 ns/iter (+/- 192) = 2226 MB/s
    test xxhash::clang::oneshot   ... bench:     16829 ns/iter (+/- 33) = 3894 MB/s
    test xxhash::rust::chunks_07  ... bench:    229779 ns/iter (+/- 2607) = 285 MB/s
    test xxhash::rust::chunks_08  ... bench:    136601 ns/iter (+/- 522) = 479 MB/s
    test xxhash::rust::chunks_15  ... bench:    147516 ns/iter (+/- 4419) = 444 MB/s
    test xxhash::rust::chunks_16  ... bench:     63370 ns/iter (+/- 271) = 1034 MB/s
    test xxhash::rust::chunks_32  ... bench:     39933 ns/iter (+/- 245) = 1641 MB/s
    test xxhash::rust::chunks_64  ... bench:     28452 ns/iter (+/- 362) = 2303 MB/s
    test xxhash::rust::oneshot    ... bench:     16996 ns/iter (+/- 42) = 3855 MB/s

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
