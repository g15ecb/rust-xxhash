#[crate_id="rust-hash#0.0"];
#[crate_type="lib"];

#[cfg(test)]
extern mod extra;

pub use xxhash::{XXHState,xxh32};

#[cfg(target_endian = "big")]
#[static_assert]
static little_endian_only_sorry :bool=false;

pub mod xxhash;

#[cfg(test)]
mod siphash {
    use extra::test::BenchHarness;

    #[inline(always)]
    fn bench_base(bench: &mut BenchHarness, f: |&[u8]| ) {
        use std::vec;
        use std::libc;
        let BUFSIZE = 64*1024;

        let buf: *mut libc::c_void = unsafe { libc::malloc(BUFSIZE as libc::size_t) };

        let v: ~[u8] = unsafe { vec::raw::from_buf_raw(buf as *u8, BUFSIZE) };
        bench.iter( || f(v) );
        bench.bytes = BUFSIZE as u64;
        unsafe { libc::free(buf as *mut libc::c_void); }

    }

    #[inline(always)]
    fn bench_chunks(bench: &mut BenchHarness, chunk_size: uint) {
        use std::hash;
        use std::hash::{Streaming};
        bench_base( bench, |v| {
            let mut sip: hash::State = hash::default_state();
            for chunk in v.chunks(chunk_size) {
                sip.write(chunk);
            }
            sip.result_u64();
        });
    }

    #[bench]
    fn oneshot(bench: &mut BenchHarness) {
        use std::hash::{Hash};
        bench_base( bench, |v| {
            v.hash();
        });
    }

    #[bench]
    fn chunks_07(bench: &mut BenchHarness) {
        bench_chunks(bench, 7);
    }

    #[bench]
    fn chunks_08(bench: &mut BenchHarness) {
        bench_chunks(bench, 8);
    }

    #[bench]
    fn chunks_15(bench: &mut BenchHarness) {
        bench_chunks(bench, 15);
    }

    #[bench]
    fn chunks_16(bench: &mut BenchHarness) {
        bench_chunks(bench, 16);
    }

    #[bench]
    fn chunks_32(bench: &mut BenchHarness) {
        bench_chunks(bench, 32);
    }

    #[bench]
    fn chunks_64(bench: &mut BenchHarness) {
        bench_chunks(bench, 64);
    }
}
