A Rust implementation of [xxHash](http://code.google.com/p/xxhash/).

### Build:

    $ ./build.sh

### Test

    $ build/rust-hash --test --bench

    running 4 tests
    test xxhash::c::test ... ok
    test xxhash::rust::test ... ok
    test xxhash::c::bench    ... bench:     75297 ns/iter (+/- 117) = 3481 MB/s
    test xxhash::rust::bench ... bench:     68074 ns/iter (+/- 500) = 3850 MB/s

    test result: ok. 2 passed; 0 failed; 0 ignored; 2 measured
