use std::raw::{Repr};
use std::cast;
use std::intrinsics;
use std::io::{IoResult, Writer};
use std::default::{Default};
use std::hash::{Hash, Hasher};

fn rotl32(x: u32, b: u32) -> u32 { #[inline(always)];
    ((x << b) | (x >> (32 - b)))
}

static PRIME1: u32 = 2654435761;
static PRIME2: u32 = 2246822519;
static PRIME3: u32 = 3266489917;
static PRIME4: u32 = 668265263;
static PRIME5: u32 = 374761393;

pub fn xxh32(input: &[u8], seed: u32) -> u32 { #[inline];
    unsafe { xxh32_impl(input, seed) }
}

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

pub struct XXState {
    // field names match the C implementation
    priv memory: [u8, ..16],
    priv total_len: u64,
    priv v1: u32,
    priv v2: u32,
    priv v3: u32,
    priv v4: u32,
    priv memsize: int,
    priv seed: u32,
}

impl XXState {
    pub fn new(seed: u32) -> XXState { #[inline];
        // no need to write it twice
        let mut xxh: XXState = unsafe { intrinsics::uninit() };
        xxh.reset(seed);
        xxh
    }

    pub fn reset(&mut self, seed: u32) { #[inline];
        self.seed = seed;
        self.v1 = seed + PRIME1 + PRIME2;
        self.v2 = seed + PRIME2;
        self.v3 = seed;
        self.v4 = seed - PRIME1;
        self.total_len = 0;
        self.memsize = 0;
    }

    pub fn update(&mut self, input: &[u8]) { #[inline];
        unsafe { self.update_impl(input) }
    }

    unsafe fn update_impl(&mut self, input: &[u8]) { #[inline];
        let mut len: int = input.len() as int;
        let mut data: *u8 = input.repr().data;

        self.total_len += len as u64;

        if self.memsize + (len as int) < 16 {
            // not enough data for one 16-byte chunk, so just fill the buffer and return.
            let dst: *mut u8 = (&self.memory as *mut u8).offset(self.memsize);
            intrinsics::copy_nonoverlapping_memory(dst, data, len as uint);
            self.memsize += len as int;
            return;
        }

        if self.memsize != 0 {
            // some data left from previous update
            // fill the buffer and eat it
            // XXH_memcpy(state->memory + state->memsize, input, 16-state->memsize);
            let dst: *mut u8 = (&self.memory as *mut u8).offset(self.memsize);
            let bump: int = 16 - self.memsize as int;
            intrinsics::copy_nonoverlapping_memory(dst, data, bump as uint);

            let mut p: *u32 = cast::transmute(&self.memory);

            let mut v1: u32 = self.v1;
            let mut v2: u32 = self.v2;
            let mut v3: u32 = self.v3;
            let mut v4: u32 = self.v4;

            v1 += (*p) * PRIME2; v1 = rotl32(v1, 13); v1 *= PRIME1; p = p.offset(1);
            v2 += (*p) * PRIME2; v2 = rotl32(v2, 13); v2 *= PRIME1; p = p.offset(1);
            v3 += (*p) * PRIME2; v3 = rotl32(v3, 13); v3 *= PRIME1; p = p.offset(1);
            v4 += (*p) * PRIME2; v4 = rotl32(v4, 13); v4 *= PRIME1;

            self.v1 = v1;
            self.v2 = v2;
            self.v3 = v3;
            self.v4 = v4;

            data = data.offset(bump as int);
            len -= bump;
            self.memsize = 0;

        }

        let mut p: *u32 = cast::transmute(data);
        let chunks = len >> 4;
        let rem: int = len & 15;

        let mut v1: u32 = self.v1;
        let mut v2: u32 = self.v2;
        let mut v3: u32 = self.v3;
        let mut v4: u32 = self.v4;

        for _ in range(0, chunks) {
            v1 += (*p) * PRIME2; v1 = rotl32(v1, 13); v1 *= PRIME1; p = p.offset(1);
            v2 += (*p) * PRIME2; v2 = rotl32(v2, 13); v2 *= PRIME1; p = p.offset(1);
            v3 += (*p) * PRIME2; v3 = rotl32(v3, 13); v3 *= PRIME1; p = p.offset(1);
            v4 += (*p) * PRIME2; v4 = rotl32(v4, 13); v4 *= PRIME1; p = p.offset(1);
        }

        self.v1 = v1;
        self.v2 = v2;
        self.v3 = v3;
        self.v4 = v4;

        if rem > 0 {
            let dst: *mut u8 = &self.memory as *mut u8;
            intrinsics::copy_nonoverlapping_memory(dst, data, rem as uint);
            self.memsize = rem as int;
        }
    }

    /// Can be called on intermediate states
    pub fn digest(&self) -> u32 { #[inline];
        unsafe { self.digest_impl() }
    }

    unsafe fn digest_impl(&self) -> u32 { #[inline];
        let mut p: *u32 = cast::transmute(&self.memory);

        let mut h32: u32 = match self.total_len >= 16 {
            true => rotl32(self.v1, 1) + rotl32(self.v2, 7) + rotl32(self.v3, 12) + rotl32(self.v4, 18),
            false => self.seed + PRIME5
        };

        h32 += self.total_len as u32;

        let rem = (self.total_len & 15) / 4;
        for _ in range (0, rem) {

            h32 += *p * PRIME3;
            h32 = rotl32(h32, 17) * PRIME4;
            p = p.offset(1);

        }

        let rem = self.total_len & 3;
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

}

impl Writer for XXState {
    fn write(&mut self, input: &[u8]) -> IoResult<()> { #[inline];
        unsafe { self.update_impl(input); }
        Ok(())
    }
}

impl Default for XXState {
    fn default() -> XXState { #[inline];
        XXState::new(0)
    }
}

impl Clone for XXState {
    fn clone(&self) -> XXState { #[inline];
        *self
    }
}

pub struct XXHasher {
    priv seed: u32
}

impl XXHasher {
    pub fn new() -> XXHasher { #[inline];
        XXHasher::new_with_seed(0)
    }

    pub fn new_with_seed(seed: u32) -> XXHasher { #[inline];
        XXHasher { seed: seed }
    }
}

impl Hasher<XXState> for XXHasher {
    fn hash<T: Hash<XXState>>(&self, value: &T) -> u64 {
        let mut state = XXState::new(self.seed);
        value.hash(&mut state);
        state.digest() as u64
    }
}

pub fn hash<T: Hash<XXState>>(value: &T) -> u32 {
    let mut state = XXState::new(0);
    value.hash(&mut state);
    state.digest()
}

pub fn hash_with_seed<T: Hash<XXState>>(seed: u32, value: &T) -> u32 {
    let mut state = XXState::new(seed);
    value.hash(&mut state);
    state.digest()
}


#[cfg(test)]
mod c {
    use std::libc::{c_int,c_uint,c_void,size_t};
    use std::libc;
    use test::test::BenchHarness;
    use std::num::{div_rem};

    static PRIME : c_uint = 2654435761u32;

    #[repr(C)]
    enum XXH_Endianess { XXH_BigEndian=0, XXH_LittleEndian=1 }

    #[cfg(clang)]
    #[link(name="xxhash-clang")]
    extern {
        fn XXH32(input: *c_void, len:c_int, seed: u32)-> c_uint;
        fn XXH32_init(seed: u32) -> *mut c_void;
        fn XXH32_update(state: *mut c_void, input: *c_void, len: c_int, endian: XXH_Endianess) -> bool;
        fn XXH32_digest(state: *mut c_void) -> u32;
    }

    #[cfg(gcc)]
    #[link(name="xxhash-gcc")]
    extern {
        fn XXH32(input: *c_void, len:c_int, seed: u32)-> c_uint;
        fn XXH32_init(seed: u32) -> *mut c_void;
        fn XXH32_update(state: *mut c_void, input: *c_void, len: c_int, endian: XXH_Endianess) -> bool;
        fn XXH32_digest(state: *mut c_void) -> u32;
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

        let test = |len: c_int, seed: u32, expected: u32| {
            let result = unsafe { XXH32(buf as *c_void, len as c_int, seed as c_uint) } as u32;
            assert_eq!(expected, result);
        };

        test(1,                0,      0xB85CBEE5);
        test(1,                PRIME,  0xD5845D64);
        test(14,               0,      0xE5AA0AB4);
        test(14,               PRIME,  0x4481951D);
        test(BUFSIZE,          0,      0x1F1AA412);
        test(BUFSIZE,          PRIME,  0x498EC8E2);

        unsafe { libc::free(buf as *mut c_void); }
    }

    #[test]
    fn test_chunks() {
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

        let test = |size: c_int, seed: u32, expected: u32| {
            unsafe {
                let state: *mut c_void = XXH32_init(seed);
                let (chunks, rem) = div_rem(size, 15);
                for chunk in range(0, chunks) {
                    XXH32_update(state, buf.offset(chunk as int *15) as *c_void, 15 as c_int, XXH_LittleEndian);
                }
                XXH32_update(state, buf.offset(chunks as int * 15) as *c_void, rem, XXH_LittleEndian);
                let result = XXH32_digest(state);

                assert_eq!(result, expected);
            }
        };

        test(1,                0,      0xB85CBEE5);
        test(1,                PRIME,  0xD5845D64);
        test(14,               0,      0xE5AA0AB4);
        test(14,               PRIME,  0x4481951D);
        test(BUFSIZE,          0,      0x1F1AA412);
        test(BUFSIZE,          PRIME,  0x498EC8E2);
    }

    #[bench]
    fn oneshot(bench: &mut BenchHarness) {
        let BUFSIZE: c_int = 64*1024;
        let buf: *mut c_void = unsafe { libc::malloc(BUFSIZE as size_t) };

        bench.iter(|| unsafe { XXH32(buf as *c_void, BUFSIZE, 0); });

        bench.bytes = BUFSIZE as u64;
        unsafe { libc::free(buf as *mut c_void); }
    }

    #[inline(always)]
    fn bench_chunks_base(bench: &mut BenchHarness, chunk_size: i32) {
        let BUFSIZE: c_int = 64*1024;
        let buf: *mut c_void = unsafe { libc::malloc(BUFSIZE as size_t) };

        bench.iter(|| unsafe {
            let state: *mut c_void = XXH32_init(0);
            let (chunks, rem) = div_rem(BUFSIZE, chunk_size);
            for chunk in range(0, chunks) {
                XXH32_update(state, buf.offset(chunk as int * chunk_size as int) as *c_void, chunk_size as c_int, XXH_LittleEndian);
            }
            XXH32_update(state, buf.offset(chunks as int * chunk_size as int) as *c_void, rem, XXH_LittleEndian);
            XXH32_digest(state);
        });

        bench.bytes = BUFSIZE as u64;
        unsafe { libc::free(buf as *mut c_void); }
    }

    #[bench]
    fn chunks_01(bench: &mut BenchHarness) {
        bench_chunks_base(bench, 1);
    }

    #[bench]
    fn chunks_02(bench: &mut BenchHarness) {
        bench_chunks_base(bench, 2);
    }

    #[bench]
    fn chunks_04(bench: &mut BenchHarness) {
        bench_chunks_base(bench, 4);
    }

    #[bench]
    fn chunks_07(bench: &mut BenchHarness) {
        bench_chunks_base(bench, 7);
    }
    #[bench]
    fn chunks_08(bench: &mut BenchHarness) {
        bench_chunks_base(bench, 8);
    }

    #[bench]
    fn chunks_15(bench: &mut BenchHarness) {
        bench_chunks_base(bench, 15);
    }
    #[bench]
    fn chunks_16(bench: &mut BenchHarness) {
        bench_chunks_base(bench, 16);
    }
    #[bench]
    fn chunks_32(bench: &mut BenchHarness) {
        bench_chunks_base(bench, 32);
    }
    #[bench]
    fn chunks_64(bench: &mut BenchHarness) {
        bench_chunks_base(bench, 64);
    }

}

#[cfg(test)]
mod rust {
    use super::{xxh32,XXState};
    use test::test::BenchHarness;

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

    #[test]
    fn test_chunks() {
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
            let v = buf.slice(0, size);
            let mut xxh: XXState = XXState::new(seed);
            for chunk in v.chunks(15) {
                xxh.update(chunk);
            }

            let result = xxh.digest();
            assert_eq!(result, expected);
        };

        test(1,                0,      0xB85CBEE5);
        test(1,                PRIME,  0xD5845D64);
        test(14,               0,      0xE5AA0AB4);
        test(14,               PRIME,  0x4481951D);
        test(BUFSIZE,          0,      0x1F1AA412);
        test(BUFSIZE,          PRIME,  0x498EC8E2);
    }

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
        bench_base( bench, |v| {
            let mut xxh: XXState = XXState::new(0);
            for chunk in v.chunks(chunk_size) {
                xxh.update(chunk);
            }
            xxh.digest();
        });
    }

//     #[bench]
//     fn iterbytes(bench: &mut BenchHarness) {
//         use std::to_bytes::{IterBytes};
//         bench_base( bench, |v| {
//             let mut xxh: XXState = XXState::new(0);
//             v.iter_bytes(true, |bytes| {
//                 xxh.update(bytes);
//                 true
//             });
//             xxh.digest();
//         });
//     }

    #[bench]
    fn oneshot(bench: &mut BenchHarness) {
        bench_base( bench, |v| {
            xxh32(v, 0);
        });
    }

    #[bench]
    fn chunks_01(bench: &mut BenchHarness) {
        bench_chunks(bench, 1);
    }

    #[bench]
    fn chunks_02(bench: &mut BenchHarness) {
        bench_chunks(bench, 2);
    }

    #[bench]
    fn chunks_04(bench: &mut BenchHarness) {
        bench_chunks(bench, 4);
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
