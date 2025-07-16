# testable-simd-models

This crates contains models for the intrinsics provided by `core::arch`. Its structure is based off of
[rust-lang/stdarch/crates/core_arch](https://github.com/rust-lang/stdarch/tree/master/crates/core_arch). Within the `core_arch` folder in this crate, there is a different
folder for each architecture whose intrinsics are being implemented (corresponding to folders in the previous link). Each such
folder has 3 sub-folders, `models`, `tests`, and `specs`. 

The `models` folder contains the models of the intrinsics, with a file corresponding to different target features, 
and are written using the various abstractions implementedin `crate::abstractions`, especially those 
in `crate::abstractions::simd`. These models are meant to closely resemble their implementations within
the Rust core itself.

The `tests` folder contains the tests of these models, and is structured the same way as `models`. Each file 
additionally contains the definition of a macro that makes writing these tests easier. The tests
work by testing the models against the intrinsics in the Rust core, trying out random inputs
(generally 1000), and comparing their outputs.

The `specs` folder contains specifications. These are implementatioons written without
using the function abstractions in `crate::abstractions::simd`, and are written to be
match their vendor specification as closely as possible.

The process of adding a specific intrinsic's model goes as follows. For this example,
let us say the intrinsic we are adding is `_mm256_bsrli_epi128` from the avx2 feature set.

1. We go to [rust-lang/stdarch/crates/core_arch/src/x86/](https://github.com/rust-lang/stdarch/tree/master/crates/core_arch/src/x86/), and find the implementation of the intrinsic in `avx2.rs`.
2. We see that the implementation looks like this:
``` rust
/// Shifts 128-bit lanes in `a` right by `imm8` bytes while shifting in zeros.
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_bsrli_epi128)
#[inline]
#[target_feature(enable = "avx2")]
#[cfg_attr(test, assert_instr(vpsrldq, IMM8 = 1))]
#[rustc_legacy_const_generics(1)]
#[stable(feature = "simd_x86", since = "1.27.0")]
pub fn _mm256_bsrli_epi128<const IMM8: i32>(a: __m256i) -> __m256i {
    static_assert_uimm_bits!(IMM8, 8);
    const fn mask(shift: i32, i: u32) -> u32 {
        let shift = shift as u32 & 0xff;
        if shift > 15 || (15 - (i % 16)) < shift {
            0
        } else {
            32 + (i + shift)
        }
    }
    unsafe {
        let a = a.as_i8x32();
        let r: i8x32 = simd_shuffle!(
            i8x32::ZERO,
            a,
            [
                mask(IMM8, 0),
                mask(IMM8, 1),
                mask(IMM8, 2),
                mask(IMM8, 3),
                mask(IMM8, 4),
                mask(IMM8, 5),
                mask(IMM8, 6),
                mask(IMM8, 7),
                mask(IMM8, 8),
                mask(IMM8, 9),
                mask(IMM8, 10),
                mask(IMM8, 11),
                mask(IMM8, 12),
                mask(IMM8, 13),
                mask(IMM8, 14),
                mask(IMM8, 15),
                mask(IMM8, 16),
                mask(IMM8, 17),
                mask(IMM8, 18),
                mask(IMM8, 19),
                mask(IMM8, 20),
                mask(IMM8, 21),
                mask(IMM8, 22),
                mask(IMM8, 23),
                mask(IMM8, 24),
                mask(IMM8, 25),
                mask(IMM8, 26),
                mask(IMM8, 27),
                mask(IMM8, 28),
                mask(IMM8, 29),
                mask(IMM8, 30),
                mask(IMM8, 31),
            ],
        );
        transmute(r)
    }
}
  ```
Thus, we then go to to `core_arch/x86/models/avx2.rs`, and add the implementation. After some modification, it ends up looking like this.
``` rust
/// Shifts 128-bit lanes in `a` right by `imm8` bytes while shifting in zeros.
///
/// [Intel's documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_bsrli_epi128)

pub fn _mm256_bsrli_epi128<const IMM8: i32>(a: __m256i) -> __m256i {
    const fn mask(shift: i32, i: u32) -> u64 {
        let shift = shift as u32 & 0xff;
        if shift > 15 || (15 - (i % 16)) < shift {
            0 as u64
        } else {
            (32 + (i + shift)) as u64
        }
    }
    
	let a = BitVec::to_i8x32(a);
	let r: i8x32 = simd_shuffle(
		i8x32::from_fn(|_| 0),
		a,
		[
			mask(IMM8, 0),
			mask(IMM8, 1),
			mask(IMM8, 2),
			mask(IMM8, 3),
			mask(IMM8, 4),
			mask(IMM8, 5),
			mask(IMM8, 6),
			mask(IMM8, 7),
			mask(IMM8, 8),
			mask(IMM8, 9),
			mask(IMM8, 10),
			mask(IMM8, 11),
			mask(IMM8, 12),
			mask(IMM8, 13),
			mask(IMM8, 14),
			mask(IMM8, 15),
			mask(IMM8, 16),
			mask(IMM8, 17),
			mask(IMM8, 18),
			mask(IMM8, 19),
			mask(IMM8, 20),
			mask(IMM8, 21),
			mask(IMM8, 22),
			mask(IMM8, 23),
			mask(IMM8, 24),
			mask(IMM8, 25),
			mask(IMM8, 26),
			mask(IMM8, 27),
			mask(IMM8, 28),
			mask(IMM8, 29),
			mask(IMM8, 30),
			mask(IMM8, 31),
		],
	);
	r.into()
}
  ```
  
3. Next, we add a test for this intrinsic. For this, we navigate to `core_arch/avx2/tests/avx2.rs`. Since the value of
   `IMM8` can be up to 8 bits, we want to test constant arguments up to 255. Thus, we write the following macro invocation.
   ```rust
	   mk!([100]_mm256_bsrli_epi128{<0>,<1>,<2>,<3>,<4>,<5>,<6>,<7>,<8>,<9>,<10>,<11>,<12>,<13>,<14>,<15>,<16>,<17>,<18>,<19>,<20>,<21>,<22>,<23>,<24>,<25>,<26>,<27>,<28>,<29>,<30>,<31>,<32>,<33>,<34>,<35>,<36>,<37>,<38>,<39>,<40>,<41>,<42>,<43>,<44>,<45>,<46>,<47>,<48>,<49>,<50>,<51>,<52>,<53>,<54>,<55>,<56>,<57>,<58>,<59>,<60>,<61>,<62>,<63>,<64>,<65>,<66>,<67>,<68>,<69>,<70>,<71>,<72>,<73>,<74>,<75>,<76>,<77>,<78>,<79>,<80>,<81>,<82>,<83>,<84>,<85>,<86>,<87>,<88>,<89>,<90>,<91>,<92>,<93>,<94>,<95>,<96>,<97>,<98>,<99>,<100>,<101>,<102>,<103>,<104>,<105>,<106>,<107>,<108>,<109>,<110>,<111>,<112>,<113>,<114>,<115>,<116>,<117>,<118>,<119>,<120>,<121>,<122>,<123>,<124>,<125>,<126>,<127>,<128>,<129>,<130>,<131>,<132>,<133>,<134>,<135>,<136>,<137>,<138>,<139>,<140>,<141>,<142>,<143>,<144>,<145>,<146>,<147>,<148>,<149>,<150>,<151>,<152>,<153>,<154>,<155>,<156>,<157>,<158>,<159>,<160>,<161>,<162>,<163>,<164>,<165>,<166>,<167>,<168>,<169>,<170>,<171>,<172>,<173>,<174>,<175>,<176>,<177>,<178>,<179>,<180>,<181>,<182>,<183>,<184>,<185>,<186>,<187>,<188>,<189>,<190>,<191>,<192>,<193>,<194>,<195>,<196>,<197>,<198>,<199>,<200>,<201>,<202>,<203>,<204>,<205>,<206>,<207>,<208>,<209>,<210>,<211>,<212>,<213>,<214>,<215>,<216>,<217>,<218>,<219>,<220>,<221>,<222>,<223>,<224>,<225>,<226>,<227>,<228>,<229>,<230>,<231>,<232>,<233>,<234>,<235>,<236>,<237>,<238>,<239>,<240>,<241>,<242>,<243>,<244>,<245>,<246>,<247>,<248>,<249>,<250>,<251>,<252>,<253>,<254>,<255>}(a: BitVec));
   ```
   Here, the `[100]` means we test 100 random inputs for each constant value. This concludes the necessary steps for implementing an intrinsic.
4. Optionally, we may want to add a specification, since the code for the Rust implemetation is non straightforward. For this, we look up the [Intel Documentation](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_bsrli_epi128).
   Based on the documentation, we may write the following specification.
   ```rust
   pub fn _mm256_bsrli_epi128<const IMM8: i32>(a: __m256i) -> __m256i {
		let a = BitVec::to_i128x2(a);
		let a = i128x2::from_fn(|i| {
			let tmp = IMM8 % 256;
			let tmp = tmp % 16;
			((a[i] as u128) >> (tmp * 8)) as i128
		});
		BitVec::from_i128x2(a)
	}
   ```
   


