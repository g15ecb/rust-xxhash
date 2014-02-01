A Rust implementation of [xxHash](http://code.google.com/p/xxhash/).

### Build:

    $ ./build.sh

### Test

    $ build/rust-hash --test --bench

    running 8 tests
    test xxhash::c::test ... ok
    test xxhash::c::test_chunks ... ok
    test xxhash::rust::test ... ok
    test xxhash::rust::test_chunks ... ok
    test xxhash::c::bench           ... bench:     75283 ns/iter (+/- 82) = 3482 MB/s
    test xxhash::c::bench_chunks    ... bench:    577563 ns/iter (+/- 757) = 453 MB/s
    test xxhash::rust::bench        ... bench:     67884 ns/iter (+/- 95) = 3861 MB/s
    test xxhash::rust::bench_chunks ... bench:    811369 ns/iter (+/- 2497) = 322 MB/s

    test result: ok. 4 passed; 0 failed; 0 ignored; 4 measured
