A Rust implementation of [xxHash](http://code.google.com/p/xxhash/).

### Build:

    $ ./build.sh

### Test

    $ build/rust-hash --test --bench

    running 18 tests
    test xxhash::c::test ... ok
    test xxhash::c::test_chunks ... ok
    test xxhash::rust::test ... ok
    test xxhash::rust::test_chunks ... ok
    test xxhash::c::bench_chunks_15    ... bench:    586097 ns/iter (+/- 11217) = 447 MB/s
    test xxhash::c::bench_chunks_16    ... bench:    357402 ns/iter (+/- 3565) = 733 MB/s
    test xxhash::c::bench_chunks_32    ... bench:    214556 ns/iter (+/- 325) = 1221 MB/s
    test xxhash::c::bench_chunks_64    ... bench:    143919 ns/iter (+/- 284) = 1821 MB/s
    test xxhash::c::bench_chunks_7     ... bench:    933481 ns/iter (+/- 1090) = 280 MB/s
    test xxhash::c::bench_chunks_8     ... bench:    523218 ns/iter (+/- 2679) = 500 MB/s
    test xxhash::c::bench_oneshot      ... bench:     75242 ns/iter (+/- 166) = 3483 MB/s
    test xxhash::rust::bench_chunks_15 ... bench:    825841 ns/iter (+/- 1628) = 317 MB/s
    test xxhash::rust::bench_chunks_16 ... bench:    265967 ns/iter (+/- 677) = 985 MB/s
    test xxhash::rust::bench_chunks_32 ... bench:    173091 ns/iter (+/- 583) = 1514 MB/s
    test xxhash::rust::bench_chunks_64 ... bench:    125132 ns/iter (+/- 612) = 2094 MB/s
    test xxhash::rust::bench_chunks_7  ... bench:   1056544 ns/iter (+/- 5073) = 247 MB/s
    test xxhash::rust::bench_chunks_8  ... bench:    695059 ns/iter (+/- 28216) = 376 MB/s
    test xxhash::rust::bench_oneshot   ... bench:     68212 ns/iter (+/- 5735) = 3843 MB/s

    test result: ok. 4 passed; 0 failed; 0 ignored; 14 measured
