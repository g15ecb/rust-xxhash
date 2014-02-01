A Rust implementation of [xxHash](http://code.google.com/p/xxhash/).

### Build:

    $ ./build.sh

### Test

    $ build/rust-hash --test --bench

    running 16 tests
    test xxhash::c::test ... ok
    test xxhash::c::test_chunks ... ok
    test xxhash::rust::test ... ok
    test xxhash::rust::test_chunks ... ok
    test xxhash::c::bench              ... bench:     75396 ns/iter (+/- 90) = 3476 MB/s
    test xxhash::c::bench_chunks_15    ... bench:    586012 ns/iter (+/- 992) = 447 MB/s
    test xxhash::c::bench_chunks_32    ... bench:    205290 ns/iter (+/- 315) = 1276 MB/s
    test xxhash::c::bench_chunks_64    ... bench:    140326 ns/iter (+/- 213) = 1868 MB/s
    test xxhash::c::bench_chunks_7     ... bench:    933486 ns/iter (+/- 699) = 280 MB/s
    test xxhash::c::bench_chunks_8     ... bench:    521924 ns/iter (+/- 677) = 502 MB/s
    test xxhash::rust::bench           ... bench:     67928 ns/iter (+/- 250) = 3859 MB/s
    test xxhash::rust::bench_chunks_15 ... bench:    825434 ns/iter (+/- 2251) = 317 MB/s
    test xxhash::rust::bench_chunks_32 ... bench:    172984 ns/iter (+/- 366) = 1515 MB/s
    test xxhash::rust::bench_chunks_64 ... bench:    125787 ns/iter (+/- 1132) = 2083 MB/s
    test xxhash::rust::bench_chunks_7  ... bench:   1059282 ns/iter (+/- 11616) = 247 MB/s
    test xxhash::rust::bench_chunks_8  ... bench:    694461 ns/iter (+/- 2409) = 377 MB/s

    test result: ok. 4 passed; 0 failed; 0 ignored; 12 measured
