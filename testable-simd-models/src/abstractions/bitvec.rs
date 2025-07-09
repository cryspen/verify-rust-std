//! This module provides a specification-friendly bit vector type.
use super::bit::{Bit, MachineInteger};
use super::funarr::*;

use std::fmt::Formatter;


// TODO: this module uses `u128/i128` as mathematic integers. We should use `hax_lib::int` or bigint.

/// A fixed-size bit vector type.
///
/// `BitVec<N>` is a specification-friendly, fixed-length bit vector that internally
/// stores an array of [`Bit`] values, where each `Bit` represents a single binary digit (0 or 1).
///
/// This type provides several utility methods for constructing and converting bit vectors:
///
/// The [`Debug`] implementation for `BitVec` pretty-prints the bits in groups of eight,
/// making the bit pattern more human-readable. The type also implements indexing,
/// allowing for easy access to individual bits.
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BitVec<const N: u64>(FunArray<N, Bit>);

/// Pretty prints a bit slice by group of 8#[hax_lib::exclude]
fn bit_slice_to_string(bits: &[Bit]) -> String {
    bits.iter()
        .map(|bit| match bit {
            Bit::Zero => '0',
            Bit::One => '1',
        })
        .collect::<Vec<_>>()
        .chunks(8)
        .map(|bits| bits.iter().collect::<String>())
        .map(|s| format!("{s} "))
        .collect::<String>()
        .trim()
        .into()
}


impl<const N: u64> core::fmt::Debug for BitVec<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", bit_slice_to_string(&self.0.as_vec()))
    }
}


impl<const N: u64> core::ops::Index<u64> for BitVec<N> {
    type Output = Bit;
    fn index(&self, index: u64) -> &Self::Output {
        self.0.get(index)
    }
}

/// Convert a bit slice into an unsigned number.

fn u128_int_from_bit_slice(bits: &[Bit]) -> u128 {
    bits.iter()
        .enumerate()
        .map(|(i, bit)| u128::from(*bit) << i)
        .sum::<u128>()
}

/// Convert a bit slice into a machine integer of type `T`.
fn int_from_bit_slice<T: TryFrom<i128> + MachineInteger + Copy>(bits: &[Bit]) -> T {
    debug_assert!(bits.len() <= T::bits() as usize);
    let result = if T::SIGNED {
        let is_negative = matches!(bits[T::bits() as usize - 1], Bit::One);
        let s = u128_int_from_bit_slice(&bits[0..T::bits() as usize - 1]) as i128;
        if is_negative {
            s + (-2i128).pow(T::bits() - 1)
        } else {
            s
        }
    } else {
        u128_int_from_bit_slice(bits) as i128
    };
    let Ok(n) = result.try_into() else {
        // Conversion must succeed as `result` is guaranteed to be in range due to the bit-length check.
        unreachable!()
    };
    n
}
impl<const N: u64> BitVec<N> {
    /// Constructor for BitVec. `BitVec::<N>::from_fn` constructs a bitvector out of a function that takes usizes smaller than `N` and produces bits.
    pub fn from_fn<F: Fn(u64) -> Bit>(f: F) -> Self {
        Self(FunArray::from_fn(f))
    }
    /// Convert a slice of machine integers where only the `d` least significant bits are relevant.
    pub fn from_slice<T: Into<i128> + MachineInteger + Copy>(x: &[T], d: u64) -> Self {
        Self::from_fn(|i| Bit::of_int::<T>(x[(i / d) as usize], (i % d) as u32))
    }

    /// Construct a BitVec out of a machine integer.
    pub fn from_int<T: Into<i128> + MachineInteger + Copy>(n: T) -> Self {
        Self::from_slice::<T>(&[n], T::bits() as u64)
    }

    /// Convert a BitVec into a machine integer of type `T`.
    pub fn to_int<T: TryFrom<i128> + MachineInteger + Copy>(self) -> T {
        int_from_bit_slice(&self.0.as_vec())
    }

    /// Convert a BitVec into a vector of machine integers of type `T`.
    pub fn to_vec<T: TryFrom<i128> + MachineInteger + Copy>(&self) -> Vec<T> {
        self.0
            .as_vec()
            .chunks(T::bits() as usize)
            .map(int_from_bit_slice)
            .collect()
    }

    /// Generate a random BitVec.
    pub fn rand() -> Self {
        use rand::prelude::*;
        let random_source: Vec<_> = {
            let mut rng = rand::rng();
            (0..N).map(|_| rng.random::<bool>()).collect()
        };
        Self::from_fn(|i| random_source[i as usize].into())
    }
}


impl<const N: u64> BitVec<N> {

    pub fn chunked_shift<const CHUNK: u64, const SHIFTS: u64>(
        self,
        shl: FunArray<SHIFTS, i128>,
    ) -> BitVec<N> {

        fn chunked_shift<const N: u64, const CHUNK: u64, const SHIFTS: u64>(
            bitvec: BitVec<N>,
            shl: FunArray<SHIFTS, i128>,
        ) -> BitVec<N> {
            BitVec::from_fn(|i| {
                let nth_bit = i % CHUNK;
                let nth_chunk = i / CHUNK;
                let shift: i128 = if nth_chunk < SHIFTS {
                    shl[nth_chunk]
                } else {
                    0
                };
                let local_index = (nth_bit as i128).wrapping_sub(shift);
                if local_index < CHUNK as i128 && local_index >= 0 {
                    let local_index = local_index as u64;
                    bitvec[nth_chunk * CHUNK + local_index]
                } else {
                    Bit::Zero
                }
            })
        }
        chunked_shift::<N, CHUNK, SHIFTS>(self, shl)
    }

    /// Folds over the array, accumulating a result.
    ///
    /// # Arguments
    /// * `init` - The initial value of the accumulator.
    /// * `f` - A function combining the accumulator and each element.
    pub fn fold<A>(&self, init: A, f: fn(A, Bit) -> A) -> A {
        self.0.fold(init, f)
    }
}

pub mod int_vec_interp {
    //! This module defines interpretation for bit vectors as vectors of machine integers of various size and signedness.
    use super::*;

    /// An F* attribute that marks an item as being an interpretation lemma.
    #[allow(dead_code)]
    /// Derives interpretations functions, simplification lemmas and type
    /// synonyms.
    macro_rules! interpretations {
        ($n:literal; $($name:ident [$ty:ty; $m:literal]),*) => {
            $(
                #[doc = concat!(stringify!($ty), " vectors of size ", stringify!($m))]
                #[allow(non_camel_case_types)]
                pub type $name = FunArray<$m, $ty>;
                pastey::paste! {
                    const _: ()  = {
                        impl BitVec<$n> {
                            #[doc = concat!("Conversion from ", stringify!($ty), " vectors of size ", stringify!($m), "to  bit vectors of size ", stringify!($n))]
                            pub fn [< from_ $name >](iv: $name) -> BitVec<$n> {
                                let vec: Vec<$ty> = iv.as_vec();
                                Self::from_slice(&vec[..], <$ty>::bits() as u64)
                            }
                            #[doc = concat!("Conversion from bit vectors of size ", stringify!($n), " to ", stringify!($ty), " vectors of size ", stringify!($m))]
                            pub fn [< to_ $name >](bv: BitVec<$n>) -> $name {
                                let vec: Vec<$ty> = bv.to_vec();
                                $name::from_fn(|i| vec[i as usize])
                            }


                        }

                        #[cfg(test)]
                        impl From<BitVec<$n>> for $name {
                            fn from(bv: BitVec<$n>) -> Self {
                                BitVec::[< to_ $name >](bv)
                            }
                        }

                        impl From<$name> for BitVec<$n> {
                            fn from(iv: $name) -> Self {
                                BitVec::[< from_ $name >](iv)
                            }
                        }

			impl $name {

			    pub fn splat(value: $ty) -> Self {
				FunArray::from_fn(|_| value)
			    }
			}
                    };
                }
            )*
        };
    }

    interpretations!(256; i32x8 [i32; 8], i64x4 [i64; 4], i16x16 [i16; 16], i128x2 [i128; 2], i8x32 [i8; 32],
		     u32x8 [u32; 8], u64x4 [u64; 4], u16x16 [u16; 16], u8x32 [u8; 32]);
    interpretations!(128; i32x4 [i32; 4], i64x2 [i64; 2], i16x8 [i16; 8], i128x1 [i128; 1], i8x16 [i8; 16],
		     u32x4 [u32; 4], u64x2 [u64; 2], u16x8 [u16; 8], u8x16 [u8; 16]);

    interpretations!(512; u32x16 [u32; 16], u16x32 [u16; 32], i32x16 [i32; 16], i16x32 [i16; 32]);
    interpretations!(64; i64x1 [i64; 1], i32x2 [i32; 2], i16x4 [i16; 4], i8x8 [i8; 8], u64x1 [u64; 1], u32x2 [u32; 2],u16x4 [u16; 4], u8x8 [u8; 8]);
    interpretations!(32; i8x4 [i8; 4], u8x4 [u8; 4]);

    impl i64x4 {
        pub fn into_i32x8(self) -> i32x8 {
            i32x8::from_fn(|i| {
                let value = *self.get(i / 2);
                (if i % 2 == 0 { value } else { value >> 32 }) as i32
            })
        }
    }

    impl i32x8 {
        pub fn into_i64x4(self) -> i64x4 {
            i64x4::from_fn(|i| {
                let low = *self.get(2 * i) as u32 as u64;
                let high = *self.get(2 * i + 1) as i32 as i64;
                (high << 32) | low as i64
            })
        }
    }

    impl From<i64x4> for i32x8 {
        fn from(vec: i64x4) -> Self {
            vec.into_i32x8()
        }
    }

    #[cfg(test)]
    mod direct_convertions_tests {
        use super::*;
        use crate::helpers::test::HasRandom;

        #[test]
        fn into_i32x8() {
            for _ in 0..10000 {
                let x: i64x4 = i64x4::random();
                let y = x.into_i32x8();
                assert_eq!(BitVec::from_i64x4(x), BitVec::from_i32x8(y));
            }
        }
        #[test]
        fn into_i64x4() {
            let x: i32x8 = i32x8::random();
            let y = x.into_i64x4();
            assert_eq!(BitVec::from_i32x8(x), BitVec::from_i64x4(y));
        }
    }
}
