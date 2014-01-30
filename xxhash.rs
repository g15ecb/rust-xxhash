use std::unstable::raw::{Repr};
use std::cast;

// no idea how to actually use simd instructions, let's hope the compiler is smarter.
// (lol. 5 minutes later: #11900)
// use std::unstable::simd;
// use std::unstable::simd::{u32x4};

#[inline(always)]
fn rotl32(x: u32, b: u32) -> u32 {
    ((x << b) | (x >> (32 - b)))
}

static PRIME1: u32 = 2654435761;
static PRIME2: u32 = 2246822519;
static PRIME3: u32 = 3266489917;
static PRIME4: u32 = 668265263;
static PRIME5: u32 = 374761393;

#[inline]
pub fn xxh32(input: &[u8], seed: u32) -> u32 {
    unsafe { xxh32_impl(input, seed) }
}

#[cfg(target_endian = "little")]
unsafe fn xxh32_impl(input: &[u8], seed: u32) -> u32 {
    let data: *u8 = input.repr().data;
    let mut p: *u32 = cast::transmute(data);
    let len = input.len();
    let mut h32: u32 = match len >= 16 {
        true => {
            let mut v1 = seed + PRIME1 + PRIME2;
            let mut v2 = seed + PRIME2;
            let mut v3 = seed;
            let mut v4 = seed - PRIME1;

            for _ in range(0, len/16) {
                v1 += *p * PRIME2; v1 = rotl32(v1, 13); v1 *= PRIME1; p = p.offset(1);
                v2 += *p * PRIME2; v2 = rotl32(v2, 13); v2 *= PRIME1; p = p.offset(1);
                v3 += *p * PRIME2; v3 = rotl32(v3, 13); v3 *= PRIME1; p = p.offset(1);
                v4 += *p * PRIME2; v4 = rotl32(v4, 13); v4 *= PRIME1; p = p.offset(1);
            }
            rotl32(v1,1) + rotl32(v2, 7) + rotl32(v3, 12) + rotl32(v4, 18)
        },
        false => seed + PRIME5
    };

    h32 += len as u32;

    let rem = (len & 15) / 4;
    for _ in range (0, rem) {

        h32 += *p * PRIME3;
        h32 = rotl32(h32, 17) * PRIME4;
        p = p.offset(1);

    }

    let rem = len & 3;
    let mut bp: *u8 = cast::transmute(p);
    for _ in range(0, rem) {
        h32 += *bp as u32 * PRIME5;
        h32 = rotl32(h32, 11) * PRIME1;
        bp = bp.offset(1);
    }

    h32 ^= h32 >> 15;
    h32 *= PRIME2;
    h32 ^= h32 >> 13;
    h32 *= PRIME3;
    h32 ^= h32 >> 16;


    h32
}

#[cfg(test)]
mod c {
    use std::libc::{c_int,c_uint,c_void,size_t};
    use std::libc;
    use extra::test::BenchHarness;

    static PRIME : c_uint = 2654435761u32;


    #[link(name="xxhash")]
    extern {
        fn XXH32(input: *c_void, len:c_int, seed: c_uint)-> c_uint;
    }

    #[test] // the xxhash benchmark's sanity test
    fn test() {
        let BUFSIZE: c_int = 101;
        let buf: *mut u8 = unsafe { libc::malloc(BUFSIZE as size_t) as *mut u8 };

        let mut random: u32 = PRIME;

        for i in range(0, BUFSIZE) {
            unsafe {
                let ptr: *mut u8 = buf.offset(i as int);
                *ptr = (random >> 24) as u8;
            }
            random *= random;
        }

        let test = |len: c_int, seed: u32, result: u32| {
            let res = unsafe { XXH32(buf as *c_void, len as c_int, seed as c_uint) } as u32;
            assert_eq!(res, result);
        };

        test(1,                0,      0xB85CBEE5);
        test(1,                PRIME,  0xD5845D64);
        test(14,               0,      0xE5AA0AB4);
        test(14,               PRIME,  0x4481951D);
        test(BUFSIZE,          0,      0x1F1AA412);
        test(BUFSIZE,          PRIME,  0x498EC8E2);

        unsafe { libc::free(buf as *mut c_void); }
    }

    #[bench]
    fn bench(bench: &mut BenchHarness) {
        let BUFSIZE: c_int = 256*1024;
        let buf: *mut c_void = unsafe { libc::malloc(BUFSIZE as size_t) };

        bench.iter(|| unsafe { XXH32(buf as *c_void, BUFSIZE, 0); });

        bench.bytes = BUFSIZE as u64;
        unsafe { libc::free(buf as *mut c_void); }
    }
}

#[cfg(test)]
mod rust {
    use super::{xxh32};
    use extra::test::BenchHarness;

    #[test]
    fn test() {
        use std::vec;
        let BUFSIZE: uint = 101;
        let PRIME: u32 = 2654435761;

        let mut random: u32 = PRIME;
        let mut buf: ~[u8] = vec::with_capacity(BUFSIZE);

        for _ in range(0, BUFSIZE) {
            buf.push((random >> 24) as u8);
            random *= random;
        }

        let test = |size: uint, seed: u32, expected: u32| {
            let result = xxh32(buf.slice(0, size), seed);
            assert_eq!(result, expected);
        };

        test(1,                0,      0xB85CBEE5);
        test(1,                PRIME,  0xD5845D64);
        test(14,               0,      0xE5AA0AB4);
        test(14,               PRIME,  0x4481951D);
        test(BUFSIZE,          0,      0x1F1AA412);
        test(BUFSIZE,          PRIME,  0x498EC8E2);
    }

    #[bench]
    fn bench(bench: &mut BenchHarness) {
        use std::vec;
        use std::libc;
        let BUFSIZE = 256*1024;

        let buf: *mut libc::c_void = unsafe { libc::malloc(BUFSIZE as libc::size_t) };

        let v: ~[u8] = unsafe { vec::raw::from_buf_raw(buf as *u8, BUFSIZE) };
        bench.iter(|| { xxh32(v, 0); } );

        bench.bytes = BUFSIZE as u64;

        unsafe { libc::free(buf as *mut libc::c_void); }
    }
}
