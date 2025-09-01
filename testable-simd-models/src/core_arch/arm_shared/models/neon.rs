use super::types::*;
use crate::abstractions::simd::*;

///CRC32 single round checksum for bytes (8 bits).
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/__crc32b)
pub fn __crc32b(crc: u32, data: u8) -> u32 {
    { ___crc32b(crc, data as u32) }
}
///CRC32-C single round checksum for bytes (8 bits).
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/__crc32cb)
pub fn __crc32cb(crc: u32, data: u8) -> u32 {
    { ___crc32cb(crc, data as u32) }
}
///CRC32-C single round checksum for quad words (64 bits).
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/__crc32cd)
pub fn __crc32cd(crc: u32, data: u64) -> u32 {
    let b: u32 = (data & 0xFFFFFFFF) as u32;
    let c: u32 = (data >> 32) as u32;
    { ___crc32cw(___crc32cw(crc, b), c) }
}
///CRC32-C single round checksum for bytes (16 bits).
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/__crc32ch)
pub fn __crc32ch(crc: u32, data: u16) -> u32 {
    { ___crc32ch(crc, data as u32) }
}
///CRC32-C single round checksum for bytes (32 bits).
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/__crc32cw)
pub fn __crc32cw(crc: u32, data: u32) -> u32 {
    { ___crc32cw(crc, data) }
}
///CRC32 single round checksum for quad words (64 bits).
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/__crc32d)
pub fn __crc32d(crc: u32, data: u64) -> u32 {
    let b: u32 = (data & 0xFFFFFFFF) as u32;
    let c: u32 = (data >> 32) as u32;
    { ___crc32w(___crc32w(crc, b), c) }
}
///CRC32 single round checksum for bytes (16 bits).
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/__crc32h)
pub fn __crc32h(crc: u32, data: u16) -> u32 {
    { ___crc32h(crc, data as u32) }
}
///CRC32 single round checksum for bytes (32 bits).
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/__crc32w)
pub fn __crc32w(crc: u32, data: u32) -> u32 {
    { ___crc32w(crc, data) }
}
///Signed Add and Accumulate Long Pairwise.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/priv_vpadal_s8)
fn priv_vpadal_s8(a: int16x4_t, b: int8x8_t) -> int16x4_t {
    { _priv_vpadal_s8(a, b) }
}
///Signed Add and Accumulate Long Pairwise.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/priv_vpadalq_s8)
fn priv_vpadalq_s8(a: int16x8_t, b: int8x16_t) -> int16x8_t {
    { _priv_vpadalq_s8(a, b) }
}
///Signed Add and Accumulate Long Pairwise.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/priv_vpadal_s16)
fn priv_vpadal_s16(a: int32x2_t, b: int16x4_t) -> int32x2_t {
    { _priv_vpadal_s16(a, b) }
}
///Signed Add and Accumulate Long Pairwise.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/priv_vpadalq_s16)
fn priv_vpadalq_s16(a: int32x4_t, b: int16x8_t) -> int32x4_t {
    { _priv_vpadalq_s16(a, b) }
}
///Signed Add and Accumulate Long Pairwise.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/priv_vpadal_s32)
fn priv_vpadal_s32(a: int64x1_t, b: int32x2_t) -> int64x1_t {
    { _priv_vpadal_s32(a, b) }
}
///Signed Add and Accumulate Long Pairwise.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/priv_vpadalq_s32)
fn priv_vpadalq_s32(a: int64x2_t, b: int32x4_t) -> int64x2_t {
    { _priv_vpadalq_s32(a, b) }
}
///Signed Add and Accumulate Long Pairwise.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/priv_vpadal_u8)
fn priv_vpadal_u8(a: uint16x4_t, b: uint8x8_t) -> uint16x4_t {
    { _priv_vpadal_u8(a, b) }
}
///Signed Add and Accumulate Long Pairwise.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/priv_vpadalq_u8)
fn priv_vpadalq_u8(a: uint16x8_t, b: uint8x16_t) -> uint16x8_t {
    { _priv_vpadalq_u8(a, b) }
}
///Signed Add and Accumulate Long Pairwise.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/priv_vpadal_u16)
fn priv_vpadal_u16(a: uint32x2_t, b: uint16x4_t) -> uint32x2_t {
    { _priv_vpadal_u16(a, b) }
}
///Signed Add and Accumulate Long Pairwise.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/priv_vpadalq_u16)
fn priv_vpadalq_u16(a: uint32x4_t, b: uint16x8_t) -> uint32x4_t {
    { _priv_vpadalq_u16(a, b) }
}
///Signed Add and Accumulate Long Pairwise.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/priv_vpadal_u32)
fn priv_vpadal_u32(a: uint64x1_t, b: uint32x2_t) -> uint64x1_t {
    { _priv_vpadal_u32(a, b) }
}
///Signed Add and Accumulate Long Pairwise.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/priv_vpadalq_u32)
fn priv_vpadalq_u32(a: uint64x2_t, b: uint32x4_t) -> uint64x2_t {
    { _priv_vpadalq_u32(a, b) }
}
///Absolute difference and accumulate (64-bit)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vaba_s16)
pub fn vaba_s16(a: int16x4_t, b: int16x4_t, c: int16x4_t) -> int16x4_t {
    { simd_add(a, vabd_s16(b, c)) }
}
///Absolute difference and accumulate (64-bit)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vaba_s32)
pub fn vaba_s32(a: int32x2_t, b: int32x2_t, c: int32x2_t) -> int32x2_t {
    { simd_add(a, vabd_s32(b, c)) }
}
///Absolute difference and accumulate (64-bit)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vaba_s8)
pub fn vaba_s8(a: int8x8_t, b: int8x8_t, c: int8x8_t) -> int8x8_t {
    { simd_add(a, vabd_s8(b, c)) }
}
///Absolute difference and accumulate (64-bit)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vaba_u16)
pub fn vaba_u16(a: uint16x4_t, b: uint16x4_t, c: uint16x4_t) -> uint16x4_t {
    { simd_add(a, vabd_u16(b, c)) }
}
///Absolute difference and accumulate (64-bit)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vaba_u32)
pub fn vaba_u32(a: uint32x2_t, b: uint32x2_t, c: uint32x2_t) -> uint32x2_t {
    { simd_add(a, vabd_u32(b, c)) }
}
///Absolute difference and accumulate (64-bit)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vaba_u8)
pub fn vaba_u8(a: uint8x8_t, b: uint8x8_t, c: uint8x8_t) -> uint8x8_t {
    { simd_add(a, vabd_u8(b, c)) }
}
///Signed Absolute difference and Accumulate Long
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vabal_s8)
pub fn vabal_s8(a: int16x8_t, b: int8x8_t, c: int8x8_t) -> int16x8_t {
    let d: int8x8_t = vabd_s8(b, c);
    {
        let e: uint8x8_t = simd_cast(d);
        simd_add(a, simd_cast(e))
    }
}
///Signed Absolute difference and Accumulate Long
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vabal_s16)
pub fn vabal_s16(a: int32x4_t, b: int16x4_t, c: int16x4_t) -> int32x4_t {
    let d: int16x4_t = vabd_s16(b, c);
    {
        let e: uint16x4_t = simd_cast(d);
        simd_add(a, simd_cast(e))
    }
}
///Signed Absolute difference and Accumulate Long
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vabal_s32)
pub fn vabal_s32(a: int64x2_t, b: int32x2_t, c: int32x2_t) -> int64x2_t {
    let d: int32x2_t = vabd_s32(b, c);
    {
        let e: uint32x2_t = simd_cast(d);
        simd_add(a, simd_cast(e))
    }
}
///Unsigned Absolute difference and Accumulate Long
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vabal_u8)
pub fn vabal_u8(a: uint16x8_t, b: uint8x8_t, c: uint8x8_t) -> uint16x8_t {
    let d: uint8x8_t = vabd_u8(b, c);
    { simd_add(a, simd_cast(d)) }
}
///Unsigned Absolute difference and Accumulate Long
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vabal_u16)
pub fn vabal_u16(a: uint32x4_t, b: uint16x4_t, c: uint16x4_t) -> uint32x4_t {
    let d: uint16x4_t = vabd_u16(b, c);
    { simd_add(a, simd_cast(d)) }
}
///Unsigned Absolute difference and Accumulate Long
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vabal_u32)
pub fn vabal_u32(a: uint64x2_t, b: uint32x2_t, c: uint32x2_t) -> uint64x2_t {
    let d: uint32x2_t = vabd_u32(b, c);
    { simd_add(a, simd_cast(d)) }
}
///Absolute difference and accumulate (128-bit)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vabaq_s16)
pub fn vabaq_s16(a: int16x8_t, b: int16x8_t, c: int16x8_t) -> int16x8_t {
    { simd_add(a, vabdq_s16(b, c)) }
}
///Absolute difference and accumulate (128-bit)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vabaq_s32)
pub fn vabaq_s32(a: int32x4_t, b: int32x4_t, c: int32x4_t) -> int32x4_t {
    { simd_add(a, vabdq_s32(b, c)) }
}
///Absolute difference and accumulate (128-bit)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vabaq_s8)
pub fn vabaq_s8(a: int8x16_t, b: int8x16_t, c: int8x16_t) -> int8x16_t {
    { simd_add(a, vabdq_s8(b, c)) }
}
///Absolute difference and accumulate (128-bit)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vabaq_u16)
pub fn vabaq_u16(a: uint16x8_t, b: uint16x8_t, c: uint16x8_t) -> uint16x8_t {
    { simd_add(a, vabdq_u16(b, c)) }
}
///Absolute difference and accumulate (128-bit)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vabaq_u32)
pub fn vabaq_u32(a: uint32x4_t, b: uint32x4_t, c: uint32x4_t) -> uint32x4_t {
    { simd_add(a, vabdq_u32(b, c)) }
}
///Absolute difference and accumulate (128-bit)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vabaq_u8)
pub fn vabaq_u8(a: uint8x16_t, b: uint8x16_t, c: uint8x16_t) -> uint8x16_t {
    { simd_add(a, vabdq_u8(b, c)) }
}
///Absolute difference between the arguments of Floating
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vabd_f16)
pub fn vabd_f16(a: float16x4_t, b: float16x4_t) -> float16x4_t {
    { _vabd_f16(a, b) }
}
///Absolute difference between the arguments of Floating
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vabdq_f16)
pub fn vabdq_f16(a: float16x8_t, b: float16x8_t) -> float16x8_t {
    { _vabdq_f16(a, b) }
}
///Absolute difference between the arguments of Floating
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vabd_f32)
pub fn vabd_f32(a: float32x2_t, b: float32x2_t) -> float32x2_t {
    { _vabd_f32(a, b) }
}
///Absolute difference between the arguments of Floating
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vabdq_f32)
pub fn vabdq_f32(a: float32x4_t, b: float32x4_t) -> float32x4_t {
    { _vabdq_f32(a, b) }
}
///Absolute difference between the arguments
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vabd_s8)
pub fn vabd_s8(a: int8x8_t, b: int8x8_t) -> int8x8_t {
    { _vabd_s8(a, b) }
}
///Absolute difference between the arguments
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vabdq_s8)
pub fn vabdq_s8(a: int8x16_t, b: int8x16_t) -> int8x16_t {
    { _vabdq_s8(a, b) }
}
///Absolute difference between the arguments
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vabd_s16)
pub fn vabd_s16(a: int16x4_t, b: int16x4_t) -> int16x4_t {
    { _vabd_s16(a, b) }
}
///Absolute difference between the arguments
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vabdq_s16)
pub fn vabdq_s16(a: int16x8_t, b: int16x8_t) -> int16x8_t {
    { _vabdq_s16(a, b) }
}
///Absolute difference between the arguments
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vabd_s32)
pub fn vabd_s32(a: int32x2_t, b: int32x2_t) -> int32x2_t {
    { _vabd_s32(a, b) }
}
///Absolute difference between the arguments
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vabdq_s32)
pub fn vabdq_s32(a: int32x4_t, b: int32x4_t) -> int32x4_t {
    { _vabdq_s32(a, b) }
}
///Absolute difference between the arguments
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vabd_u8)
pub fn vabd_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
    { _vabd_u8(a, b) }
}
///Absolute difference between the arguments
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vabdq_u8)
pub fn vabdq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
    { _vabdq_u8(a, b) }
}
///Absolute difference between the arguments
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vabd_u16)
pub fn vabd_u16(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
    { _vabd_u16(a, b) }
}
///Absolute difference between the arguments
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vabdq_u16)
pub fn vabdq_u16(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
    { _vabdq_u16(a, b) }
}
///Absolute difference between the arguments
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vabd_u32)
pub fn vabd_u32(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
    { _vabd_u32(a, b) }
}
///Absolute difference between the arguments
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vabdq_u32)
pub fn vabdq_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
    { _vabdq_u32(a, b) }
}
///Signed Absolute difference Long
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vabdl_s8)
pub fn vabdl_s8(a: int8x8_t, b: int8x8_t) -> int16x8_t {
    {
        let c: uint8x8_t = simd_cast(vabd_s8(a, b));
        simd_cast(c)
    }
}
///Signed Absolute difference Long
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vabdl_s16)
pub fn vabdl_s16(a: int16x4_t, b: int16x4_t) -> int32x4_t {
    {
        let c: uint16x4_t = simd_cast(vabd_s16(a, b));
        simd_cast(c)
    }
}
///Signed Absolute difference Long
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vabdl_s32)
pub fn vabdl_s32(a: int32x2_t, b: int32x2_t) -> int64x2_t {
    {
        let c: uint32x2_t = simd_cast(vabd_s32(a, b));
        simd_cast(c)
    }
}
///Unsigned Absolute difference Long
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vabdl_u8)
pub fn vabdl_u8(a: uint8x8_t, b: uint8x8_t) -> uint16x8_t {
    { simd_cast(vabd_u8(a, b)) }
}
///Unsigned Absolute difference Long
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vabdl_u16)
pub fn vabdl_u16(a: uint16x4_t, b: uint16x4_t) -> uint32x4_t {
    { simd_cast(vabd_u16(a, b)) }
}
///Unsigned Absolute difference Long
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vabdl_u32)
pub fn vabdl_u32(a: uint32x2_t, b: uint32x2_t) -> uint64x2_t {
    { simd_cast(vabd_u32(a, b)) }
}
///Floating-point absolute value
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vabs_f16)
pub fn vabs_f16(a: float16x4_t) -> float16x4_t {
    { simd_fabs(a) }
}
///Floating-point absolute value
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vabsq_f16)
pub fn vabsq_f16(a: float16x8_t) -> float16x8_t {
    { simd_fabs(a) }
}
///Floating-point absolute value
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vabs_f32)
pub fn vabs_f32(a: float32x2_t) -> float32x2_t {
    { simd_fabs(a) }
}
///Floating-point absolute value
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vabsq_f32)
pub fn vabsq_f32(a: float32x4_t) -> float32x4_t {
    { simd_fabs(a) }
}
///Absolute value (wrapping).
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vabs_s8)
pub fn vabs_s8(a: int8x8_t) -> int8x8_t {
    { _vabs_s8(a) }
}
///Absolute value (wrapping).
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vabsq_s8)
pub fn vabsq_s8(a: int8x16_t) -> int8x16_t {
    { _vabsq_s8(a) }
}
///Absolute value (wrapping).
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vabs_s16)
pub fn vabs_s16(a: int16x4_t) -> int16x4_t {
    { _vabs_s16(a) }
}
///Absolute value (wrapping).
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vabsq_s16)
pub fn vabsq_s16(a: int16x8_t) -> int16x8_t {
    { _vabsq_s16(a) }
}
///Absolute value (wrapping).
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vabs_s32)
pub fn vabs_s32(a: int32x2_t) -> int32x2_t {
    { _vabs_s32(a) }
}
///Absolute value (wrapping).
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vabsq_s32)
pub fn vabsq_s32(a: int32x4_t) -> int32x4_t {
    { _vabsq_s32(a) }
}
///Floating-point absolute value
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vabsh_f16)
pub fn vabsh_f16(a: f16) -> f16 {
    { simd_extract(vabs_f16(vdup_n_f16(a)), 0) }
}
///Floating-point Add (vector).
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vadd_f16)
pub fn vadd_f16(a: float16x4_t, b: float16x4_t) -> float16x4_t {
    { simd_add(a, b) }
}
///Floating-point Add (vector).
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vaddq_f16)
pub fn vaddq_f16(a: float16x8_t, b: float16x8_t) -> float16x8_t {
    { simd_add(a, b) }
}
///Vector add.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vadd_f32)
pub fn vadd_f32(a: float32x2_t, b: float32x2_t) -> float32x2_t {
    { simd_add(a, b) }
}
///Vector add.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vadd_s16)
pub fn vadd_s16(a: int16x4_t, b: int16x4_t) -> int16x4_t {
    { simd_add(a, b) }
}
///Vector add.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vadd_s32)
pub fn vadd_s32(a: int32x2_t, b: int32x2_t) -> int32x2_t {
    { simd_add(a, b) }
}
///Vector add.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vadd_s8)
pub fn vadd_s8(a: int8x8_t, b: int8x8_t) -> int8x8_t {
    { simd_add(a, b) }
}
///Vector add.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vadd_u16)
pub fn vadd_u16(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
    { simd_add(a, b) }
}
///Vector add.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vadd_u32)
pub fn vadd_u32(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
    { simd_add(a, b) }
}
///Vector add.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vadd_u8)
pub fn vadd_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
    { simd_add(a, b) }
}
///Vector add.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vaddq_f32)
pub fn vaddq_f32(a: float32x4_t, b: float32x4_t) -> float32x4_t {
    { simd_add(a, b) }
}
///Vector add.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vaddq_s16)
pub fn vaddq_s16(a: int16x8_t, b: int16x8_t) -> int16x8_t {
    { simd_add(a, b) }
}
///Vector add.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vaddq_s32)
pub fn vaddq_s32(a: int32x4_t, b: int32x4_t) -> int32x4_t {
    { simd_add(a, b) }
}
///Vector add.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vaddq_s64)
pub fn vaddq_s64(a: int64x2_t, b: int64x2_t) -> int64x2_t {
    { simd_add(a, b) }
}
///Vector add.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vaddq_s8)
pub fn vaddq_s8(a: int8x16_t, b: int8x16_t) -> int8x16_t {
    { simd_add(a, b) }
}
///Vector add.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vaddq_u16)
pub fn vaddq_u16(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
    { simd_add(a, b) }
}
///Vector add.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vaddq_u32)
pub fn vaddq_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
    { simd_add(a, b) }
}
///Vector add.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vaddq_u64)
pub fn vaddq_u64(a: uint64x2_t, b: uint64x2_t) -> uint64x2_t {
    { simd_add(a, b) }
}
///Vector add.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vaddq_u8)
pub fn vaddq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
    { simd_add(a, b) }
}
///Bitwise exclusive OR
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vadd_p8)
pub fn vadd_p8(a: poly8x8_t, b: poly8x8_t) -> poly8x8_t {
    { simd_xor(a, b) }
}
///Bitwise exclusive OR
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vaddq_p8)
pub fn vaddq_p8(a: poly8x16_t, b: poly8x16_t) -> poly8x16_t {
    { simd_xor(a, b) }
}
///Bitwise exclusive OR
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vadd_p16)
pub fn vadd_p16(a: poly16x4_t, b: poly16x4_t) -> poly16x4_t {
    { simd_xor(a, b) }
}
///Bitwise exclusive OR
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vaddq_p16)
pub fn vaddq_p16(a: poly16x8_t, b: poly16x8_t) -> poly16x8_t {
    { simd_xor(a, b) }
}
///Bitwise exclusive OR
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vadd_p64)
pub fn vadd_p64(a: poly64x1_t, b: poly64x1_t) -> poly64x1_t {
    { simd_xor(a, b) }
}
///Bitwise exclusive OR
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vaddq_p64)
pub fn vaddq_p64(a: poly64x2_t, b: poly64x2_t) -> poly64x2_t {
    { simd_xor(a, b) }
}
///Add
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vaddh_f16)
pub fn vaddh_f16(a: f16, b: f16) -> f16 {
    a + b
}
///Add returning High Narrow (high half).
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vaddhn_high_s16)
pub fn vaddhn_high_s16(r: int8x8_t, a: int16x8_t, b: int16x8_t) -> int8x16_t {
    {
        let x = simd_cast(simd_shr(simd_add(a, b), int16x8_t::splat(8)));
        simd_shuffle(r, x, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15])
    }
}
///Add returning High Narrow (high half).
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vaddhn_high_s32)
pub fn vaddhn_high_s32(r: int16x4_t, a: int32x4_t, b: int32x4_t) -> int16x8_t {
    {
        let x = simd_cast(simd_shr(simd_add(a, b), int32x4_t::splat(16)));
        simd_shuffle(r, x, [0, 1, 2, 3, 4, 5, 6, 7])
    }
}
///Add returning High Narrow (high half).
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vaddhn_high_s64)
pub fn vaddhn_high_s64(r: int32x2_t, a: int64x2_t, b: int64x2_t) -> int32x4_t {
    {
        let x = simd_cast(simd_shr(simd_add(a, b), int64x2_t::splat(32)));
        simd_shuffle(r, x, [0, 1, 2, 3])
    }
}
///Add returning High Narrow (high half).
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vaddhn_high_u16)
pub fn vaddhn_high_u16(r: uint8x8_t, a: uint16x8_t, b: uint16x8_t) -> uint8x16_t {
    {
        let x = simd_cast(simd_shr(simd_add(a, b), uint16x8_t::splat(8)));
        simd_shuffle(r, x, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15])
    }
}
///Add returning High Narrow (high half).
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vaddhn_high_u32)
pub fn vaddhn_high_u32(r: uint16x4_t, a: uint32x4_t, b: uint32x4_t) -> uint16x8_t {
    {
        let x = simd_cast(simd_shr(simd_add(a, b), uint32x4_t::splat(16)));
        simd_shuffle(r, x, [0, 1, 2, 3, 4, 5, 6, 7])
    }
}
///Add returning High Narrow (high half).
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vaddhn_high_u64)
pub fn vaddhn_high_u64(r: uint32x2_t, a: uint64x2_t, b: uint64x2_t) -> uint32x4_t {
    {
        let x = simd_cast(simd_shr(simd_add(a, b), uint64x2_t::splat(32)));
        simd_shuffle(r, x, [0, 1, 2, 3])
    }
}
///Add returning High Narrow.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vaddhn_s16)
pub fn vaddhn_s16(a: int16x8_t, b: int16x8_t) -> int8x8_t {
    { simd_cast(simd_shr(simd_add(a, b), int16x8_t::splat(8))) }
}
///Add returning High Narrow.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vaddhn_s32)
pub fn vaddhn_s32(a: int32x4_t, b: int32x4_t) -> int16x4_t {
    { simd_cast(simd_shr(simd_add(a, b), int32x4_t::splat(16))) }
}
///Add returning High Narrow.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vaddhn_s64)
pub fn vaddhn_s64(a: int64x2_t, b: int64x2_t) -> int32x2_t {
    { simd_cast(simd_shr(simd_add(a, b), int64x2_t::splat(32))) }
}
///Add returning High Narrow.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vaddhn_u16)
pub fn vaddhn_u16(a: uint16x8_t, b: uint16x8_t) -> uint8x8_t {
    { simd_cast(simd_shr(simd_add(a, b), uint16x8_t::splat(8))) }
}
///Add returning High Narrow.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vaddhn_u32)
pub fn vaddhn_u32(a: uint32x4_t, b: uint32x4_t) -> uint16x4_t {
    { simd_cast(simd_shr(simd_add(a, b), uint32x4_t::splat(16))) }
}
///Add returning High Narrow.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vaddhn_u64)
pub fn vaddhn_u64(a: uint64x2_t, b: uint64x2_t) -> uint32x2_t {
    { simd_cast(simd_shr(simd_add(a, b), uint64x2_t::splat(32))) }
}
///Signed Add Long (vector, high half).
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vaddl_high_s16)
pub fn vaddl_high_s16(a: int16x8_t, b: int16x8_t) -> int32x4_t {
    {
        let a: int16x4_t = simd_shuffle(a, a, [4, 5, 6, 7]);
        let b: int16x4_t = simd_shuffle(b, b, [4, 5, 6, 7]);
        let a: int32x4_t = simd_cast(a);
        let b: int32x4_t = simd_cast(b);
        simd_add(a, b)
    }
}
///Signed Add Long (vector, high half).
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vaddl_high_s32)
pub fn vaddl_high_s32(a: int32x4_t, b: int32x4_t) -> int64x2_t {
    {
        let a: int32x2_t = simd_shuffle(a, a, [2, 3]);
        let b: int32x2_t = simd_shuffle(b, b, [2, 3]);
        let a: int64x2_t = simd_cast(a);
        let b: int64x2_t = simd_cast(b);
        simd_add(a, b)
    }
}
///Signed Add Long (vector, high half).
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vaddl_high_s8)
pub fn vaddl_high_s8(a: int8x16_t, b: int8x16_t) -> int16x8_t {
    {
        let a: int8x8_t = simd_shuffle(a, a, [8, 9, 10, 11, 12, 13, 14, 15]);
        let b: int8x8_t = simd_shuffle(b, b, [8, 9, 10, 11, 12, 13, 14, 15]);
        let a: int16x8_t = simd_cast(a);
        let b: int16x8_t = simd_cast(b);
        simd_add(a, b)
    }
}
///Signed Add Long (vector, high half).
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vaddl_high_u16)
pub fn vaddl_high_u16(a: uint16x8_t, b: uint16x8_t) -> uint32x4_t {
    {
        let a: uint16x4_t = simd_shuffle(a, a, [4, 5, 6, 7]);
        let b: uint16x4_t = simd_shuffle(b, b, [4, 5, 6, 7]);
        let a: uint32x4_t = simd_cast(a);
        let b: uint32x4_t = simd_cast(b);
        simd_add(a, b)
    }
}
///Signed Add Long (vector, high half).
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vaddl_high_u32)
pub fn vaddl_high_u32(a: uint32x4_t, b: uint32x4_t) -> uint64x2_t {
    {
        let a: uint32x2_t = simd_shuffle(a, a, [2, 3]);
        let b: uint32x2_t = simd_shuffle(b, b, [2, 3]);
        let a: uint64x2_t = simd_cast(a);
        let b: uint64x2_t = simd_cast(b);
        simd_add(a, b)
    }
}
///Signed Add Long (vector, high half).
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vaddl_high_u8)
pub fn vaddl_high_u8(a: uint8x16_t, b: uint8x16_t) -> uint16x8_t {
    {
        let a: uint8x8_t = simd_shuffle(a, a, [8, 9, 10, 11, 12, 13, 14, 15]);
        let b: uint8x8_t = simd_shuffle(b, b, [8, 9, 10, 11, 12, 13, 14, 15]);
        let a: uint16x8_t = simd_cast(a);
        let b: uint16x8_t = simd_cast(b);
        simd_add(a, b)
    }
}
///Add Long (vector).
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vaddl_s16)
pub fn vaddl_s16(a: int16x4_t, b: int16x4_t) -> int32x4_t {
    {
        let a: int32x4_t = simd_cast(a);
        let b: int32x4_t = simd_cast(b);
        simd_add(a, b)
    }
}
///Add Long (vector).
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vaddl_s32)
pub fn vaddl_s32(a: int32x2_t, b: int32x2_t) -> int64x2_t {
    {
        let a: int64x2_t = simd_cast(a);
        let b: int64x2_t = simd_cast(b);
        simd_add(a, b)
    }
}
///Add Long (vector).
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vaddl_s8)
pub fn vaddl_s8(a: int8x8_t, b: int8x8_t) -> int16x8_t {
    {
        let a: int16x8_t = simd_cast(a);
        let b: int16x8_t = simd_cast(b);
        simd_add(a, b)
    }
}
///Add Long (vector).
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vaddl_u16)
pub fn vaddl_u16(a: uint16x4_t, b: uint16x4_t) -> uint32x4_t {
    {
        let a: uint32x4_t = simd_cast(a);
        let b: uint32x4_t = simd_cast(b);
        simd_add(a, b)
    }
}
///Add Long (vector).
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vaddl_u32)
pub fn vaddl_u32(a: uint32x2_t, b: uint32x2_t) -> uint64x2_t {
    {
        let a: uint64x2_t = simd_cast(a);
        let b: uint64x2_t = simd_cast(b);
        simd_add(a, b)
    }
}
///Add Long (vector).
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vaddl_u8)
pub fn vaddl_u8(a: uint8x8_t, b: uint8x8_t) -> uint16x8_t {
    {
        let a: uint16x8_t = simd_cast(a);
        let b: uint16x8_t = simd_cast(b);
        simd_add(a, b)
    }
}
///Bitwise exclusive OR
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vaddq_p128)
pub fn vaddq_p128(a: p128, b: p128) -> p128 {
    a ^ b
}
///Add Wide (high half).
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vaddw_high_s16)
pub fn vaddw_high_s16(a: int32x4_t, b: int16x8_t) -> int32x4_t {
    {
        let b: int16x4_t = simd_shuffle(b, b, [4, 5, 6, 7]);
        let b: int32x4_t = simd_cast(b);
        simd_add(a, b)
    }
}
///Add Wide (high half).
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vaddw_high_s32)
pub fn vaddw_high_s32(a: int64x2_t, b: int32x4_t) -> int64x2_t {
    {
        let b: int32x2_t = simd_shuffle(b, b, [2, 3]);
        let b: int64x2_t = simd_cast(b);
        simd_add(a, b)
    }
}
///Add Wide (high half).
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vaddw_high_s8)
pub fn vaddw_high_s8(a: int16x8_t, b: int8x16_t) -> int16x8_t {
    {
        let b: int8x8_t = simd_shuffle(b, b, [8, 9, 10, 11, 12, 13, 14, 15]);
        let b: int16x8_t = simd_cast(b);
        simd_add(a, b)
    }
}
///Add Wide (high half).
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vaddw_high_u16)
pub fn vaddw_high_u16(a: uint32x4_t, b: uint16x8_t) -> uint32x4_t {
    {
        let b: uint16x4_t = simd_shuffle(b, b, [4, 5, 6, 7]);
        let b: uint32x4_t = simd_cast(b);
        simd_add(a, b)
    }
}
///Add Wide (high half).
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vaddw_high_u32)
pub fn vaddw_high_u32(a: uint64x2_t, b: uint32x4_t) -> uint64x2_t {
    {
        let b: uint32x2_t = simd_shuffle(b, b, [2, 3]);
        let b: uint64x2_t = simd_cast(b);
        simd_add(a, b)
    }
}
///Add Wide (high half).
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vaddw_high_u8)
pub fn vaddw_high_u8(a: uint16x8_t, b: uint8x16_t) -> uint16x8_t {
    {
        let b: uint8x8_t = simd_shuffle(b, b, [8, 9, 10, 11, 12, 13, 14, 15]);
        let b: uint16x8_t = simd_cast(b);
        simd_add(a, b)
    }
}
///Add Wide
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vaddw_s16)
pub fn vaddw_s16(a: int32x4_t, b: int16x4_t) -> int32x4_t {
    {
        let b: int32x4_t = simd_cast(b);
        simd_add(a, b)
    }
}
///Add Wide
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vaddw_s32)
pub fn vaddw_s32(a: int64x2_t, b: int32x2_t) -> int64x2_t {
    {
        let b: int64x2_t = simd_cast(b);
        simd_add(a, b)
    }
}
///Add Wide
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vaddw_s8)
pub fn vaddw_s8(a: int16x8_t, b: int8x8_t) -> int16x8_t {
    {
        let b: int16x8_t = simd_cast(b);
        simd_add(a, b)
    }
}
///Add Wide
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vaddw_u16)
pub fn vaddw_u16(a: uint32x4_t, b: uint16x4_t) -> uint32x4_t {
    {
        let b: uint32x4_t = simd_cast(b);
        simd_add(a, b)
    }
}
///Add Wide
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vaddw_u32)
pub fn vaddw_u32(a: uint64x2_t, b: uint32x2_t) -> uint64x2_t {
    {
        let b: uint64x2_t = simd_cast(b);
        simd_add(a, b)
    }
}
///Add Wide
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vaddw_u8)
pub fn vaddw_u8(a: uint16x8_t, b: uint8x8_t) -> uint16x8_t {
    {
        let b: uint16x8_t = simd_cast(b);
        simd_add(a, b)
    }
}
///AES single round encryption.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vaesdq_u8)
pub fn vaesdq_u8(data: uint8x16_t, key: uint8x16_t) -> uint8x16_t {
    { _vaesdq_u8(data, key) }
}
///AES single round encryption.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vaeseq_u8)
pub fn vaeseq_u8(data: uint8x16_t, key: uint8x16_t) -> uint8x16_t {
    { _vaeseq_u8(data, key) }
}
///AES inverse mix columns.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vaesimcq_u8)
pub fn vaesimcq_u8(data: uint8x16_t) -> uint8x16_t {
    { _vaesimcq_u8(data) }
}
///AES mix columns.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vaesmcq_u8)
pub fn vaesmcq_u8(data: uint8x16_t) -> uint8x16_t {
    { _vaesmcq_u8(data) }
}
///Vector bitwise and
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vand_s8)
pub fn vand_s8(a: int8x8_t, b: int8x8_t) -> int8x8_t {
    { simd_and(a, b) }
}
///Vector bitwise and
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vandq_s8)
pub fn vandq_s8(a: int8x16_t, b: int8x16_t) -> int8x16_t {
    { simd_and(a, b) }
}
///Vector bitwise and
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vand_s16)
pub fn vand_s16(a: int16x4_t, b: int16x4_t) -> int16x4_t {
    { simd_and(a, b) }
}
///Vector bitwise and
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vandq_s16)
pub fn vandq_s16(a: int16x8_t, b: int16x8_t) -> int16x8_t {
    { simd_and(a, b) }
}
///Vector bitwise and
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vand_s32)
pub fn vand_s32(a: int32x2_t, b: int32x2_t) -> int32x2_t {
    { simd_and(a, b) }
}
///Vector bitwise and
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vandq_s32)
pub fn vandq_s32(a: int32x4_t, b: int32x4_t) -> int32x4_t {
    { simd_and(a, b) }
}
///Vector bitwise and
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vand_s64)
pub fn vand_s64(a: int64x1_t, b: int64x1_t) -> int64x1_t {
    { simd_and(a, b) }
}
///Vector bitwise and
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vandq_s64)
pub fn vandq_s64(a: int64x2_t, b: int64x2_t) -> int64x2_t {
    { simd_and(a, b) }
}
///Vector bitwise and
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vand_u8)
pub fn vand_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
    { simd_and(a, b) }
}
///Vector bitwise and
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vandq_u8)
pub fn vandq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
    { simd_and(a, b) }
}
///Vector bitwise and
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vand_u16)
pub fn vand_u16(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
    { simd_and(a, b) }
}
///Vector bitwise and
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vandq_u16)
pub fn vandq_u16(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
    { simd_and(a, b) }
}
///Vector bitwise and
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vand_u32)
pub fn vand_u32(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
    { simd_and(a, b) }
}
///Vector bitwise and
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vandq_u32)
pub fn vandq_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
    { simd_and(a, b) }
}
///Vector bitwise and
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vand_u64)
pub fn vand_u64(a: uint64x1_t, b: uint64x1_t) -> uint64x1_t {
    { simd_and(a, b) }
}
///Vector bitwise and
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vandq_u64)
pub fn vandq_u64(a: uint64x2_t, b: uint64x2_t) -> uint64x2_t {
    { simd_and(a, b) }
}
///Vector bitwise bit clear.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vbic_s16)
pub fn vbic_s16(a: int16x4_t, b: int16x4_t) -> int16x4_t {
    let c = int16x4_t::splat(-1);
    { simd_and(simd_xor(b, c), a) }
}
///Vector bitwise bit clear.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vbic_s32)
pub fn vbic_s32(a: int32x2_t, b: int32x2_t) -> int32x2_t {
    let c = int32x2_t::splat(-1);
    { simd_and(simd_xor(b, c), a) }
}
///Vector bitwise bit clear.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vbic_s64)
pub fn vbic_s64(a: int64x1_t, b: int64x1_t) -> int64x1_t {
    let c = int64x1_t::splat(-1);
    { simd_and(simd_xor(b, c), a) }
}
///Vector bitwise bit clear.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vbic_s8)
pub fn vbic_s8(a: int8x8_t, b: int8x8_t) -> int8x8_t {
    let c = int8x8_t::splat(-1);
    { simd_and(simd_xor(b, c), a) }
}
///Vector bitwise bit clear.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vbicq_s16)
pub fn vbicq_s16(a: int16x8_t, b: int16x8_t) -> int16x8_t {
    let c = int16x8_t::splat(-1);
    { simd_and(simd_xor(b, c), a) }
}
///Vector bitwise bit clear.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vbicq_s32)
pub fn vbicq_s32(a: int32x4_t, b: int32x4_t) -> int32x4_t {
    let c = int32x4_t::splat(-1);
    { simd_and(simd_xor(b, c), a) }
}
///Vector bitwise bit clear.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vbicq_s64)
pub fn vbicq_s64(a: int64x2_t, b: int64x2_t) -> int64x2_t {
    let c = int64x2_t::splat(-1);
    { simd_and(simd_xor(b, c), a) }
}
///Vector bitwise bit clear.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vbicq_s8)
pub fn vbicq_s8(a: int8x16_t, b: int8x16_t) -> int8x16_t {
    let c = int8x16_t::splat(-1);
    { simd_and(simd_xor(b, c), a) }
}
///Vector bitwise bit clear.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vbic_u16)
pub fn vbic_u16(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
    let c = int16x4_t::splat(-1);
    { simd_and(simd_xor(b, transmute(c)), a) }
}
///Vector bitwise bit clear.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vbic_u32)
pub fn vbic_u32(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
    let c = int32x2_t::splat(-1);
    { simd_and(simd_xor(b, transmute(c)), a) }
}
///Vector bitwise bit clear.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vbic_u64)
pub fn vbic_u64(a: uint64x1_t, b: uint64x1_t) -> uint64x1_t {
    let c = int64x1_t::splat(-1);
    { simd_and(simd_xor(b, transmute(c)), a) }
}
///Vector bitwise bit clear.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vbic_u8)
pub fn vbic_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
    let c = int8x8_t::splat(-1);
    { simd_and(simd_xor(b, transmute(c)), a) }
}
///Vector bitwise bit clear.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vbicq_u16)
pub fn vbicq_u16(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
    let c = int16x8_t::splat(-1);
    { simd_and(simd_xor(b, transmute(c)), a) }
}
///Vector bitwise bit clear.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vbicq_u32)
pub fn vbicq_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
    let c = int32x4_t::splat(-1);
    { simd_and(simd_xor(b, transmute(c)), a) }
}
///Vector bitwise bit clear.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vbicq_u64)
pub fn vbicq_u64(a: uint64x2_t, b: uint64x2_t) -> uint64x2_t {
    let c = int64x2_t::splat(-1);
    { simd_and(simd_xor(b, transmute(c)), a) }
}
///Vector bitwise bit clear.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vbicq_u8)
pub fn vbicq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
    let c = int8x16_t::splat(-1);
    { simd_and(simd_xor(b, transmute(c)), a) }
}
///Bitwise Select.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vbsl_f16)
pub fn vbsl_f16(a: uint16x4_t, b: float16x4_t, c: float16x4_t) -> float16x4_t {
    let not = int16x4_t::splat(-1);
    {
        transmute(
            simd_or(
                simd_and(a, transmute(b)),
                simd_and(simd_xor(a, transmute(not)), transmute(c)),
            ),
        )
    }
}
///Bitwise Select.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vbslq_f16)
pub fn vbslq_f16(a: uint16x8_t, b: float16x8_t, c: float16x8_t) -> float16x8_t {
    let not = int16x8_t::splat(-1);
    {
        transmute(
            simd_or(
                simd_and(a, transmute(b)),
                simd_and(simd_xor(a, transmute(not)), transmute(c)),
            ),
        )
    }
}
///Bitwise Select.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vbsl_f32)
pub fn vbsl_f32(a: uint32x2_t, b: float32x2_t, c: float32x2_t) -> float32x2_t {
    let not = int32x2_t::splat(-1);
    {
        transmute(
            simd_or(
                simd_and(a, transmute(b)),
                simd_and(simd_xor(a, transmute(not)), transmute(c)),
            ),
        )
    }
}
///Bitwise Select.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vbsl_p16)
pub fn vbsl_p16(a: uint16x4_t, b: poly16x4_t, c: poly16x4_t) -> poly16x4_t {
    let not = int16x4_t::splat(-1);
    {
        transmute(
            simd_or(
                simd_and(a, transmute(b)),
                simd_and(simd_xor(a, transmute(not)), transmute(c)),
            ),
        )
    }
}
///Bitwise Select.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vbsl_p8)
pub fn vbsl_p8(a: uint8x8_t, b: poly8x8_t, c: poly8x8_t) -> poly8x8_t {
    let not = int8x8_t::splat(-1);
    {
        transmute(
            simd_or(
                simd_and(a, transmute(b)),
                simd_and(simd_xor(a, transmute(not)), transmute(c)),
            ),
        )
    }
}
///Bitwise Select.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vbsl_s16)
pub fn vbsl_s16(a: uint16x4_t, b: int16x4_t, c: int16x4_t) -> int16x4_t {
    let not = int16x4_t::splat(-1);
    {
        transmute(
            simd_or(
                simd_and(a, transmute(b)),
                simd_and(simd_xor(a, transmute(not)), transmute(c)),
            ),
        )
    }
}
///Bitwise Select.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vbsl_s32)
pub fn vbsl_s32(a: uint32x2_t, b: int32x2_t, c: int32x2_t) -> int32x2_t {
    let not = int32x2_t::splat(-1);
    {
        transmute(
            simd_or(
                simd_and(a, transmute(b)),
                simd_and(simd_xor(a, transmute(not)), transmute(c)),
            ),
        )
    }
}
///Bitwise Select.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vbsl_s64)
pub fn vbsl_s64(a: uint64x1_t, b: int64x1_t, c: int64x1_t) -> int64x1_t {
    let not = int64x1_t::splat(-1);
    {
        transmute(
            simd_or(
                simd_and(a, transmute(b)),
                simd_and(simd_xor(a, transmute(not)), transmute(c)),
            ),
        )
    }
}
///Bitwise Select.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vbsl_s8)
pub fn vbsl_s8(a: uint8x8_t, b: int8x8_t, c: int8x8_t) -> int8x8_t {
    let not = int8x8_t::splat(-1);
    {
        transmute(
            simd_or(
                simd_and(a, transmute(b)),
                simd_and(simd_xor(a, transmute(not)), transmute(c)),
            ),
        )
    }
}
///Bitwise Select.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vbslq_f32)
pub fn vbslq_f32(a: uint32x4_t, b: float32x4_t, c: float32x4_t) -> float32x4_t {
    let not = int32x4_t::splat(-1);
    {
        transmute(
            simd_or(
                simd_and(a, transmute(b)),
                simd_and(simd_xor(a, transmute(not)), transmute(c)),
            ),
        )
    }
}
///Bitwise Select.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vbslq_p16)
pub fn vbslq_p16(a: uint16x8_t, b: poly16x8_t, c: poly16x8_t) -> poly16x8_t {
    let not = int16x8_t::splat(-1);
    {
        transmute(
            simd_or(
                simd_and(a, transmute(b)),
                simd_and(simd_xor(a, transmute(not)), transmute(c)),
            ),
        )
    }
}
///Bitwise Select.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vbslq_p8)
pub fn vbslq_p8(a: uint8x16_t, b: poly8x16_t, c: poly8x16_t) -> poly8x16_t {
    let not = int8x16_t::splat(-1);
    {
        transmute(
            simd_or(
                simd_and(a, transmute(b)),
                simd_and(simd_xor(a, transmute(not)), transmute(c)),
            ),
        )
    }
}
///Bitwise Select.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vbslq_s16)
pub fn vbslq_s16(a: uint16x8_t, b: int16x8_t, c: int16x8_t) -> int16x8_t {
    let not = int16x8_t::splat(-1);
    {
        transmute(
            simd_or(
                simd_and(a, transmute(b)),
                simd_and(simd_xor(a, transmute(not)), transmute(c)),
            ),
        )
    }
}
///Bitwise Select.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vbslq_s32)
pub fn vbslq_s32(a: uint32x4_t, b: int32x4_t, c: int32x4_t) -> int32x4_t {
    let not = int32x4_t::splat(-1);
    {
        transmute(
            simd_or(
                simd_and(a, transmute(b)),
                simd_and(simd_xor(a, transmute(not)), transmute(c)),
            ),
        )
    }
}
///Bitwise Select.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vbslq_s64)
pub fn vbslq_s64(a: uint64x2_t, b: int64x2_t, c: int64x2_t) -> int64x2_t {
    let not = int64x2_t::splat(-1);
    {
        transmute(
            simd_or(
                simd_and(a, transmute(b)),
                simd_and(simd_xor(a, transmute(not)), transmute(c)),
            ),
        )
    }
}
///Bitwise Select.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vbslq_s8)
pub fn vbslq_s8(a: uint8x16_t, b: int8x16_t, c: int8x16_t) -> int8x16_t {
    let not = int8x16_t::splat(-1);
    {
        transmute(
            simd_or(
                simd_and(a, transmute(b)),
                simd_and(simd_xor(a, transmute(not)), transmute(c)),
            ),
        )
    }
}
///Bitwise Select.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vbsl_u16)
pub fn vbsl_u16(a: uint16x4_t, b: uint16x4_t, c: uint16x4_t) -> uint16x4_t {
    let not = int16x4_t::splat(-1);
    { transmute(simd_or(simd_and(a, b), simd_and(simd_xor(a, transmute(not)), c))) }
}
///Bitwise Select.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vbsl_u32)
pub fn vbsl_u32(a: uint32x2_t, b: uint32x2_t, c: uint32x2_t) -> uint32x2_t {
    let not = int32x2_t::splat(-1);
    { transmute(simd_or(simd_and(a, b), simd_and(simd_xor(a, transmute(not)), c))) }
}
///Bitwise Select.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vbsl_u64)
pub fn vbsl_u64(a: uint64x1_t, b: uint64x1_t, c: uint64x1_t) -> uint64x1_t {
    let not = int64x1_t::splat(-1);
    { transmute(simd_or(simd_and(a, b), simd_and(simd_xor(a, transmute(not)), c))) }
}
///Bitwise Select.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vbsl_u8)
pub fn vbsl_u8(a: uint8x8_t, b: uint8x8_t, c: uint8x8_t) -> uint8x8_t {
    let not = int8x8_t::splat(-1);
    { transmute(simd_or(simd_and(a, b), simd_and(simd_xor(a, transmute(not)), c))) }
}
///Bitwise Select.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vbslq_u16)
pub fn vbslq_u16(a: uint16x8_t, b: uint16x8_t, c: uint16x8_t) -> uint16x8_t {
    let not = int16x8_t::splat(-1);
    { transmute(simd_or(simd_and(a, b), simd_and(simd_xor(a, transmute(not)), c))) }
}
///Bitwise Select.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vbslq_u32)
pub fn vbslq_u32(a: uint32x4_t, b: uint32x4_t, c: uint32x4_t) -> uint32x4_t {
    let not = int32x4_t::splat(-1);
    { transmute(simd_or(simd_and(a, b), simd_and(simd_xor(a, transmute(not)), c))) }
}
///Bitwise Select.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vbslq_u64)
pub fn vbslq_u64(a: uint64x2_t, b: uint64x2_t, c: uint64x2_t) -> uint64x2_t {
    let not = int64x2_t::splat(-1);
    { transmute(simd_or(simd_and(a, b), simd_and(simd_xor(a, transmute(not)), c))) }
}
///Bitwise Select.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vbslq_u8)
pub fn vbslq_u8(a: uint8x16_t, b: uint8x16_t, c: uint8x16_t) -> uint8x16_t {
    let not = int8x16_t::splat(-1);
    { transmute(simd_or(simd_and(a, b), simd_and(simd_xor(a, transmute(not)), c))) }
}
///Floating-point absolute compare greater than or equal
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcage_f16)
pub fn vcage_f16(a: float16x4_t, b: float16x4_t) -> uint16x4_t {
    { _vcage_f16(a, b) }
}
///Floating-point absolute compare greater than or equal
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcageq_f16)
pub fn vcageq_f16(a: float16x8_t, b: float16x8_t) -> uint16x8_t {
    { _vcageq_f16(a, b) }
}
///Floating-point absolute compare greater than or equal
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcage_f32)
pub fn vcage_f32(a: float32x2_t, b: float32x2_t) -> uint32x2_t {
    { _vcage_f32(a, b) }
}
///Floating-point absolute compare greater than or equal
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcageq_f32)
pub fn vcageq_f32(a: float32x4_t, b: float32x4_t) -> uint32x4_t {
    { _vcageq_f32(a, b) }
}
///Floating-point absolute compare greater than
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcagt_f16)
pub fn vcagt_f16(a: float16x4_t, b: float16x4_t) -> uint16x4_t {
    { _vcagt_f16(a, b) }
}
///Floating-point absolute compare greater than
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcagtq_f16)
pub fn vcagtq_f16(a: float16x8_t, b: float16x8_t) -> uint16x8_t {
    { _vcagtq_f16(a, b) }
}
///Floating-point absolute compare greater than
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcagt_f32)
pub fn vcagt_f32(a: float32x2_t, b: float32x2_t) -> uint32x2_t {
    { _vcagt_f32(a, b) }
}
///Floating-point absolute compare greater than
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcagtq_f32)
pub fn vcagtq_f32(a: float32x4_t, b: float32x4_t) -> uint32x4_t {
    { _vcagtq_f32(a, b) }
}
///Floating-point absolute compare less than or equal
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcale_f16)
pub fn vcale_f16(a: float16x4_t, b: float16x4_t) -> uint16x4_t {
    vcage_f16(b, a)
}
///Floating-point absolute compare less than or equal
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcaleq_f16)
pub fn vcaleq_f16(a: float16x8_t, b: float16x8_t) -> uint16x8_t {
    vcageq_f16(b, a)
}
///Floating-point absolute compare less than or equal
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcale_f32)
pub fn vcale_f32(a: float32x2_t, b: float32x2_t) -> uint32x2_t {
    vcage_f32(b, a)
}
///Floating-point absolute compare less than or equal
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcaleq_f32)
pub fn vcaleq_f32(a: float32x4_t, b: float32x4_t) -> uint32x4_t {
    vcageq_f32(b, a)
}
///Floating-point absolute compare less than
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcalt_f16)
pub fn vcalt_f16(a: float16x4_t, b: float16x4_t) -> uint16x4_t {
    vcagt_f16(b, a)
}
///Floating-point absolute compare less than
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcaltq_f16)
pub fn vcaltq_f16(a: float16x8_t, b: float16x8_t) -> uint16x8_t {
    vcagtq_f16(b, a)
}
///Floating-point absolute compare less than
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcalt_f32)
pub fn vcalt_f32(a: float32x2_t, b: float32x2_t) -> uint32x2_t {
    vcagt_f32(b, a)
}
///Floating-point absolute compare less than
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcaltq_f32)
pub fn vcaltq_f32(a: float32x4_t, b: float32x4_t) -> uint32x4_t {
    vcagtq_f32(b, a)
}
///Floating-point compare equal
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vceq_f16)
pub fn vceq_f16(a: float16x4_t, b: float16x4_t) -> uint16x4_t {
    { simd_eq(a, b) }
}
///Floating-point compare equal
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vceqq_f16)
pub fn vceqq_f16(a: float16x8_t, b: float16x8_t) -> uint16x8_t {
    { simd_eq(a, b) }
}
///Floating-point compare equal
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vceq_f32)
pub fn vceq_f32(a: float32x2_t, b: float32x2_t) -> uint32x2_t {
    { simd_eq(a, b) }
}
///Floating-point compare equal
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vceqq_f32)
pub fn vceqq_f32(a: float32x4_t, b: float32x4_t) -> uint32x4_t {
    { simd_eq(a, b) }
}
///Compare bitwise Equal (vector)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vceq_s8)
pub fn vceq_s8(a: int8x8_t, b: int8x8_t) -> uint8x8_t {
    { simd_eq(a, b) }
}
///Compare bitwise Equal (vector)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vceqq_s8)
pub fn vceqq_s8(a: int8x16_t, b: int8x16_t) -> uint8x16_t {
    { simd_eq(a, b) }
}
///Compare bitwise Equal (vector)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vceq_s16)
pub fn vceq_s16(a: int16x4_t, b: int16x4_t) -> uint16x4_t {
    { simd_eq(a, b) }
}
///Compare bitwise Equal (vector)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vceqq_s16)
pub fn vceqq_s16(a: int16x8_t, b: int16x8_t) -> uint16x8_t {
    { simd_eq(a, b) }
}
///Compare bitwise Equal (vector)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vceq_s32)
pub fn vceq_s32(a: int32x2_t, b: int32x2_t) -> uint32x2_t {
    { simd_eq(a, b) }
}
///Compare bitwise Equal (vector)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vceqq_s32)
pub fn vceqq_s32(a: int32x4_t, b: int32x4_t) -> uint32x4_t {
    { simd_eq(a, b) }
}
///Compare bitwise Equal (vector)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vceq_u8)
pub fn vceq_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
    { simd_eq(a, b) }
}
///Compare bitwise Equal (vector)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vceqq_u8)
pub fn vceqq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
    { simd_eq(a, b) }
}
///Compare bitwise Equal (vector)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vceq_u16)
pub fn vceq_u16(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
    { simd_eq(a, b) }
}
///Compare bitwise Equal (vector)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vceqq_u16)
pub fn vceqq_u16(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
    { simd_eq(a, b) }
}
///Compare bitwise Equal (vector)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vceq_u32)
pub fn vceq_u32(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
    { simd_eq(a, b) }
}
///Compare bitwise Equal (vector)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vceqq_u32)
pub fn vceqq_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
    { simd_eq(a, b) }
}
///Compare bitwise Equal (vector)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vceq_p8)
pub fn vceq_p8(a: poly8x8_t, b: poly8x8_t) -> uint8x8_t {
    { simd_eq(a, b) }
}
///Compare bitwise Equal (vector)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vceqq_p8)
pub fn vceqq_p8(a: poly8x16_t, b: poly8x16_t) -> uint8x16_t {
    { simd_eq(a, b) }
}
///Floating-point compare greater than or equal
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcge_f16)
pub fn vcge_f16(a: float16x4_t, b: float16x4_t) -> uint16x4_t {
    { simd_ge(a, b) }
}
///Floating-point compare greater than or equal
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcgeq_f16)
pub fn vcgeq_f16(a: float16x8_t, b: float16x8_t) -> uint16x8_t {
    { simd_ge(a, b) }
}
///Floating-point compare greater than or equal
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcge_f32)
pub fn vcge_f32(a: float32x2_t, b: float32x2_t) -> uint32x2_t {
    { simd_ge(a, b) }
}
///Floating-point compare greater than or equal
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcgeq_f32)
pub fn vcgeq_f32(a: float32x4_t, b: float32x4_t) -> uint32x4_t {
    { simd_ge(a, b) }
}
///Compare signed greater than or equal
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcge_s8)
pub fn vcge_s8(a: int8x8_t, b: int8x8_t) -> uint8x8_t {
    { simd_ge(a, b) }
}
///Compare signed greater than or equal
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcgeq_s8)
pub fn vcgeq_s8(a: int8x16_t, b: int8x16_t) -> uint8x16_t {
    { simd_ge(a, b) }
}
///Compare signed greater than or equal
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcge_s16)
pub fn vcge_s16(a: int16x4_t, b: int16x4_t) -> uint16x4_t {
    { simd_ge(a, b) }
}
///Compare signed greater than or equal
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcgeq_s16)
pub fn vcgeq_s16(a: int16x8_t, b: int16x8_t) -> uint16x8_t {
    { simd_ge(a, b) }
}
///Compare signed greater than or equal
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcge_s32)
pub fn vcge_s32(a: int32x2_t, b: int32x2_t) -> uint32x2_t {
    { simd_ge(a, b) }
}
///Compare signed greater than or equal
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcgeq_s32)
pub fn vcgeq_s32(a: int32x4_t, b: int32x4_t) -> uint32x4_t {
    { simd_ge(a, b) }
}
///Compare unsigned greater than or equal
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcge_u8)
pub fn vcge_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
    { simd_ge(a, b) }
}
///Compare unsigned greater than or equal
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcgeq_u8)
pub fn vcgeq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
    { simd_ge(a, b) }
}
///Compare unsigned greater than or equal
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcge_u16)
pub fn vcge_u16(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
    { simd_ge(a, b) }
}
///Compare unsigned greater than or equal
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcgeq_u16)
pub fn vcgeq_u16(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
    { simd_ge(a, b) }
}
///Compare unsigned greater than or equal
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcge_u32)
pub fn vcge_u32(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
    { simd_ge(a, b) }
}
///Compare unsigned greater than or equal
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcgeq_u32)
pub fn vcgeq_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
    { simd_ge(a, b) }
}
///Floating-point compare greater than or equal to zero
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcgez_f16)
pub fn vcgez_f16(a: float16x4_t) -> uint16x4_t {
    let b: f16x4 = f16x4::new(0.0, 0.0, 0.0, 0.0);
    { simd_ge(a, transmute(b)) }
}
///Floating-point compare greater than or equal to zero
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcgezq_f16)
pub fn vcgezq_f16(a: float16x8_t) -> uint16x8_t {
    let b: f16x8 = f16x8::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    { simd_ge(a, transmute(b)) }
}
///Floating-point compare greater than
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcgt_f16)
pub fn vcgt_f16(a: float16x4_t, b: float16x4_t) -> uint16x4_t {
    { simd_gt(a, b) }
}
///Floating-point compare greater than
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcgtq_f16)
pub fn vcgtq_f16(a: float16x8_t, b: float16x8_t) -> uint16x8_t {
    { simd_gt(a, b) }
}
///Floating-point compare greater than
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcgt_f32)
pub fn vcgt_f32(a: float32x2_t, b: float32x2_t) -> uint32x2_t {
    { simd_gt(a, b) }
}
///Floating-point compare greater than
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcgtq_f32)
pub fn vcgtq_f32(a: float32x4_t, b: float32x4_t) -> uint32x4_t {
    { simd_gt(a, b) }
}
///Compare signed greater than
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcgt_s8)
pub fn vcgt_s8(a: int8x8_t, b: int8x8_t) -> uint8x8_t {
    { simd_gt(a, b) }
}
///Compare signed greater than
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcgtq_s8)
pub fn vcgtq_s8(a: int8x16_t, b: int8x16_t) -> uint8x16_t {
    { simd_gt(a, b) }
}
///Compare signed greater than
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcgt_s16)
pub fn vcgt_s16(a: int16x4_t, b: int16x4_t) -> uint16x4_t {
    { simd_gt(a, b) }
}
///Compare signed greater than
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcgtq_s16)
pub fn vcgtq_s16(a: int16x8_t, b: int16x8_t) -> uint16x8_t {
    { simd_gt(a, b) }
}
///Compare signed greater than
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcgt_s32)
pub fn vcgt_s32(a: int32x2_t, b: int32x2_t) -> uint32x2_t {
    { simd_gt(a, b) }
}
///Compare signed greater than
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcgtq_s32)
pub fn vcgtq_s32(a: int32x4_t, b: int32x4_t) -> uint32x4_t {
    { simd_gt(a, b) }
}
///Compare unsigned greater than
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcgt_u8)
pub fn vcgt_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
    { simd_gt(a, b) }
}
///Compare unsigned greater than
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcgtq_u8)
pub fn vcgtq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
    { simd_gt(a, b) }
}
///Compare unsigned greater than
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcgt_u16)
pub fn vcgt_u16(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
    { simd_gt(a, b) }
}
///Compare unsigned greater than
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcgtq_u16)
pub fn vcgtq_u16(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
    { simd_gt(a, b) }
}
///Compare unsigned greater than
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcgt_u32)
pub fn vcgt_u32(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
    { simd_gt(a, b) }
}
///Compare unsigned greater than
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcgtq_u32)
pub fn vcgtq_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
    { simd_gt(a, b) }
}
///Floating-point compare greater than zero
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcgtz_f16)
pub fn vcgtz_f16(a: float16x4_t) -> uint16x4_t {
    let b: f16x4 = f16x4::new(0.0, 0.0, 0.0, 0.0);
    { simd_gt(a, transmute(b)) }
}
///Floating-point compare greater than zero
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcgtzq_f16)
pub fn vcgtzq_f16(a: float16x8_t) -> uint16x8_t {
    let b: f16x8 = f16x8::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    { simd_gt(a, transmute(b)) }
}
///Floating-point compare less than or equal
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcle_f16)
pub fn vcle_f16(a: float16x4_t, b: float16x4_t) -> uint16x4_t {
    { simd_le(a, b) }
}
///Floating-point compare less than or equal
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcleq_f16)
pub fn vcleq_f16(a: float16x8_t, b: float16x8_t) -> uint16x8_t {
    { simd_le(a, b) }
}
///Floating-point compare less than or equal
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcle_f32)
pub fn vcle_f32(a: float32x2_t, b: float32x2_t) -> uint32x2_t {
    { simd_le(a, b) }
}
///Floating-point compare less than or equal
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcleq_f32)
pub fn vcleq_f32(a: float32x4_t, b: float32x4_t) -> uint32x4_t {
    { simd_le(a, b) }
}
///Compare signed less than or equal
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcle_s8)
pub fn vcle_s8(a: int8x8_t, b: int8x8_t) -> uint8x8_t {
    { simd_le(a, b) }
}
///Compare signed less than or equal
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcleq_s8)
pub fn vcleq_s8(a: int8x16_t, b: int8x16_t) -> uint8x16_t {
    { simd_le(a, b) }
}
///Compare signed less than or equal
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcle_s16)
pub fn vcle_s16(a: int16x4_t, b: int16x4_t) -> uint16x4_t {
    { simd_le(a, b) }
}
///Compare signed less than or equal
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcleq_s16)
pub fn vcleq_s16(a: int16x8_t, b: int16x8_t) -> uint16x8_t {
    { simd_le(a, b) }
}
///Compare signed less than or equal
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcle_s32)
pub fn vcle_s32(a: int32x2_t, b: int32x2_t) -> uint32x2_t {
    { simd_le(a, b) }
}
///Compare signed less than or equal
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcleq_s32)
pub fn vcleq_s32(a: int32x4_t, b: int32x4_t) -> uint32x4_t {
    { simd_le(a, b) }
}
///Compare unsigned less than or equal
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcle_u8)
pub fn vcle_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
    { simd_le(a, b) }
}
///Compare unsigned less than or equal
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcleq_u8)
pub fn vcleq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
    { simd_le(a, b) }
}
///Compare unsigned less than or equal
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcle_u16)
pub fn vcle_u16(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
    { simd_le(a, b) }
}
///Compare unsigned less than or equal
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcleq_u16)
pub fn vcleq_u16(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
    { simd_le(a, b) }
}
///Compare unsigned less than or equal
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcle_u32)
pub fn vcle_u32(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
    { simd_le(a, b) }
}
///Compare unsigned less than or equal
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcleq_u32)
pub fn vcleq_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
    { simd_le(a, b) }
}
///Floating-point compare less than or equal to zero
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vclez_f16)
pub fn vclez_f16(a: float16x4_t) -> uint16x4_t {
    let b: f16x4 = f16x4::new(0.0, 0.0, 0.0, 0.0);
    { simd_le(a, transmute(b)) }
}
///Floating-point compare less than or equal to zero
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vclezq_f16)
pub fn vclezq_f16(a: float16x8_t) -> uint16x8_t {
    let b: f16x8 = f16x8::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    { simd_le(a, transmute(b)) }
}
///Count leading sign bits
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcls_s8)
pub fn vcls_s8(a: int8x8_t) -> int8x8_t {
    { _vcls_s8(a) }
}
///Count leading sign bits
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vclsq_s8)
pub fn vclsq_s8(a: int8x16_t) -> int8x16_t {
    { _vclsq_s8(a) }
}
///Count leading sign bits
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcls_s16)
pub fn vcls_s16(a: int16x4_t) -> int16x4_t {
    { _vcls_s16(a) }
}
///Count leading sign bits
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vclsq_s16)
pub fn vclsq_s16(a: int16x8_t) -> int16x8_t {
    { _vclsq_s16(a) }
}
///Count leading sign bits
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcls_s32)
pub fn vcls_s32(a: int32x2_t) -> int32x2_t {
    { _vcls_s32(a) }
}
///Count leading sign bits
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vclsq_s32)
pub fn vclsq_s32(a: int32x4_t) -> int32x4_t {
    { _vclsq_s32(a) }
}
///Count leading sign bits
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcls_u8)
pub fn vcls_u8(a: uint8x8_t) -> int8x8_t {
    { vcls_s8(transmute(a)) }
}
///Count leading sign bits
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vclsq_u8)
pub fn vclsq_u8(a: uint8x16_t) -> int8x16_t {
    { vclsq_s8(transmute(a)) }
}
///Count leading sign bits
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcls_u16)
pub fn vcls_u16(a: uint16x4_t) -> int16x4_t {
    { vcls_s16(transmute(a)) }
}
///Count leading sign bits
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vclsq_u16)
pub fn vclsq_u16(a: uint16x8_t) -> int16x8_t {
    { vclsq_s16(transmute(a)) }
}
///Count leading sign bits
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcls_u32)
pub fn vcls_u32(a: uint32x2_t) -> int32x2_t {
    { vcls_s32(transmute(a)) }
}
///Count leading sign bits
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vclsq_u32)
pub fn vclsq_u32(a: uint32x4_t) -> int32x4_t {
    { vclsq_s32(transmute(a)) }
}
///Floating-point compare less than
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vclt_f16)
pub fn vclt_f16(a: float16x4_t, b: float16x4_t) -> uint16x4_t {
    { simd_lt(a, b) }
}
///Floating-point compare less than
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcltq_f16)
pub fn vcltq_f16(a: float16x8_t, b: float16x8_t) -> uint16x8_t {
    { simd_lt(a, b) }
}
///Floating-point compare less than
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vclt_f32)
pub fn vclt_f32(a: float32x2_t, b: float32x2_t) -> uint32x2_t {
    { simd_lt(a, b) }
}
///Floating-point compare less than
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcltq_f32)
pub fn vcltq_f32(a: float32x4_t, b: float32x4_t) -> uint32x4_t {
    { simd_lt(a, b) }
}
///Compare signed less than
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vclt_s8)
pub fn vclt_s8(a: int8x8_t, b: int8x8_t) -> uint8x8_t {
    { simd_lt(a, b) }
}
///Compare signed less than
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcltq_s8)
pub fn vcltq_s8(a: int8x16_t, b: int8x16_t) -> uint8x16_t {
    { simd_lt(a, b) }
}
///Compare signed less than
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vclt_s16)
pub fn vclt_s16(a: int16x4_t, b: int16x4_t) -> uint16x4_t {
    { simd_lt(a, b) }
}
///Compare signed less than
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcltq_s16)
pub fn vcltq_s16(a: int16x8_t, b: int16x8_t) -> uint16x8_t {
    { simd_lt(a, b) }
}
///Compare signed less than
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vclt_s32)
pub fn vclt_s32(a: int32x2_t, b: int32x2_t) -> uint32x2_t {
    { simd_lt(a, b) }
}
///Compare signed less than
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcltq_s32)
pub fn vcltq_s32(a: int32x4_t, b: int32x4_t) -> uint32x4_t {
    { simd_lt(a, b) }
}
///Compare unsigned less than
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vclt_u8)
pub fn vclt_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
    { simd_lt(a, b) }
}
///Compare unsigned less than
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcltq_u8)
pub fn vcltq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
    { simd_lt(a, b) }
}
///Compare unsigned less than
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vclt_u16)
pub fn vclt_u16(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
    { simd_lt(a, b) }
}
///Compare unsigned less than
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcltq_u16)
pub fn vcltq_u16(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
    { simd_lt(a, b) }
}
///Compare unsigned less than
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vclt_u32)
pub fn vclt_u32(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
    { simd_lt(a, b) }
}
///Compare unsigned less than
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcltq_u32)
pub fn vcltq_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
    { simd_lt(a, b) }
}
///Floating-point compare less than
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcltz_f16)
pub fn vcltz_f16(a: float16x4_t) -> uint16x4_t {
    let b: f16x4 = f16x4::new(0.0, 0.0, 0.0, 0.0);
    { simd_lt(a, transmute(b)) }
}
///Floating-point compare less than
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcltzq_f16)
pub fn vcltzq_f16(a: float16x8_t) -> uint16x8_t {
    let b: f16x8 = f16x8::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    { simd_lt(a, transmute(b)) }
}
///Count leading zero bits
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vclz_s8)
pub fn vclz_s8(a: int8x8_t) -> int8x8_t {
    { simd_ctlz(a) }
}
///Count leading zero bits
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vclzq_s8)
pub fn vclzq_s8(a: int8x16_t) -> int8x16_t {
    { simd_ctlz(a) }
}
///Count leading zero bits
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vclz_s16)
pub fn vclz_s16(a: int16x4_t) -> int16x4_t {
    { simd_ctlz(a) }
}
///Count leading zero bits
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vclzq_s16)
pub fn vclzq_s16(a: int16x8_t) -> int16x8_t {
    { simd_ctlz(a) }
}
///Count leading zero bits
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vclz_s32)
pub fn vclz_s32(a: int32x2_t) -> int32x2_t {
    { simd_ctlz(a) }
}
///Count leading zero bits
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vclzq_s32)
pub fn vclzq_s32(a: int32x4_t) -> int32x4_t {
    { simd_ctlz(a) }
}
///Count leading zero bits
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vclz_u16)
pub fn vclz_u16(a: uint16x4_t) -> uint16x4_t {
    { transmute(vclz_s16(transmute(a))) }
}
///Count leading zero bits
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vclz_u16)
pub fn vclz_u16(a: uint16x4_t) -> uint16x4_t {
    let a: uint16x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    {
        let ret_val: uint16x4_t = transmute(vclz_s16(transmute(a)));
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Count leading zero bits
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vclzq_u16)
pub fn vclzq_u16(a: uint16x8_t) -> uint16x8_t {
    { transmute(vclzq_s16(transmute(a))) }
}
///Count leading zero bits
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vclzq_u16)
pub fn vclzq_u16(a: uint16x8_t) -> uint16x8_t {
    let a: uint16x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: uint16x8_t = transmute(vclzq_s16(transmute(a)));
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Count leading zero bits
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vclz_u32)
pub fn vclz_u32(a: uint32x2_t) -> uint32x2_t {
    { transmute(vclz_s32(transmute(a))) }
}
///Count leading zero bits
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vclz_u32)
pub fn vclz_u32(a: uint32x2_t) -> uint32x2_t {
    let a: uint32x2_t = unsafe { simd_shuffle(a, a, [1, 0]) };
    {
        let ret_val: uint32x2_t = transmute(vclz_s32(transmute(a)));
        simd_shuffle(ret_val, ret_val, [1, 0])
    }
}
///Count leading zero bits
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vclzq_u32)
pub fn vclzq_u32(a: uint32x4_t) -> uint32x4_t {
    { transmute(vclzq_s32(transmute(a))) }
}
///Count leading zero bits
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vclzq_u32)
pub fn vclzq_u32(a: uint32x4_t) -> uint32x4_t {
    let a: uint32x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    {
        let ret_val: uint32x4_t = transmute(vclzq_s32(transmute(a)));
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Count leading zero bits
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vclz_u8)
pub fn vclz_u8(a: uint8x8_t) -> uint8x8_t {
    { transmute(vclz_s8(transmute(a))) }
}
///Count leading zero bits
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vclz_u8)
pub fn vclz_u8(a: uint8x8_t) -> uint8x8_t {
    let a: uint8x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: uint8x8_t = transmute(vclz_s8(transmute(a)));
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Count leading zero bits
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vclzq_u8)
pub fn vclzq_u8(a: uint8x16_t) -> uint8x16_t {
    { transmute(vclzq_s8(transmute(a))) }
}
///Count leading zero bits
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vclzq_u8)
pub fn vclzq_u8(a: uint8x16_t) -> uint8x16_t {
    let a: uint8x16_t = unsafe {
        simd_shuffle(a, a, [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0])
    };
    {
        let ret_val: uint8x16_t = transmute(vclzq_s8(transmute(a)));
        simd_shuffle(
            ret_val, ret_val, [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
        )
    }
}
///Population count per byte.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcnt_s8)
pub fn vcnt_s8(a: int8x8_t) -> int8x8_t {
    { simd_ctpop(a) }
}
///Population count per byte.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcntq_s8)
pub fn vcntq_s8(a: int8x16_t) -> int8x16_t {
    { simd_ctpop(a) }
}
///Population count per byte.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcnt_u8)
pub fn vcnt_u8(a: uint8x8_t) -> uint8x8_t {
    { transmute(vcnt_s8(transmute(a))) }
}
///Population count per byte.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcnt_u8)
pub fn vcnt_u8(a: uint8x8_t) -> uint8x8_t {
    let a: uint8x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: uint8x8_t = transmute(vcnt_s8(transmute(a)));
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Population count per byte.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcntq_u8)
pub fn vcntq_u8(a: uint8x16_t) -> uint8x16_t {
    { transmute(vcntq_s8(transmute(a))) }
}
///Population count per byte.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcntq_u8)
pub fn vcntq_u8(a: uint8x16_t) -> uint8x16_t {
    let a: uint8x16_t = unsafe {
        simd_shuffle(a, a, [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0])
    };
    {
        let ret_val: uint8x16_t = transmute(vcntq_s8(transmute(a)));
        simd_shuffle(
            ret_val, ret_val, [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
        )
    }
}
///Population count per byte.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcnt_p8)
pub fn vcnt_p8(a: poly8x8_t) -> poly8x8_t {
    { transmute(vcnt_s8(transmute(a))) }
}
///Population count per byte.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcnt_p8)
pub fn vcnt_p8(a: poly8x8_t) -> poly8x8_t {
    let a: poly8x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: poly8x8_t = transmute(vcnt_s8(transmute(a)));
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Population count per byte.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcntq_p8)
pub fn vcntq_p8(a: poly8x16_t) -> poly8x16_t {
    { transmute(vcntq_s8(transmute(a))) }
}
///Population count per byte.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcntq_p8)
pub fn vcntq_p8(a: poly8x16_t) -> poly8x16_t {
    let a: poly8x16_t = unsafe {
        simd_shuffle(a, a, [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0])
    };
    {
        let ret_val: poly8x16_t = transmute(vcntq_s8(transmute(a)));
        simd_shuffle(
            ret_val, ret_val, [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
        )
    }
}
///Join two smaller vectors into a single larger vector
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcombine_f16)
pub fn vcombine_f16(a: float16x4_t, b: float16x4_t) -> float16x8_t {
    { simd_shuffle(a, b, [0, 1, 2, 3, 4, 5, 6, 7]) }
}
///Join two smaller vectors into a single larger vector
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcombine_f32)
pub fn vcombine_f32(a: float32x2_t, b: float32x2_t) -> float32x4_t {
    { simd_shuffle(a, b, [0, 1, 2, 3]) }
}
///Join two smaller vectors into a single larger vector
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcombine_s8)
pub fn vcombine_s8(a: int8x8_t, b: int8x8_t) -> int8x16_t {
    { simd_shuffle(a, b, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]) }
}
///Join two smaller vectors into a single larger vector
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcombine_s16)
pub fn vcombine_s16(a: int16x4_t, b: int16x4_t) -> int16x8_t {
    { simd_shuffle(a, b, [0, 1, 2, 3, 4, 5, 6, 7]) }
}
///Join two smaller vectors into a single larger vector
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcombine_s32)
pub fn vcombine_s32(a: int32x2_t, b: int32x2_t) -> int32x4_t {
    { simd_shuffle(a, b, [0, 1, 2, 3]) }
}
///Join two smaller vectors into a single larger vector
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcombine_s64)
pub fn vcombine_s64(a: int64x1_t, b: int64x1_t) -> int64x2_t {
    { simd_shuffle(a, b, [0, 1]) }
}
///Join two smaller vectors into a single larger vector
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcombine_u8)
pub fn vcombine_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x16_t {
    { simd_shuffle(a, b, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]) }
}
///Join two smaller vectors into a single larger vector
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcombine_u16)
pub fn vcombine_u16(a: uint16x4_t, b: uint16x4_t) -> uint16x8_t {
    { simd_shuffle(a, b, [0, 1, 2, 3, 4, 5, 6, 7]) }
}
///Join two smaller vectors into a single larger vector
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcombine_u32)
pub fn vcombine_u32(a: uint32x2_t, b: uint32x2_t) -> uint32x4_t {
    { simd_shuffle(a, b, [0, 1, 2, 3]) }
}
///Join two smaller vectors into a single larger vector
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcombine_u64)
pub fn vcombine_u64(a: uint64x1_t, b: uint64x1_t) -> uint64x2_t {
    { simd_shuffle(a, b, [0, 1]) }
}
///Join two smaller vectors into a single larger vector
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcombine_p8)
pub fn vcombine_p8(a: poly8x8_t, b: poly8x8_t) -> poly8x16_t {
    { simd_shuffle(a, b, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]) }
}
///Join two smaller vectors into a single larger vector
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcombine_p16)
pub fn vcombine_p16(a: poly16x4_t, b: poly16x4_t) -> poly16x8_t {
    { simd_shuffle(a, b, [0, 1, 2, 3, 4, 5, 6, 7]) }
}
///Join two smaller vectors into a single larger vector
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcombine_p64)
pub fn vcombine_p64(a: poly64x1_t, b: poly64x1_t) -> poly64x2_t {
    { simd_shuffle(a, b, [0, 1]) }
}
///Insert vector element from another vector element
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcreate_f16)
pub fn vcreate_f16(a: u64) -> float16x4_t {
    { transmute(a) }
}
///Insert vector element from another vector element
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcreate_f16)
pub fn vcreate_f16(a: u64) -> float16x4_t {
    {
        let ret_val: float16x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Insert vector element from another vector element
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcreate_f32)
pub fn vcreate_f32(a: u64) -> float32x2_t {
    { transmute(a) }
}
///Insert vector element from another vector element
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcreate_f32)
pub fn vcreate_f32(a: u64) -> float32x2_t {
    {
        let ret_val: float32x2_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [1, 0])
    }
}
///Insert vector element from another vector element
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcreate_s8)
pub fn vcreate_s8(a: u64) -> int8x8_t {
    { transmute(a) }
}
///Insert vector element from another vector element
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcreate_s8)
pub fn vcreate_s8(a: u64) -> int8x8_t {
    {
        let ret_val: int8x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Insert vector element from another vector element
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcreate_s16)
pub fn vcreate_s16(a: u64) -> int16x4_t {
    { transmute(a) }
}
///Insert vector element from another vector element
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcreate_s16)
pub fn vcreate_s16(a: u64) -> int16x4_t {
    {
        let ret_val: int16x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Insert vector element from another vector element
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcreate_s32)
pub fn vcreate_s32(a: u64) -> int32x2_t {
    { transmute(a) }
}
///Insert vector element from another vector element
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcreate_s32)
pub fn vcreate_s32(a: u64) -> int32x2_t {
    {
        let ret_val: int32x2_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [1, 0])
    }
}
///Insert vector element from another vector element
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcreate_s64)
pub fn vcreate_s64(a: u64) -> int64x1_t {
    { transmute(a) }
}
///Insert vector element from another vector element
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcreate_u8)
pub fn vcreate_u8(a: u64) -> uint8x8_t {
    { transmute(a) }
}
///Insert vector element from another vector element
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcreate_u8)
pub fn vcreate_u8(a: u64) -> uint8x8_t {
    {
        let ret_val: uint8x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Insert vector element from another vector element
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcreate_u16)
pub fn vcreate_u16(a: u64) -> uint16x4_t {
    { transmute(a) }
}
///Insert vector element from another vector element
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcreate_u16)
pub fn vcreate_u16(a: u64) -> uint16x4_t {
    {
        let ret_val: uint16x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Insert vector element from another vector element
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcreate_u32)
pub fn vcreate_u32(a: u64) -> uint32x2_t {
    { transmute(a) }
}
///Insert vector element from another vector element
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcreate_u32)
pub fn vcreate_u32(a: u64) -> uint32x2_t {
    {
        let ret_val: uint32x2_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [1, 0])
    }
}
///Insert vector element from another vector element
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcreate_u64)
pub fn vcreate_u64(a: u64) -> uint64x1_t {
    { transmute(a) }
}
///Insert vector element from another vector element
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcreate_p8)
pub fn vcreate_p8(a: u64) -> poly8x8_t {
    { transmute(a) }
}
///Insert vector element from another vector element
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcreate_p8)
pub fn vcreate_p8(a: u64) -> poly8x8_t {
    {
        let ret_val: poly8x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Insert vector element from another vector element
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcreate_p16)
pub fn vcreate_p16(a: u64) -> poly16x4_t {
    { transmute(a) }
}
///Insert vector element from another vector element
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcreate_p16)
pub fn vcreate_p16(a: u64) -> poly16x4_t {
    {
        let ret_val: poly16x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Insert vector element from another vector element
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcreate_p64)
pub fn vcreate_p64(a: u64) -> poly64x1_t {
    { transmute(a) }
}
///Floating-point convert to lower precision narrow
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcvt_f16_f32)
pub fn vcvt_f16_f32(a: float32x4_t) -> float16x4_t {
    { simd_cast(a) }
}
///Fixed-point convert to floating-point
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcvt_f16_s16)
pub fn vcvt_f16_s16(a: int16x4_t) -> float16x4_t {
    { simd_cast(a) }
}
///Fixed-point convert to floating-point
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcvtq_f16_s16)
pub fn vcvtq_f16_s16(a: int16x8_t) -> float16x8_t {
    { simd_cast(a) }
}
///Fixed-point convert to floating-point
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcvt_f16_u16)
pub fn vcvt_f16_u16(a: uint16x4_t) -> float16x4_t {
    { simd_cast(a) }
}
///Fixed-point convert to floating-point
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcvtq_f16_u16)
pub fn vcvtq_f16_u16(a: uint16x8_t) -> float16x8_t {
    { simd_cast(a) }
}
///Floating-point convert to higher precision long
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcvt_f32_f16)
pub fn vcvt_f32_f16(a: float16x4_t) -> float32x4_t {
    { simd_cast(a) }
}
///Fixed-point convert to floating-point
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcvt_f32_s32)
pub fn vcvt_f32_s32(a: int32x2_t) -> float32x2_t {
    { simd_cast(a) }
}
///Fixed-point convert to floating-point
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcvtq_f32_s32)
pub fn vcvtq_f32_s32(a: int32x4_t) -> float32x4_t {
    { simd_cast(a) }
}
///Fixed-point convert to floating-point
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcvt_f32_u32)
pub fn vcvt_f32_u32(a: uint32x2_t) -> float32x2_t {
    { simd_cast(a) }
}
///Fixed-point convert to floating-point
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcvtq_f32_u32)
pub fn vcvtq_f32_u32(a: uint32x4_t) -> float32x4_t {
    { simd_cast(a) }
}
///Fixed-point convert to floating-point
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcvt_n_f16_s16)
pub fn vcvt_n_f16_s16<const N: i32>(a: int16x4_t) -> float16x4_t {
    static_assert!(N >= 1 && N <= 16);
    { _vcvt_n_f16_s16(a, N) }
}
///Fixed-point convert to floating-point
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcvtq_n_f16_s16)
pub fn vcvtq_n_f16_s16<const N: i32>(a: int16x8_t) -> float16x8_t {
    static_assert!(N >= 1 && N <= 16);
    { _vcvtq_n_f16_s16(a, N) }
}
///Fixed-point convert to floating-point
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcvt_n_f16_u16)
pub fn vcvt_n_f16_u16<const N: i32>(a: uint16x4_t) -> float16x4_t {
    static_assert!(N >= 1 && N <= 16);
    { _vcvt_n_f16_u16(a, N) }
}
///Fixed-point convert to floating-point
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcvtq_n_f16_u16)
pub fn vcvtq_n_f16_u16<const N: i32>(a: uint16x8_t) -> float16x8_t {
    static_assert!(N >= 1 && N <= 16);
    { _vcvtq_n_f16_u16(a, N) }
}
///Fixed-point convert to floating-point
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcvt_n_f32_s32)
pub fn vcvt_n_f32_s32<const N: i32>(a: int32x2_t) -> float32x2_t {
    static_assert!(N >= 1 && N <= 32);
    { _vcvt_n_f32_s32(a, N) }
}
///Fixed-point convert to floating-point
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcvtq_n_f32_s32)
pub fn vcvtq_n_f32_s32<const N: i32>(a: int32x4_t) -> float32x4_t {
    static_assert!(N >= 1 && N <= 32);
    { _vcvtq_n_f32_s32(a, N) }
}
///Fixed-point convert to floating-point
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcvt_n_f32_s32)
pub fn vcvt_n_f32_s32<const N: i32>(a: int32x2_t) -> float32x2_t {
    static_assert!(N >= 1 && N <= 32);
    { _vcvt_n_f32_s32(a, N) }
}
///Fixed-point convert to floating-point
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcvtq_n_f32_s32)
pub fn vcvtq_n_f32_s32<const N: i32>(a: int32x4_t) -> float32x4_t {
    static_assert!(N >= 1 && N <= 32);
    { _vcvtq_n_f32_s32(a, N) }
}
///Fixed-point convert to floating-point
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcvt_n_f32_u32)
pub fn vcvt_n_f32_u32<const N: i32>(a: uint32x2_t) -> float32x2_t {
    static_assert!(N >= 1 && N <= 32);
    { _vcvt_n_f32_u32(a, N) }
}
///Fixed-point convert to floating-point
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcvtq_n_f32_u32)
pub fn vcvtq_n_f32_u32<const N: i32>(a: uint32x4_t) -> float32x4_t {
    static_assert!(N >= 1 && N <= 32);
    { _vcvtq_n_f32_u32(a, N) }
}
///Fixed-point convert to floating-point
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcvt_n_f32_u32)
pub fn vcvt_n_f32_u32<const N: i32>(a: uint32x2_t) -> float32x2_t {
    static_assert!(N >= 1 && N <= 32);
    { _vcvt_n_f32_u32(a, N) }
}
///Fixed-point convert to floating-point
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcvtq_n_f32_u32)
pub fn vcvtq_n_f32_u32<const N: i32>(a: uint32x4_t) -> float32x4_t {
    static_assert!(N >= 1 && N <= 32);
    { _vcvtq_n_f32_u32(a, N) }
}
///Floating-point convert to signed fixed-point
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcvt_n_s16_f16)
pub fn vcvt_n_s16_f16<const N: i32>(a: float16x4_t) -> int16x4_t {
    static_assert!(N >= 1 && N <= 16);
    { _vcvt_n_s16_f16(a, N) }
}
///Floating-point convert to signed fixed-point
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcvtq_n_s16_f16)
pub fn vcvtq_n_s16_f16<const N: i32>(a: float16x8_t) -> int16x8_t {
    static_assert!(N >= 1 && N <= 16);
    { _vcvtq_n_s16_f16(a, N) }
}
///Floating-point convert to fixed-point, rounding toward zero
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcvt_n_s32_f32)
pub fn vcvt_n_s32_f32<const N: i32>(a: float32x2_t) -> int32x2_t {
    static_assert!(N >= 1 && N <= 32);
    { _vcvt_n_s32_f32(a, N) }
}
///Floating-point convert to fixed-point, rounding toward zero
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcvtq_n_s32_f32)
pub fn vcvtq_n_s32_f32<const N: i32>(a: float32x4_t) -> int32x4_t {
    static_assert!(N >= 1 && N <= 32);
    { _vcvtq_n_s32_f32(a, N) }
}
///Floating-point convert to fixed-point, rounding toward zero
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcvt_n_s32_f32)
pub fn vcvt_n_s32_f32<const N: i32>(a: float32x2_t) -> int32x2_t {
    static_assert!(N >= 1 && N <= 32);
    { _vcvt_n_s32_f32(a, N) }
}
///Floating-point convert to fixed-point, rounding toward zero
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcvtq_n_s32_f32)
pub fn vcvtq_n_s32_f32<const N: i32>(a: float32x4_t) -> int32x4_t {
    static_assert!(N >= 1 && N <= 32);
    { _vcvtq_n_s32_f32(a, N) }
}
///Fixed-point convert to unsigned fixed-point, rounding toward zero
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcvt_n_u16_f16)
pub fn vcvt_n_u16_f16<const N: i32>(a: float16x4_t) -> uint16x4_t {
    static_assert!(N >= 1 && N <= 16);
    { _vcvt_n_u16_f16(a, N) }
}
///Fixed-point convert to unsigned fixed-point, rounding toward zero
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcvtq_n_u16_f16)
pub fn vcvtq_n_u16_f16<const N: i32>(a: float16x8_t) -> uint16x8_t {
    static_assert!(N >= 1 && N <= 16);
    { _vcvtq_n_u16_f16(a, N) }
}
///Floating-point convert to fixed-point, rounding toward zero
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcvt_n_u32_f32)
pub fn vcvt_n_u32_f32<const N: i32>(a: float32x2_t) -> uint32x2_t {
    static_assert!(N >= 1 && N <= 32);
    { _vcvt_n_u32_f32(a, N) }
}
///Floating-point convert to fixed-point, rounding toward zero
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcvtq_n_u32_f32)
pub fn vcvtq_n_u32_f32<const N: i32>(a: float32x4_t) -> uint32x4_t {
    static_assert!(N >= 1 && N <= 32);
    { _vcvtq_n_u32_f32(a, N) }
}
///Floating-point convert to fixed-point, rounding toward zero
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcvt_n_u32_f32)
pub fn vcvt_n_u32_f32<const N: i32>(a: float32x2_t) -> uint32x2_t {
    static_assert!(N >= 1 && N <= 32);
    { _vcvt_n_u32_f32(a, N) }
}
///Floating-point convert to fixed-point, rounding toward zero
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcvtq_n_u32_f32)
pub fn vcvtq_n_u32_f32<const N: i32>(a: float32x4_t) -> uint32x4_t {
    static_assert!(N >= 1 && N <= 32);
    { _vcvtq_n_u32_f32(a, N) }
}
///Floating-point convert to signed fixed-point, rounding toward zero
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcvt_s16_f16)
pub fn vcvt_s16_f16(a: float16x4_t) -> int16x4_t {
    { simd_cast(a) }
}
///Floating-point convert to signed fixed-point, rounding toward zero
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcvtq_s16_f16)
pub fn vcvtq_s16_f16(a: float16x8_t) -> int16x8_t {
    { simd_cast(a) }
}
///Floating-point convert to signed fixed-point, rounding toward zero
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcvt_s32_f32)
pub fn vcvt_s32_f32(a: float32x2_t) -> int32x2_t {
    { _vcvt_s32_f32(a) }
}
///Floating-point convert to signed fixed-point, rounding toward zero
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcvtq_s32_f32)
pub fn vcvtq_s32_f32(a: float32x4_t) -> int32x4_t {
    { _vcvtq_s32_f32(a) }
}
///Floating-point convert to unsigned fixed-point, rounding toward zero
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcvt_u16_f16)
pub fn vcvt_u16_f16(a: float16x4_t) -> uint16x4_t {
    { simd_cast(a) }
}
///Floating-point convert to unsigned fixed-point, rounding toward zero
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcvtq_u16_f16)
pub fn vcvtq_u16_f16(a: float16x8_t) -> uint16x8_t {
    { simd_cast(a) }
}
///Floating-point convert to unsigned fixed-point, rounding toward zero
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcvt_u32_f32)
pub fn vcvt_u32_f32(a: float32x2_t) -> uint32x2_t {
    { _vcvt_u32_f32(a) }
}
///Floating-point convert to unsigned fixed-point, rounding toward zero
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vcvtq_u32_f32)
pub fn vcvtq_u32_f32(a: float32x4_t) -> uint32x4_t {
    { _vcvtq_u32_f32(a) }
}
///Dot product arithmetic (indexed)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vdot_lane_s32)
pub fn vdot_lane_s32<const LANE: i32>(
    a: int32x2_t,
    b: int8x8_t,
    c: int8x8_t,
) -> int32x2_t {
    static_assert_uimm_bits!(LANE, 1);
    {
        let c: int32x2_t = transmute(c);
        let c: int32x2_t = simd_shuffle(c, c, [LANE as u32, LANE as u32]);
        vdot_s32(a, b, transmute(c))
    }
}
///Dot product arithmetic (indexed)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vdotq_lane_s32)
pub fn vdotq_lane_s32<const LANE: i32>(
    a: int32x4_t,
    b: int8x16_t,
    c: int8x8_t,
) -> int32x4_t {
    static_assert_uimm_bits!(LANE, 1);
    {
        let c: int32x2_t = transmute(c);
        let c: int32x4_t = simd_shuffle(
            c, c, [LANE as u32, LANE as u32, LANE as u32, LANE as u32]
        );
        vdotq_s32(a, b, transmute(c))
    }
}
///Dot product arithmetic (indexed)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vdot_lane_u32)
pub fn vdot_lane_u32<const LANE: i32>(
    a: uint32x2_t,
    b: uint8x8_t,
    c: uint8x8_t,
) -> uint32x2_t {
    static_assert_uimm_bits!(LANE, 1);
    {
        let c: uint32x2_t = transmute(c);
        let c: uint32x2_t = simd_shuffle(c, c, [LANE as u32, LANE as u32]);
        vdot_u32(a, b, transmute(c))
    }
}
///Dot product arithmetic (indexed)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vdotq_lane_u32)
pub fn vdotq_lane_u32<const LANE: i32>(
    a: uint32x4_t,
    b: uint8x16_t,
    c: uint8x8_t,
) -> uint32x4_t {
    static_assert_uimm_bits!(LANE, 1);
    {
        let c: uint32x2_t = transmute(c);
        let c: uint32x4_t = simd_shuffle(
            c, c, [LANE as u32, LANE as u32, LANE as u32, LANE as u32]
        );
        vdotq_u32(a, b, transmute(c))
    }
}
///Dot product arithmetic (vector)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vdot_s32)
pub fn vdot_s32(a: int32x2_t, b: int8x8_t, c: int8x8_t) -> int32x2_t {
    { _vdot_s32(a, b, c) }
}
///Dot product arithmetic (vector)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vdotq_s32)
pub fn vdotq_s32(a: int32x4_t, b: int8x16_t, c: int8x16_t) -> int32x4_t {
    { _vdotq_s32(a, b, c) }
}
///Dot product arithmetic (vector)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vdot_u32)
pub fn vdot_u32(a: uint32x2_t, b: uint8x8_t, c: uint8x8_t) -> uint32x2_t {
    { _vdot_u32(a, b, c) }
}
///Dot product arithmetic (vector)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vdotq_u32)
pub fn vdotq_u32(a: uint32x4_t, b: uint8x16_t, c: uint8x16_t) -> uint32x4_t {
    { _vdotq_u32(a, b, c) }
}
///Set all vector lanes to the same value
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vdup_lane_f16)
pub fn vdup_lane_f16<const N: i32>(a: float16x4_t) -> float16x4_t {
    static_assert_uimm_bits!(N, 2);
    { simd_shuffle(a, a, [N as u32, N as u32, N as u32, N as u32]) }
}
///Set all vector lanes to the same value
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vdupq_lane_f16)
pub fn vdupq_lane_f16<const N: i32>(a: float16x4_t) -> float16x8_t {
    static_assert_uimm_bits!(N, 2);
    {
        simd_shuffle(
            a, a, [N as u32, N as u32, N as u32, N as u32, N as u32, N as u32, N as u32,
            N as u32]
        )
    }
}
///Set all vector lanes to the same value
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vdup_lane_f32)
pub fn vdup_lane_f32<const N: i32>(a: float32x2_t) -> float32x2_t {
    static_assert_uimm_bits!(N, 1);
    { simd_shuffle(a, a, [N as u32, N as u32]) }
}
///Set all vector lanes to the same value
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vdup_lane_s32)
pub fn vdup_lane_s32<const N: i32>(a: int32x2_t) -> int32x2_t {
    static_assert_uimm_bits!(N, 1);
    { simd_shuffle(a, a, [N as u32, N as u32]) }
}
///Set all vector lanes to the same value
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vdup_lane_u32)
pub fn vdup_lane_u32<const N: i32>(a: uint32x2_t) -> uint32x2_t {
    static_assert_uimm_bits!(N, 1);
    { simd_shuffle(a, a, [N as u32, N as u32]) }
}
///Set all vector lanes to the same value
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vdupq_lane_f32)
pub fn vdupq_lane_f32<const N: i32>(a: float32x2_t) -> float32x4_t {
    static_assert_uimm_bits!(N, 1);
    { simd_shuffle(a, a, [N as u32, N as u32, N as u32, N as u32]) }
}
///Set all vector lanes to the same value
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vdupq_lane_s32)
pub fn vdupq_lane_s32<const N: i32>(a: int32x2_t) -> int32x4_t {
    static_assert_uimm_bits!(N, 1);
    { simd_shuffle(a, a, [N as u32, N as u32, N as u32, N as u32]) }
}
///Set all vector lanes to the same value
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vdupq_lane_u32)
pub fn vdupq_lane_u32<const N: i32>(a: uint32x2_t) -> uint32x4_t {
    static_assert_uimm_bits!(N, 1);
    { simd_shuffle(a, a, [N as u32, N as u32, N as u32, N as u32]) }
}
///Set all vector lanes to the same value
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vdup_lane_p16)
pub fn vdup_lane_p16<const N: i32>(a: poly16x4_t) -> poly16x4_t {
    static_assert_uimm_bits!(N, 2);
    { simd_shuffle(a, a, [N as u32, N as u32, N as u32, N as u32]) }
}
///Set all vector lanes to the same value
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vdup_lane_s16)
pub fn vdup_lane_s16<const N: i32>(a: int16x4_t) -> int16x4_t {
    static_assert_uimm_bits!(N, 2);
    { simd_shuffle(a, a, [N as u32, N as u32, N as u32, N as u32]) }
}
///Set all vector lanes to the same value
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vdup_lane_u16)
pub fn vdup_lane_u16<const N: i32>(a: uint16x4_t) -> uint16x4_t {
    static_assert_uimm_bits!(N, 2);
    { simd_shuffle(a, a, [N as u32, N as u32, N as u32, N as u32]) }
}
///Set all vector lanes to the same value
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vdupq_lane_p16)
pub fn vdupq_lane_p16<const N: i32>(a: poly16x4_t) -> poly16x8_t {
    static_assert_uimm_bits!(N, 2);
    {
        simd_shuffle(
            a, a, [N as u32, N as u32, N as u32, N as u32, N as u32, N as u32, N as u32,
            N as u32]
        )
    }
}
///Set all vector lanes to the same value
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vdupq_lane_s16)
pub fn vdupq_lane_s16<const N: i32>(a: int16x4_t) -> int16x8_t {
    static_assert_uimm_bits!(N, 2);
    {
        simd_shuffle(
            a, a, [N as u32, N as u32, N as u32, N as u32, N as u32, N as u32, N as u32,
            N as u32]
        )
    }
}
///Set all vector lanes to the same value
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vdupq_lane_u16)
pub fn vdupq_lane_u16<const N: i32>(a: uint16x4_t) -> uint16x8_t {
    static_assert_uimm_bits!(N, 2);
    {
        simd_shuffle(
            a, a, [N as u32, N as u32, N as u32, N as u32, N as u32, N as u32, N as u32,
            N as u32]
        )
    }
}
///Set all vector lanes to the same value
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vdup_lane_p8)
pub fn vdup_lane_p8<const N: i32>(a: poly8x8_t) -> poly8x8_t {
    static_assert_uimm_bits!(N, 3);
    {
        simd_shuffle(
            a, a, [N as u32, N as u32, N as u32, N as u32, N as u32, N as u32, N as u32,
            N as u32]
        )
    }
}
///Set all vector lanes to the same value
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vdup_lane_s8)
pub fn vdup_lane_s8<const N: i32>(a: int8x8_t) -> int8x8_t {
    static_assert_uimm_bits!(N, 3);
    {
        simd_shuffle(
            a, a, [N as u32, N as u32, N as u32, N as u32, N as u32, N as u32, N as u32,
            N as u32]
        )
    }
}
///Set all vector lanes to the same value
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vdup_lane_u8)
pub fn vdup_lane_u8<const N: i32>(a: uint8x8_t) -> uint8x8_t {
    static_assert_uimm_bits!(N, 3);
    {
        simd_shuffle(
            a, a, [N as u32, N as u32, N as u32, N as u32, N as u32, N as u32, N as u32,
            N as u32]
        )
    }
}
///Set all vector lanes to the same value
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vdupq_lane_p8)
pub fn vdupq_lane_p8<const N: i32>(a: poly8x8_t) -> poly8x16_t {
    static_assert_uimm_bits!(N, 3);
    {
        simd_shuffle(
            a, a, [N as u32, N as u32, N as u32, N as u32, N as u32, N as u32, N as u32,
            N as u32, N as u32, N as u32, N as u32, N as u32, N as u32, N as u32, N as
            u32, N as u32]
        )
    }
}
///Set all vector lanes to the same value
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vdupq_lane_s8)
pub fn vdupq_lane_s8<const N: i32>(a: int8x8_t) -> int8x16_t {
    static_assert_uimm_bits!(N, 3);
    {
        simd_shuffle(
            a, a, [N as u32, N as u32, N as u32, N as u32, N as u32, N as u32, N as u32,
            N as u32, N as u32, N as u32, N as u32, N as u32, N as u32, N as u32, N as
            u32, N as u32]
        )
    }
}
///Set all vector lanes to the same value
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vdupq_lane_u8)
pub fn vdupq_lane_u8<const N: i32>(a: uint8x8_t) -> uint8x16_t {
    static_assert_uimm_bits!(N, 3);
    {
        simd_shuffle(
            a, a, [N as u32, N as u32, N as u32, N as u32, N as u32, N as u32, N as u32,
            N as u32, N as u32, N as u32, N as u32, N as u32, N as u32, N as u32, N as
            u32, N as u32]
        )
    }
}
///Set all vector lanes to the same value
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vdup_lane_s64)
pub fn vdup_lane_s64<const N: i32>(a: int64x1_t) -> int64x1_t {
    static_assert!(N == 0);
    a
}
///Set all vector lanes to the same value
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vdup_lane_u64)
pub fn vdup_lane_u64<const N: i32>(a: uint64x1_t) -> uint64x1_t {
    static_assert!(N == 0);
    a
}
///Set all vector lanes to the same value
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vdup_laneq_f16)
pub fn vdup_laneq_f16<const N: i32>(a: float16x8_t) -> float16x4_t {
    static_assert_uimm_bits!(N, 3);
    { simd_shuffle(a, a, [N as u32, N as u32, N as u32, N as u32]) }
}
///Set all vector lanes to the same value
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vdupq_laneq_f16)
pub fn vdupq_laneq_f16<const N: i32>(a: float16x8_t) -> float16x8_t {
    static_assert_uimm_bits!(N, 3);
    {
        simd_shuffle(
            a, a, [N as u32, N as u32, N as u32, N as u32, N as u32, N as u32, N as u32,
            N as u32]
        )
    }
}
///Set all vector lanes to the same value
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vdup_laneq_f32)
pub fn vdup_laneq_f32<const N: i32>(a: float32x4_t) -> float32x2_t {
    static_assert_uimm_bits!(N, 2);
    { simd_shuffle(a, a, [N as u32, N as u32]) }
}
///Set all vector lanes to the same value
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vdup_laneq_s32)
pub fn vdup_laneq_s32<const N: i32>(a: int32x4_t) -> int32x2_t {
    static_assert_uimm_bits!(N, 2);
    { simd_shuffle(a, a, [N as u32, N as u32]) }
}
///Set all vector lanes to the same value
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vdup_laneq_u32)
pub fn vdup_laneq_u32<const N: i32>(a: uint32x4_t) -> uint32x2_t {
    static_assert_uimm_bits!(N, 2);
    { simd_shuffle(a, a, [N as u32, N as u32]) }
}
///Set all vector lanes to the same value
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vdupq_laneq_f32)
pub fn vdupq_laneq_f32<const N: i32>(a: float32x4_t) -> float32x4_t {
    static_assert_uimm_bits!(N, 2);
    { simd_shuffle(a, a, [N as u32, N as u32, N as u32, N as u32]) }
}
///Set all vector lanes to the same value
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vdupq_laneq_s32)
pub fn vdupq_laneq_s32<const N: i32>(a: int32x4_t) -> int32x4_t {
    static_assert_uimm_bits!(N, 2);
    { simd_shuffle(a, a, [N as u32, N as u32, N as u32, N as u32]) }
}
///Set all vector lanes to the same value
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vdupq_laneq_u32)
pub fn vdupq_laneq_u32<const N: i32>(a: uint32x4_t) -> uint32x4_t {
    static_assert_uimm_bits!(N, 2);
    { simd_shuffle(a, a, [N as u32, N as u32, N as u32, N as u32]) }
}
///Set all vector lanes to the same value
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vdup_laneq_p16)
pub fn vdup_laneq_p16<const N: i32>(a: poly16x8_t) -> poly16x4_t {
    static_assert_uimm_bits!(N, 3);
    { simd_shuffle(a, a, [N as u32, N as u32, N as u32, N as u32]) }
}
///Set all vector lanes to the same value
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vdup_laneq_s16)
pub fn vdup_laneq_s16<const N: i32>(a: int16x8_t) -> int16x4_t {
    static_assert_uimm_bits!(N, 3);
    { simd_shuffle(a, a, [N as u32, N as u32, N as u32, N as u32]) }
}
///Set all vector lanes to the same value
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vdup_laneq_u16)
pub fn vdup_laneq_u16<const N: i32>(a: uint16x8_t) -> uint16x4_t {
    static_assert_uimm_bits!(N, 3);
    { simd_shuffle(a, a, [N as u32, N as u32, N as u32, N as u32]) }
}
///Set all vector lanes to the same value
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vdupq_laneq_p16)
pub fn vdupq_laneq_p16<const N: i32>(a: poly16x8_t) -> poly16x8_t {
    static_assert_uimm_bits!(N, 3);
    {
        simd_shuffle(
            a, a, [N as u32, N as u32, N as u32, N as u32, N as u32, N as u32, N as u32,
            N as u32]
        )
    }
}
///Set all vector lanes to the same value
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vdupq_laneq_s16)
pub fn vdupq_laneq_s16<const N: i32>(a: int16x8_t) -> int16x8_t {
    static_assert_uimm_bits!(N, 3);
    {
        simd_shuffle(
            a, a, [N as u32, N as u32, N as u32, N as u32, N as u32, N as u32, N as u32,
            N as u32]
        )
    }
}
///Set all vector lanes to the same value
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vdupq_laneq_u16)
pub fn vdupq_laneq_u16<const N: i32>(a: uint16x8_t) -> uint16x8_t {
    static_assert_uimm_bits!(N, 3);
    {
        simd_shuffle(
            a, a, [N as u32, N as u32, N as u32, N as u32, N as u32, N as u32, N as u32,
            N as u32]
        )
    }
}
///Set all vector lanes to the same value
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vdup_laneq_p8)
pub fn vdup_laneq_p8<const N: i32>(a: poly8x16_t) -> poly8x8_t {
    static_assert_uimm_bits!(N, 4);
    {
        simd_shuffle(
            a, a, [N as u32, N as u32, N as u32, N as u32, N as u32, N as u32, N as u32,
            N as u32]
        )
    }
}
///Set all vector lanes to the same value
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vdup_laneq_s8)
pub fn vdup_laneq_s8<const N: i32>(a: int8x16_t) -> int8x8_t {
    static_assert_uimm_bits!(N, 4);
    {
        simd_shuffle(
            a, a, [N as u32, N as u32, N as u32, N as u32, N as u32, N as u32, N as u32,
            N as u32]
        )
    }
}
///Set all vector lanes to the same value
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vdup_laneq_u8)
pub fn vdup_laneq_u8<const N: i32>(a: uint8x16_t) -> uint8x8_t {
    static_assert_uimm_bits!(N, 4);
    {
        simd_shuffle(
            a, a, [N as u32, N as u32, N as u32, N as u32, N as u32, N as u32, N as u32,
            N as u32]
        )
    }
}
///Set all vector lanes to the same value
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vdupq_laneq_p8)
pub fn vdupq_laneq_p8<const N: i32>(a: poly8x16_t) -> poly8x16_t {
    static_assert_uimm_bits!(N, 4);
    {
        simd_shuffle(
            a, a, [N as u32, N as u32, N as u32, N as u32, N as u32, N as u32, N as u32,
            N as u32, N as u32, N as u32, N as u32, N as u32, N as u32, N as u32, N as
            u32, N as u32]
        )
    }
}
///Set all vector lanes to the same value
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vdupq_laneq_s8)
pub fn vdupq_laneq_s8<const N: i32>(a: int8x16_t) -> int8x16_t {
    static_assert_uimm_bits!(N, 4);
    {
        simd_shuffle(
            a, a, [N as u32, N as u32, N as u32, N as u32, N as u32, N as u32, N as u32,
            N as u32, N as u32, N as u32, N as u32, N as u32, N as u32, N as u32, N as
            u32, N as u32]
        )
    }
}
///Set all vector lanes to the same value
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vdupq_laneq_u8)
pub fn vdupq_laneq_u8<const N: i32>(a: uint8x16_t) -> uint8x16_t {
    static_assert_uimm_bits!(N, 4);
    {
        simd_shuffle(
            a, a, [N as u32, N as u32, N as u32, N as u32, N as u32, N as u32, N as u32,
            N as u32, N as u32, N as u32, N as u32, N as u32, N as u32, N as u32, N as
            u32, N as u32]
        )
    }
}
///Set all vector lanes to the same value
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vdup_laneq_s64)
pub fn vdup_laneq_s64<const N: i32>(a: int64x2_t) -> int64x1_t {
    static_assert_uimm_bits!(N, 1);
    { transmute::<i64, _>(simd_extract(a, N as u32)) }
}
///Set all vector lanes to the same value
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vdup_laneq_u64)
pub fn vdup_laneq_u64<const N: i32>(a: uint64x2_t) -> uint64x1_t {
    static_assert_uimm_bits!(N, 1);
    { transmute::<u64, _>(simd_extract(a, N as u32)) }
}
///Create a new vector with all lanes set to a value
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vdup_n_f16)
pub fn vdup_n_f16(a: f16) -> float16x4_t {
    float16x4_t::splat(a)
}
///Create a new vector with all lanes set to a value
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vdupq_n_f16)
pub fn vdupq_n_f16(a: f16) -> float16x8_t {
    float16x8_t::splat(a)
}
///Duplicate vector element to vector or scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vdup_n_f32)
pub fn vdup_n_f32(value: f32) -> float32x2_t {
    float32x2_t::splat(value)
}
///Duplicate vector element to vector or scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vdup_n_p16)
pub fn vdup_n_p16(value: p16) -> poly16x4_t {
    poly16x4_t::splat(value)
}
///Duplicate vector element to vector or scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vdup_n_p8)
pub fn vdup_n_p8(value: p8) -> poly8x8_t {
    poly8x8_t::splat(value)
}
///Duplicate vector element to vector or scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vdup_n_s16)
pub fn vdup_n_s16(value: i16) -> int16x4_t {
    int16x4_t::splat(value)
}
///Duplicate vector element to vector or scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vdup_n_s32)
pub fn vdup_n_s32(value: i32) -> int32x2_t {
    int32x2_t::splat(value)
}
///Duplicate vector element to vector or scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vdup_n_s64)
pub fn vdup_n_s64(value: i64) -> int64x1_t {
    int64x1_t::splat(value)
}
///Duplicate vector element to vector or scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vdup_n_s8)
pub fn vdup_n_s8(value: i8) -> int8x8_t {
    int8x8_t::splat(value)
}
///Duplicate vector element to vector or scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vdup_n_u16)
pub fn vdup_n_u16(value: u16) -> uint16x4_t {
    uint16x4_t::splat(value)
}
///Duplicate vector element to vector or scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vdup_n_u32)
pub fn vdup_n_u32(value: u32) -> uint32x2_t {
    uint32x2_t::splat(value)
}
///Duplicate vector element to vector or scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vdup_n_u64)
pub fn vdup_n_u64(value: u64) -> uint64x1_t {
    uint64x1_t::splat(value)
}
///Duplicate vector element to vector or scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vdup_n_u8)
pub fn vdup_n_u8(value: u8) -> uint8x8_t {
    uint8x8_t::splat(value)
}
///Duplicate vector element to vector or scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vdupq_n_f32)
pub fn vdupq_n_f32(value: f32) -> float32x4_t {
    float32x4_t::splat(value)
}
///Duplicate vector element to vector or scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vdupq_n_p16)
pub fn vdupq_n_p16(value: p16) -> poly16x8_t {
    poly16x8_t::splat(value)
}
///Duplicate vector element to vector or scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vdupq_n_p8)
pub fn vdupq_n_p8(value: p8) -> poly8x16_t {
    poly8x16_t::splat(value)
}
///Duplicate vector element to vector or scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vdupq_n_s16)
pub fn vdupq_n_s16(value: i16) -> int16x8_t {
    int16x8_t::splat(value)
}
///Duplicate vector element to vector or scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vdupq_n_s32)
pub fn vdupq_n_s32(value: i32) -> int32x4_t {
    int32x4_t::splat(value)
}
///Duplicate vector element to vector or scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vdupq_n_s64)
pub fn vdupq_n_s64(value: i64) -> int64x2_t {
    int64x2_t::splat(value)
}
///Duplicate vector element to vector or scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vdupq_n_s8)
pub fn vdupq_n_s8(value: i8) -> int8x16_t {
    int8x16_t::splat(value)
}
///Duplicate vector element to vector or scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vdupq_n_u16)
pub fn vdupq_n_u16(value: u16) -> uint16x8_t {
    uint16x8_t::splat(value)
}
///Duplicate vector element to vector or scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vdupq_n_u32)
pub fn vdupq_n_u32(value: u32) -> uint32x4_t {
    uint32x4_t::splat(value)
}
///Duplicate vector element to vector or scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vdupq_n_u64)
pub fn vdupq_n_u64(value: u64) -> uint64x2_t {
    uint64x2_t::splat(value)
}
///Duplicate vector element to vector or scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vdupq_n_u8)
pub fn vdupq_n_u8(value: u8) -> uint8x16_t {
    uint8x16_t::splat(value)
}
///Duplicate vector element to vector or scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vdup_n_f32_vfp4)
fn vdup_n_f32_vfp4(value: f32) -> float32x2_t {
    float32x2_t::splat(value)
}
///Duplicate vector element to vector or scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vdupq_n_f32_vfp4)
fn vdupq_n_f32_vfp4(value: f32) -> float32x4_t {
    float32x4_t::splat(value)
}
///Set all vector lanes to the same value
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vdupq_lane_s64)
pub fn vdupq_lane_s64<const N: i32>(a: int64x1_t) -> int64x2_t {
    static_assert!(N == 0);
    { simd_shuffle(a, a, [N as u32, N as u32]) }
}
///Set all vector lanes to the same value
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vdupq_lane_u64)
pub fn vdupq_lane_u64<const N: i32>(a: uint64x1_t) -> uint64x2_t {
    static_assert!(N == 0);
    { simd_shuffle(a, a, [N as u32, N as u32]) }
}
///Set all vector lanes to the same value
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vdupq_laneq_s64)
pub fn vdupq_laneq_s64<const N: i32>(a: int64x2_t) -> int64x2_t {
    static_assert_uimm_bits!(N, 1);
    { simd_shuffle(a, a, [N as u32, N as u32]) }
}
///Set all vector lanes to the same value
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vdupq_laneq_u64)
pub fn vdupq_laneq_u64<const N: i32>(a: uint64x2_t) -> uint64x2_t {
    static_assert_uimm_bits!(N, 1);
    { simd_shuffle(a, a, [N as u32, N as u32]) }
}
///Vector bitwise exclusive or (vector)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/veor_s8)
pub fn veor_s8(a: int8x8_t, b: int8x8_t) -> int8x8_t {
    { simd_xor(a, b) }
}
///Vector bitwise exclusive or (vector)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/veorq_s8)
pub fn veorq_s8(a: int8x16_t, b: int8x16_t) -> int8x16_t {
    { simd_xor(a, b) }
}
///Vector bitwise exclusive or (vector)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/veor_s16)
pub fn veor_s16(a: int16x4_t, b: int16x4_t) -> int16x4_t {
    { simd_xor(a, b) }
}
///Vector bitwise exclusive or (vector)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/veorq_s16)
pub fn veorq_s16(a: int16x8_t, b: int16x8_t) -> int16x8_t {
    { simd_xor(a, b) }
}
///Vector bitwise exclusive or (vector)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/veor_s32)
pub fn veor_s32(a: int32x2_t, b: int32x2_t) -> int32x2_t {
    { simd_xor(a, b) }
}
///Vector bitwise exclusive or (vector)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/veorq_s32)
pub fn veorq_s32(a: int32x4_t, b: int32x4_t) -> int32x4_t {
    { simd_xor(a, b) }
}
///Vector bitwise exclusive or (vector)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/veor_s64)
pub fn veor_s64(a: int64x1_t, b: int64x1_t) -> int64x1_t {
    { simd_xor(a, b) }
}
///Vector bitwise exclusive or (vector)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/veorq_s64)
pub fn veorq_s64(a: int64x2_t, b: int64x2_t) -> int64x2_t {
    { simd_xor(a, b) }
}
///Vector bitwise exclusive or (vector)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/veor_u8)
pub fn veor_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
    { simd_xor(a, b) }
}
///Vector bitwise exclusive or (vector)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/veorq_u8)
pub fn veorq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
    { simd_xor(a, b) }
}
///Vector bitwise exclusive or (vector)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/veor_u16)
pub fn veor_u16(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
    { simd_xor(a, b) }
}
///Vector bitwise exclusive or (vector)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/veorq_u16)
pub fn veorq_u16(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
    { simd_xor(a, b) }
}
///Vector bitwise exclusive or (vector)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/veor_u32)
pub fn veor_u32(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
    { simd_xor(a, b) }
}
///Vector bitwise exclusive or (vector)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/veorq_u32)
pub fn veorq_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
    { simd_xor(a, b) }
}
///Vector bitwise exclusive or (vector)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/veor_u64)
pub fn veor_u64(a: uint64x1_t, b: uint64x1_t) -> uint64x1_t {
    { simd_xor(a, b) }
}
///Vector bitwise exclusive or (vector)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/veorq_u64)
pub fn veorq_u64(a: uint64x2_t, b: uint64x2_t) -> uint64x2_t {
    { simd_xor(a, b) }
}
///Extract vector from pair of vectors
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vext_f16)
pub fn vext_f16<const N: i32>(a: float16x4_t, b: float16x4_t) -> float16x4_t {
    static_assert_uimm_bits!(N, 2);
    {
        match N & 0b11 {
            0 => simd_shuffle(a, b, [0, 1, 2, 3]),
            1 => simd_shuffle(a, b, [1, 2, 3, 4]),
            2 => simd_shuffle(a, b, [2, 3, 4, 5]),
            3 => simd_shuffle(a, b, [3, 4, 5, 6]),
            _ => unreachable_unchecked(),
        }
    }
}
///Extract vector from pair of vectors
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vext_f32)
pub fn vext_f32<const N: i32>(a: float32x2_t, b: float32x2_t) -> float32x2_t {
    static_assert_uimm_bits!(N, 1);
    {
        match N & 0b1 {
            0 => simd_shuffle(a, b, [0, 1]),
            1 => simd_shuffle(a, b, [1, 2]),
            _ => unreachable_unchecked(),
        }
    }
}
///Extract vector from pair of vectors
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vext_s32)
pub fn vext_s32<const N: i32>(a: int32x2_t, b: int32x2_t) -> int32x2_t {
    static_assert_uimm_bits!(N, 1);
    {
        match N & 0b1 {
            0 => simd_shuffle(a, b, [0, 1]),
            1 => simd_shuffle(a, b, [1, 2]),
            _ => unreachable_unchecked(),
        }
    }
}
///Extract vector from pair of vectors
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vext_u32)
pub fn vext_u32<const N: i32>(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
    static_assert_uimm_bits!(N, 1);
    {
        match N & 0b1 {
            0 => simd_shuffle(a, b, [0, 1]),
            1 => simd_shuffle(a, b, [1, 2]),
            _ => unreachable_unchecked(),
        }
    }
}
///Extract vector from pair of vectors
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vext_s8)
pub fn vext_s8<const N: i32>(a: int8x8_t, b: int8x8_t) -> int8x8_t {
    static_assert_uimm_bits!(N, 3);
    {
        match N & 0b111 {
            0 => simd_shuffle(a, b, [0, 1, 2, 3, 4, 5, 6, 7]),
            1 => simd_shuffle(a, b, [1, 2, 3, 4, 5, 6, 7, 8]),
            2 => simd_shuffle(a, b, [2, 3, 4, 5, 6, 7, 8, 9]),
            3 => simd_shuffle(a, b, [3, 4, 5, 6, 7, 8, 9, 10]),
            4 => simd_shuffle(a, b, [4, 5, 6, 7, 8, 9, 10, 11]),
            5 => simd_shuffle(a, b, [5, 6, 7, 8, 9, 10, 11, 12]),
            6 => simd_shuffle(a, b, [6, 7, 8, 9, 10, 11, 12, 13]),
            7 => simd_shuffle(a, b, [7, 8, 9, 10, 11, 12, 13, 14]),
            _ => unreachable_unchecked(),
        }
    }
}
///Extract vector from pair of vectors
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vextq_s16)
pub fn vextq_s16<const N: i32>(a: int16x8_t, b: int16x8_t) -> int16x8_t {
    static_assert_uimm_bits!(N, 3);
    {
        match N & 0b111 {
            0 => simd_shuffle(a, b, [0, 1, 2, 3, 4, 5, 6, 7]),
            1 => simd_shuffle(a, b, [1, 2, 3, 4, 5, 6, 7, 8]),
            2 => simd_shuffle(a, b, [2, 3, 4, 5, 6, 7, 8, 9]),
            3 => simd_shuffle(a, b, [3, 4, 5, 6, 7, 8, 9, 10]),
            4 => simd_shuffle(a, b, [4, 5, 6, 7, 8, 9, 10, 11]),
            5 => simd_shuffle(a, b, [5, 6, 7, 8, 9, 10, 11, 12]),
            6 => simd_shuffle(a, b, [6, 7, 8, 9, 10, 11, 12, 13]),
            7 => simd_shuffle(a, b, [7, 8, 9, 10, 11, 12, 13, 14]),
            _ => unreachable_unchecked(),
        }
    }
}
///Extract vector from pair of vectors
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vext_u8)
pub fn vext_u8<const N: i32>(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
    static_assert_uimm_bits!(N, 3);
    {
        match N & 0b111 {
            0 => simd_shuffle(a, b, [0, 1, 2, 3, 4, 5, 6, 7]),
            1 => simd_shuffle(a, b, [1, 2, 3, 4, 5, 6, 7, 8]),
            2 => simd_shuffle(a, b, [2, 3, 4, 5, 6, 7, 8, 9]),
            3 => simd_shuffle(a, b, [3, 4, 5, 6, 7, 8, 9, 10]),
            4 => simd_shuffle(a, b, [4, 5, 6, 7, 8, 9, 10, 11]),
            5 => simd_shuffle(a, b, [5, 6, 7, 8, 9, 10, 11, 12]),
            6 => simd_shuffle(a, b, [6, 7, 8, 9, 10, 11, 12, 13]),
            7 => simd_shuffle(a, b, [7, 8, 9, 10, 11, 12, 13, 14]),
            _ => unreachable_unchecked(),
        }
    }
}
///Extract vector from pair of vectors
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vextq_u16)
pub fn vextq_u16<const N: i32>(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
    static_assert_uimm_bits!(N, 3);
    {
        match N & 0b111 {
            0 => simd_shuffle(a, b, [0, 1, 2, 3, 4, 5, 6, 7]),
            1 => simd_shuffle(a, b, [1, 2, 3, 4, 5, 6, 7, 8]),
            2 => simd_shuffle(a, b, [2, 3, 4, 5, 6, 7, 8, 9]),
            3 => simd_shuffle(a, b, [3, 4, 5, 6, 7, 8, 9, 10]),
            4 => simd_shuffle(a, b, [4, 5, 6, 7, 8, 9, 10, 11]),
            5 => simd_shuffle(a, b, [5, 6, 7, 8, 9, 10, 11, 12]),
            6 => simd_shuffle(a, b, [6, 7, 8, 9, 10, 11, 12, 13]),
            7 => simd_shuffle(a, b, [7, 8, 9, 10, 11, 12, 13, 14]),
            _ => unreachable_unchecked(),
        }
    }
}
///Extract vector from pair of vectors
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vext_p8)
pub fn vext_p8<const N: i32>(a: poly8x8_t, b: poly8x8_t) -> poly8x8_t {
    static_assert_uimm_bits!(N, 3);
    {
        match N & 0b111 {
            0 => simd_shuffle(a, b, [0, 1, 2, 3, 4, 5, 6, 7]),
            1 => simd_shuffle(a, b, [1, 2, 3, 4, 5, 6, 7, 8]),
            2 => simd_shuffle(a, b, [2, 3, 4, 5, 6, 7, 8, 9]),
            3 => simd_shuffle(a, b, [3, 4, 5, 6, 7, 8, 9, 10]),
            4 => simd_shuffle(a, b, [4, 5, 6, 7, 8, 9, 10, 11]),
            5 => simd_shuffle(a, b, [5, 6, 7, 8, 9, 10, 11, 12]),
            6 => simd_shuffle(a, b, [6, 7, 8, 9, 10, 11, 12, 13]),
            7 => simd_shuffle(a, b, [7, 8, 9, 10, 11, 12, 13, 14]),
            _ => unreachable_unchecked(),
        }
    }
}
///Extract vector from pair of vectors
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vextq_p16)
pub fn vextq_p16<const N: i32>(a: poly16x8_t, b: poly16x8_t) -> poly16x8_t {
    static_assert_uimm_bits!(N, 3);
    {
        match N & 0b111 {
            0 => simd_shuffle(a, b, [0, 1, 2, 3, 4, 5, 6, 7]),
            1 => simd_shuffle(a, b, [1, 2, 3, 4, 5, 6, 7, 8]),
            2 => simd_shuffle(a, b, [2, 3, 4, 5, 6, 7, 8, 9]),
            3 => simd_shuffle(a, b, [3, 4, 5, 6, 7, 8, 9, 10]),
            4 => simd_shuffle(a, b, [4, 5, 6, 7, 8, 9, 10, 11]),
            5 => simd_shuffle(a, b, [5, 6, 7, 8, 9, 10, 11, 12]),
            6 => simd_shuffle(a, b, [6, 7, 8, 9, 10, 11, 12, 13]),
            7 => simd_shuffle(a, b, [7, 8, 9, 10, 11, 12, 13, 14]),
            _ => unreachable_unchecked(),
        }
    }
}
///Extract vector from pair of vectors
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vextq_f16)
pub fn vextq_f16<const N: i32>(a: float16x8_t, b: float16x8_t) -> float16x8_t {
    static_assert_uimm_bits!(N, 3);
    {
        match N & 0b111 {
            0 => simd_shuffle(a, b, [0, 1, 2, 3, 4, 5, 6, 7]),
            1 => simd_shuffle(a, b, [1, 2, 3, 4, 5, 6, 7, 8]),
            2 => simd_shuffle(a, b, [2, 3, 4, 5, 6, 7, 8, 9]),
            3 => simd_shuffle(a, b, [3, 4, 5, 6, 7, 8, 9, 10]),
            4 => simd_shuffle(a, b, [4, 5, 6, 7, 8, 9, 10, 11]),
            5 => simd_shuffle(a, b, [5, 6, 7, 8, 9, 10, 11, 12]),
            6 => simd_shuffle(a, b, [6, 7, 8, 9, 10, 11, 12, 13]),
            7 => simd_shuffle(a, b, [7, 8, 9, 10, 11, 12, 13, 14]),
            _ => unreachable_unchecked(),
        }
    }
}
///Extract vector from pair of vectors
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vextq_f32)
pub fn vextq_f32<const N: i32>(a: float32x4_t, b: float32x4_t) -> float32x4_t {
    static_assert_uimm_bits!(N, 2);
    {
        match N & 0b11 {
            0 => simd_shuffle(a, b, [0, 1, 2, 3]),
            1 => simd_shuffle(a, b, [1, 2, 3, 4]),
            2 => simd_shuffle(a, b, [2, 3, 4, 5]),
            3 => simd_shuffle(a, b, [3, 4, 5, 6]),
            _ => unreachable_unchecked(),
        }
    }
}
///Extract vector from pair of vectors
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vext_s16)
pub fn vext_s16<const N: i32>(a: int16x4_t, b: int16x4_t) -> int16x4_t {
    static_assert_uimm_bits!(N, 2);
    {
        match N & 0b11 {
            0 => simd_shuffle(a, b, [0, 1, 2, 3]),
            1 => simd_shuffle(a, b, [1, 2, 3, 4]),
            2 => simd_shuffle(a, b, [2, 3, 4, 5]),
            3 => simd_shuffle(a, b, [3, 4, 5, 6]),
            _ => unreachable_unchecked(),
        }
    }
}
///Extract vector from pair of vectors
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vextq_s32)
pub fn vextq_s32<const N: i32>(a: int32x4_t, b: int32x4_t) -> int32x4_t {
    static_assert_uimm_bits!(N, 2);
    {
        match N & 0b11 {
            0 => simd_shuffle(a, b, [0, 1, 2, 3]),
            1 => simd_shuffle(a, b, [1, 2, 3, 4]),
            2 => simd_shuffle(a, b, [2, 3, 4, 5]),
            3 => simd_shuffle(a, b, [3, 4, 5, 6]),
            _ => unreachable_unchecked(),
        }
    }
}
///Extract vector from pair of vectors
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vext_u16)
pub fn vext_u16<const N: i32>(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
    static_assert_uimm_bits!(N, 2);
    {
        match N & 0b11 {
            0 => simd_shuffle(a, b, [0, 1, 2, 3]),
            1 => simd_shuffle(a, b, [1, 2, 3, 4]),
            2 => simd_shuffle(a, b, [2, 3, 4, 5]),
            3 => simd_shuffle(a, b, [3, 4, 5, 6]),
            _ => unreachable_unchecked(),
        }
    }
}
///Extract vector from pair of vectors
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vextq_u32)
pub fn vextq_u32<const N: i32>(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
    static_assert_uimm_bits!(N, 2);
    {
        match N & 0b11 {
            0 => simd_shuffle(a, b, [0, 1, 2, 3]),
            1 => simd_shuffle(a, b, [1, 2, 3, 4]),
            2 => simd_shuffle(a, b, [2, 3, 4, 5]),
            3 => simd_shuffle(a, b, [3, 4, 5, 6]),
            _ => unreachable_unchecked(),
        }
    }
}
///Extract vector from pair of vectors
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vext_p16)
pub fn vext_p16<const N: i32>(a: poly16x4_t, b: poly16x4_t) -> poly16x4_t {
    static_assert_uimm_bits!(N, 2);
    {
        match N & 0b11 {
            0 => simd_shuffle(a, b, [0, 1, 2, 3]),
            1 => simd_shuffle(a, b, [1, 2, 3, 4]),
            2 => simd_shuffle(a, b, [2, 3, 4, 5]),
            3 => simd_shuffle(a, b, [3, 4, 5, 6]),
            _ => unreachable_unchecked(),
        }
    }
}
///Extract vector from pair of vectors
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vextq_s64)
pub fn vextq_s64<const N: i32>(a: int64x2_t, b: int64x2_t) -> int64x2_t {
    static_assert_uimm_bits!(N, 1);
    {
        match N & 0b1 {
            0 => simd_shuffle(a, b, [0, 1]),
            1 => simd_shuffle(a, b, [1, 2]),
            _ => unreachable_unchecked(),
        }
    }
}
///Extract vector from pair of vectors
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vextq_u64)
pub fn vextq_u64<const N: i32>(a: uint64x2_t, b: uint64x2_t) -> uint64x2_t {
    static_assert_uimm_bits!(N, 1);
    {
        match N & 0b1 {
            0 => simd_shuffle(a, b, [0, 1]),
            1 => simd_shuffle(a, b, [1, 2]),
            _ => unreachable_unchecked(),
        }
    }
}
///Extract vector from pair of vectors
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vextq_s8)
pub fn vextq_s8<const N: i32>(a: int8x16_t, b: int8x16_t) -> int8x16_t {
    static_assert_uimm_bits!(N, 4);
    {
        match N & 0b1111 {
            0 => {
                simd_shuffle(
                    a, b, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
                )
            }
            1 => {
                simd_shuffle(
                    a, b, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]
                )
            }
            2 => {
                simd_shuffle(
                    a, b, [2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17]
                )
            }
            3 => {
                simd_shuffle(
                    a, b, [3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18]
                )
            }
            4 => {
                simd_shuffle(
                    a, b, [4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]
                )
            }
            5 => {
                simd_shuffle(
                    a, b, [5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]
                )
            }
            6 => {
                simd_shuffle(
                    a, b, [6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21]
                )
            }
            7 => {
                simd_shuffle(
                    a, b, [7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22]
                )
            }
            8 => {
                simd_shuffle(
                    a, b, [8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23]
                )
            }
            9 => {
                simd_shuffle(
                    a, b, [9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24]
                )
            }
            10 => {
                simd_shuffle(
                    a, b, [10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
                    25]
                )
            }
            11 => {
                simd_shuffle(
                    a, b, [11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
                    26]
                )
            }
            12 => {
                simd_shuffle(
                    a, b, [12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
                    27]
                )
            }
            13 => {
                simd_shuffle(
                    a, b, [13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27,
                    28]
                )
            }
            14 => {
                simd_shuffle(
                    a, b, [14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28,
                    29]
                )
            }
            15 => {
                simd_shuffle(
                    a, b, [15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29,
                    30]
                )
            }
            _ => unreachable_unchecked(),
        }
    }
}
///Extract vector from pair of vectors
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vextq_u8)
pub fn vextq_u8<const N: i32>(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
    static_assert_uimm_bits!(N, 4);
    {
        match N & 0b1111 {
            0 => {
                simd_shuffle(
                    a, b, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
                )
            }
            1 => {
                simd_shuffle(
                    a, b, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]
                )
            }
            2 => {
                simd_shuffle(
                    a, b, [2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17]
                )
            }
            3 => {
                simd_shuffle(
                    a, b, [3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18]
                )
            }
            4 => {
                simd_shuffle(
                    a, b, [4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]
                )
            }
            5 => {
                simd_shuffle(
                    a, b, [5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]
                )
            }
            6 => {
                simd_shuffle(
                    a, b, [6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21]
                )
            }
            7 => {
                simd_shuffle(
                    a, b, [7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22]
                )
            }
            8 => {
                simd_shuffle(
                    a, b, [8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23]
                )
            }
            9 => {
                simd_shuffle(
                    a, b, [9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24]
                )
            }
            10 => {
                simd_shuffle(
                    a, b, [10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
                    25]
                )
            }
            11 => {
                simd_shuffle(
                    a, b, [11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
                    26]
                )
            }
            12 => {
                simd_shuffle(
                    a, b, [12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
                    27]
                )
            }
            13 => {
                simd_shuffle(
                    a, b, [13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27,
                    28]
                )
            }
            14 => {
                simd_shuffle(
                    a, b, [14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28,
                    29]
                )
            }
            15 => {
                simd_shuffle(
                    a, b, [15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29,
                    30]
                )
            }
            _ => unreachable_unchecked(),
        }
    }
}
///Extract vector from pair of vectors
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vextq_p8)
pub fn vextq_p8<const N: i32>(a: poly8x16_t, b: poly8x16_t) -> poly8x16_t {
    static_assert_uimm_bits!(N, 4);
    {
        match N & 0b1111 {
            0 => {
                simd_shuffle(
                    a, b, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
                )
            }
            1 => {
                simd_shuffle(
                    a, b, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]
                )
            }
            2 => {
                simd_shuffle(
                    a, b, [2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17]
                )
            }
            3 => {
                simd_shuffle(
                    a, b, [3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18]
                )
            }
            4 => {
                simd_shuffle(
                    a, b, [4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]
                )
            }
            5 => {
                simd_shuffle(
                    a, b, [5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]
                )
            }
            6 => {
                simd_shuffle(
                    a, b, [6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21]
                )
            }
            7 => {
                simd_shuffle(
                    a, b, [7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22]
                )
            }
            8 => {
                simd_shuffle(
                    a, b, [8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23]
                )
            }
            9 => {
                simd_shuffle(
                    a, b, [9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24]
                )
            }
            10 => {
                simd_shuffle(
                    a, b, [10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
                    25]
                )
            }
            11 => {
                simd_shuffle(
                    a, b, [11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
                    26]
                )
            }
            12 => {
                simd_shuffle(
                    a, b, [12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
                    27]
                )
            }
            13 => {
                simd_shuffle(
                    a, b, [13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27,
                    28]
                )
            }
            14 => {
                simd_shuffle(
                    a, b, [14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28,
                    29]
                )
            }
            15 => {
                simd_shuffle(
                    a, b, [15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29,
                    30]
                )
            }
            _ => unreachable_unchecked(),
        }
    }
}
///Floating-point fused Multiply-Add to accumulator (vector)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vfma_f16)
pub fn vfma_f16(a: float16x4_t, b: float16x4_t, c: float16x4_t) -> float16x4_t {
    { simd_fma(b, c, a) }
}
///Floating-point fused Multiply-Add to accumulator (vector)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vfmaq_f16)
pub fn vfmaq_f16(a: float16x8_t, b: float16x8_t, c: float16x8_t) -> float16x8_t {
    { simd_fma(b, c, a) }
}
///Floating-point fused Multiply-Add to accumulator(vector)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vfma_f32)
pub fn vfma_f32(a: float32x2_t, b: float32x2_t, c: float32x2_t) -> float32x2_t {
    { simd_fma(b, c, a) }
}
///Floating-point fused Multiply-Add to accumulator(vector)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vfmaq_f32)
pub fn vfmaq_f32(a: float32x4_t, b: float32x4_t, c: float32x4_t) -> float32x4_t {
    { simd_fma(b, c, a) }
}
///Floating-point fused Multiply-Add to accumulator(vector)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vfma_n_f32)
pub fn vfma_n_f32(a: float32x2_t, b: float32x2_t, c: f32) -> float32x2_t {
    vfma_f32(a, b, vdup_n_f32_vfp4(c))
}
///Floating-point fused Multiply-Add to accumulator(vector)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vfmaq_n_f32)
pub fn vfmaq_n_f32(a: float32x4_t, b: float32x4_t, c: f32) -> float32x4_t {
    vfmaq_f32(a, b, vdupq_n_f32_vfp4(c))
}
///Floating-point fused multiply-subtract from accumulator
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vfms_f16)
pub fn vfms_f16(a: float16x4_t, b: float16x4_t, c: float16x4_t) -> float16x4_t {
    {
        let b: float16x4_t = simd_neg(b);
        vfma_f16(a, b, c)
    }
}
///Floating-point fused multiply-subtract from accumulator
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vfmsq_f16)
pub fn vfmsq_f16(a: float16x8_t, b: float16x8_t, c: float16x8_t) -> float16x8_t {
    {
        let b: float16x8_t = simd_neg(b);
        vfmaq_f16(a, b, c)
    }
}
///Floating-point fused multiply-subtract from accumulator
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vfms_f32)
pub fn vfms_f32(a: float32x2_t, b: float32x2_t, c: float32x2_t) -> float32x2_t {
    {
        let b: float32x2_t = simd_neg(b);
        vfma_f32(a, b, c)
    }
}
///Floating-point fused multiply-subtract from accumulator
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vfmsq_f32)
pub fn vfmsq_f32(a: float32x4_t, b: float32x4_t, c: float32x4_t) -> float32x4_t {
    {
        let b: float32x4_t = simd_neg(b);
        vfmaq_f32(a, b, c)
    }
}
///Floating-point fused Multiply-subtract to accumulator(vector)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vfms_n_f32)
pub fn vfms_n_f32(a: float32x2_t, b: float32x2_t, c: f32) -> float32x2_t {
    vfms_f32(a, b, vdup_n_f32_vfp4(c))
}
///Floating-point fused Multiply-subtract to accumulator(vector)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vfmsq_n_f32)
pub fn vfmsq_n_f32(a: float32x4_t, b: float32x4_t, c: f32) -> float32x4_t {
    vfmsq_f32(a, b, vdupq_n_f32_vfp4(c))
}
///Duplicate vector element to vector
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vget_high_f16)
pub fn vget_high_f16(a: float16x8_t) -> float16x4_t {
    { simd_shuffle(a, a, [4, 5, 6, 7]) }
}
///Duplicate vector element to vector
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vget_low_f16)
pub fn vget_low_f16(a: float16x8_t) -> float16x4_t {
    { simd_shuffle(a, a, [0, 1, 2, 3]) }
}
///Duplicate vector element to vector or scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vget_high_f32)
pub fn vget_high_f32(a: float32x4_t) -> float32x2_t {
    { simd_shuffle(a, a, [2, 3]) }
}
///Duplicate vector element to vector or scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vget_high_p16)
pub fn vget_high_p16(a: poly16x8_t) -> poly16x4_t {
    { simd_shuffle(a, a, [4, 5, 6, 7]) }
}
///Duplicate vector element to vector or scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vget_high_p8)
pub fn vget_high_p8(a: poly8x16_t) -> poly8x8_t {
    { simd_shuffle(a, a, [8, 9, 10, 11, 12, 13, 14, 15]) }
}
///Duplicate vector element to vector or scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vget_high_s16)
pub fn vget_high_s16(a: int16x8_t) -> int16x4_t {
    { simd_shuffle(a, a, [4, 5, 6, 7]) }
}
///Duplicate vector element to vector or scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vget_high_s32)
pub fn vget_high_s32(a: int32x4_t) -> int32x2_t {
    { simd_shuffle(a, a, [2, 3]) }
}
///Duplicate vector element to vector or scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vget_high_s8)
pub fn vget_high_s8(a: int8x16_t) -> int8x8_t {
    { simd_shuffle(a, a, [8, 9, 10, 11, 12, 13, 14, 15]) }
}
///Duplicate vector element to vector or scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vget_high_u16)
pub fn vget_high_u16(a: uint16x8_t) -> uint16x4_t {
    { simd_shuffle(a, a, [4, 5, 6, 7]) }
}
///Duplicate vector element to vector or scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vget_high_u32)
pub fn vget_high_u32(a: uint32x4_t) -> uint32x2_t {
    { simd_shuffle(a, a, [2, 3]) }
}
///Duplicate vector element to vector or scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vget_high_u8)
pub fn vget_high_u8(a: uint8x16_t) -> uint8x8_t {
    { simd_shuffle(a, a, [8, 9, 10, 11, 12, 13, 14, 15]) }
}
///Duplicate vector element to vector or scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vget_high_s64)
pub fn vget_high_s64(a: int64x2_t) -> int64x1_t {
    { int64x1_t([simd_extract(a, 1)]) }
}
///Duplicate vector element to vector or scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vget_high_u64)
pub fn vget_high_u64(a: uint64x2_t) -> uint64x1_t {
    { uint64x1_t([simd_extract(a, 1)]) }
}
///Duplicate vector element to scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vget_lane_f16)
pub fn vget_lane_f16<const LANE: i32>(a: float16x4_t) -> f16 {
    static_assert_uimm_bits!(LANE, 2);
    { simd_extract(a, LANE as u32) }
}
///Duplicate vector element to scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vgetq_lane_f16)
pub fn vgetq_lane_f16<const LANE: i32>(a: float16x8_t) -> f16 {
    static_assert_uimm_bits!(LANE, 3);
    { simd_extract(a, LANE as u32) }
}
///Move vector element to general-purpose register
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vget_lane_f32)
pub fn vget_lane_f32<const IMM5: i32>(v: float32x2_t) -> f32 {
    static_assert_uimm_bits!(IMM5, 1);
    { simd_extract(v, IMM5 as u32) }
}
///Move vector element to general-purpose register
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vget_lane_p16)
pub fn vget_lane_p16<const IMM5: i32>(v: poly16x4_t) -> p16 {
    static_assert_uimm_bits!(IMM5, 2);
    { simd_extract(v, IMM5 as u32) }
}
///Move vector element to general-purpose register
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vget_lane_p8)
pub fn vget_lane_p8<const IMM5: i32>(v: poly8x8_t) -> p8 {
    static_assert_uimm_bits!(IMM5, 3);
    { simd_extract(v, IMM5 as u32) }
}
///Move vector element to general-purpose register
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vget_lane_s16)
pub fn vget_lane_s16<const IMM5: i32>(v: int16x4_t) -> i16 {
    static_assert_uimm_bits!(IMM5, 2);
    { simd_extract(v, IMM5 as u32) }
}
///Move vector element to general-purpose register
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vget_lane_s32)
pub fn vget_lane_s32<const IMM5: i32>(v: int32x2_t) -> i32 {
    static_assert_uimm_bits!(IMM5, 1);
    { simd_extract(v, IMM5 as u32) }
}
///Move vector element to general-purpose register
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vget_lane_s8)
pub fn vget_lane_s8<const IMM5: i32>(v: int8x8_t) -> i8 {
    static_assert_uimm_bits!(IMM5, 3);
    { simd_extract(v, IMM5 as u32) }
}
///Move vector element to general-purpose register
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vget_lane_u16)
pub fn vget_lane_u16<const IMM5: i32>(v: uint16x4_t) -> u16 {
    static_assert_uimm_bits!(IMM5, 2);
    { simd_extract(v, IMM5 as u32) }
}
///Move vector element to general-purpose register
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vget_lane_u32)
pub fn vget_lane_u32<const IMM5: i32>(v: uint32x2_t) -> u32 {
    static_assert_uimm_bits!(IMM5, 1);
    { simd_extract(v, IMM5 as u32) }
}
///Move vector element to general-purpose register
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vget_lane_u8)
pub fn vget_lane_u8<const IMM5: i32>(v: uint8x8_t) -> u8 {
    static_assert_uimm_bits!(IMM5, 3);
    { simd_extract(v, IMM5 as u32) }
}
///Move vector element to general-purpose register
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vgetq_lane_f32)
pub fn vgetq_lane_f32<const IMM5: i32>(v: float32x4_t) -> f32 {
    static_assert_uimm_bits!(IMM5, 2);
    { simd_extract(v, IMM5 as u32) }
}
///Move vector element to general-purpose register
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vgetq_lane_p16)
pub fn vgetq_lane_p16<const IMM5: i32>(v: poly16x8_t) -> p16 {
    static_assert_uimm_bits!(IMM5, 3);
    { simd_extract(v, IMM5 as u32) }
}
///Move vector element to general-purpose register
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vgetq_lane_p64)
pub fn vgetq_lane_p64<const IMM5: i32>(v: poly64x2_t) -> p64 {
    static_assert_uimm_bits!(IMM5, 1);
    { simd_extract(v, IMM5 as u32) }
}
///Move vector element to general-purpose register
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vgetq_lane_p8)
pub fn vgetq_lane_p8<const IMM5: i32>(v: poly8x16_t) -> p8 {
    static_assert_uimm_bits!(IMM5, 4);
    { simd_extract(v, IMM5 as u32) }
}
///Move vector element to general-purpose register
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vgetq_lane_s16)
pub fn vgetq_lane_s16<const IMM5: i32>(v: int16x8_t) -> i16 {
    static_assert_uimm_bits!(IMM5, 3);
    { simd_extract(v, IMM5 as u32) }
}
///Move vector element to general-purpose register
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vgetq_lane_s32)
pub fn vgetq_lane_s32<const IMM5: i32>(v: int32x4_t) -> i32 {
    static_assert_uimm_bits!(IMM5, 2);
    { simd_extract(v, IMM5 as u32) }
}
///Move vector element to general-purpose register
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vgetq_lane_s64)
pub fn vgetq_lane_s64<const IMM5: i32>(v: int64x2_t) -> i64 {
    static_assert_uimm_bits!(IMM5, 1);
    { simd_extract(v, IMM5 as u32) }
}
///Move vector element to general-purpose register
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vgetq_lane_s8)
pub fn vgetq_lane_s8<const IMM5: i32>(v: int8x16_t) -> i8 {
    static_assert_uimm_bits!(IMM5, 4);
    { simd_extract(v, IMM5 as u32) }
}
///Move vector element to general-purpose register
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vgetq_lane_u16)
pub fn vgetq_lane_u16<const IMM5: i32>(v: uint16x8_t) -> u16 {
    static_assert_uimm_bits!(IMM5, 3);
    { simd_extract(v, IMM5 as u32) }
}
///Move vector element to general-purpose register
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vgetq_lane_u32)
pub fn vgetq_lane_u32<const IMM5: i32>(v: uint32x4_t) -> u32 {
    static_assert_uimm_bits!(IMM5, 2);
    { simd_extract(v, IMM5 as u32) }
}
///Move vector element to general-purpose register
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vgetq_lane_u64)
pub fn vgetq_lane_u64<const IMM5: i32>(v: uint64x2_t) -> u64 {
    static_assert_uimm_bits!(IMM5, 2);
    { simd_extract(v, IMM5 as u32) }
}
///Move vector element to general-purpose register
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vgetq_lane_u8)
pub fn vgetq_lane_u8<const IMM5: i32>(v: uint8x16_t) -> u8 {
    static_assert_uimm_bits!(IMM5, 4);
    { simd_extract(v, IMM5 as u32) }
}
///Move vector element to general-purpose register
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vget_lane_p64)
pub fn vget_lane_p64<const IMM5: i32>(v: poly64x1_t) -> p64 {
    static_assert!(IMM5 == 0);
    { simd_extract(v, IMM5 as u32) }
}
///Move vector element to general-purpose register
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vget_lane_s64)
pub fn vget_lane_s64<const IMM5: i32>(v: int64x1_t) -> i64 {
    static_assert!(IMM5 == 0);
    { simd_extract(v, IMM5 as u32) }
}
///Move vector element to general-purpose register
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vget_lane_u64)
pub fn vget_lane_u64<const IMM5: i32>(v: uint64x1_t) -> u64 {
    static_assert!(IMM5 == 0);
    { simd_extract(v, 0) }
}
///Duplicate vector element to vector or scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vget_low_f32)
pub fn vget_low_f32(a: float32x4_t) -> float32x2_t {
    { simd_shuffle(a, a, [0, 1]) }
}
///Duplicate vector element to vector or scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vget_low_p16)
pub fn vget_low_p16(a: poly16x8_t) -> poly16x4_t {
    { simd_shuffle(a, a, [0, 1, 2, 3]) }
}
///Duplicate vector element to vector or scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vget_low_p8)
pub fn vget_low_p8(a: poly8x16_t) -> poly8x8_t {
    { simd_shuffle(a, a, [0, 1, 2, 3, 4, 5, 6, 7]) }
}
///Duplicate vector element to vector or scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vget_low_s16)
pub fn vget_low_s16(a: int16x8_t) -> int16x4_t {
    { simd_shuffle(a, a, [0, 1, 2, 3]) }
}
///Duplicate vector element to vector or scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vget_low_s32)
pub fn vget_low_s32(a: int32x4_t) -> int32x2_t {
    { simd_shuffle(a, a, [0, 1]) }
}
///Duplicate vector element to vector or scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vget_low_s8)
pub fn vget_low_s8(a: int8x16_t) -> int8x8_t {
    { simd_shuffle(a, a, [0, 1, 2, 3, 4, 5, 6, 7]) }
}
///Duplicate vector element to vector or scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vget_low_u16)
pub fn vget_low_u16(a: uint16x8_t) -> uint16x4_t {
    { simd_shuffle(a, a, [0, 1, 2, 3]) }
}
///Duplicate vector element to vector or scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vget_low_u32)
pub fn vget_low_u32(a: uint32x4_t) -> uint32x2_t {
    { simd_shuffle(a, a, [0, 1]) }
}
///Duplicate vector element to vector or scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vget_low_u8)
pub fn vget_low_u8(a: uint8x16_t) -> uint8x8_t {
    { simd_shuffle(a, a, [0, 1, 2, 3, 4, 5, 6, 7]) }
}
///Duplicate vector element to vector or scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vget_low_s64)
pub fn vget_low_s64(a: int64x2_t) -> int64x1_t {
    { int64x1_t([simd_extract(a, 0)]) }
}
///Duplicate vector element to vector or scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vget_low_u64)
pub fn vget_low_u64(a: uint64x2_t) -> uint64x1_t {
    { uint64x1_t([simd_extract(a, 0)]) }
}
///Halving add
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vhadd_s8)
pub fn vhadd_s8(a: int8x8_t, b: int8x8_t) -> int8x8_t {
    { _vhadd_s8(a, b) }
}
///Halving add
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vhaddq_s8)
pub fn vhaddq_s8(a: int8x16_t, b: int8x16_t) -> int8x16_t {
    { _vhaddq_s8(a, b) }
}
///Halving add
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vhadd_s16)
pub fn vhadd_s16(a: int16x4_t, b: int16x4_t) -> int16x4_t {
    { _vhadd_s16(a, b) }
}
///Halving add
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vhaddq_s16)
pub fn vhaddq_s16(a: int16x8_t, b: int16x8_t) -> int16x8_t {
    { _vhaddq_s16(a, b) }
}
///Halving add
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vhadd_s32)
pub fn vhadd_s32(a: int32x2_t, b: int32x2_t) -> int32x2_t {
    { _vhadd_s32(a, b) }
}
///Halving add
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vhaddq_s32)
pub fn vhaddq_s32(a: int32x4_t, b: int32x4_t) -> int32x4_t {
    { _vhaddq_s32(a, b) }
}
///Halving add
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vhadd_u8)
pub fn vhadd_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
    { _vhadd_u8(a, b) }
}
///Halving add
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vhaddq_u8)
pub fn vhaddq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
    { _vhaddq_u8(a, b) }
}
///Halving add
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vhadd_u16)
pub fn vhadd_u16(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
    { _vhadd_u16(a, b) }
}
///Halving add
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vhaddq_u16)
pub fn vhaddq_u16(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
    { _vhaddq_u16(a, b) }
}
///Halving add
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vhadd_u32)
pub fn vhadd_u32(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
    { _vhadd_u32(a, b) }
}
///Halving add
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vhaddq_u32)
pub fn vhaddq_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
    { _vhaddq_u32(a, b) }
}
///Signed halving subtract
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vhsub_s16)
pub fn vhsub_s16(a: int16x4_t, b: int16x4_t) -> int16x4_t {
    { _vhsub_s16(a, b) }
}
///Signed halving subtract
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vhsubq_s16)
pub fn vhsubq_s16(a: int16x8_t, b: int16x8_t) -> int16x8_t {
    { _vhsubq_s16(a, b) }
}
///Signed halving subtract
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vhsub_s32)
pub fn vhsub_s32(a: int32x2_t, b: int32x2_t) -> int32x2_t {
    { _vhsub_s32(a, b) }
}
///Signed halving subtract
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vhsubq_s32)
pub fn vhsubq_s32(a: int32x4_t, b: int32x4_t) -> int32x4_t {
    { _vhsubq_s32(a, b) }
}
///Signed halving subtract
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vhsub_s8)
pub fn vhsub_s8(a: int8x8_t, b: int8x8_t) -> int8x8_t {
    { _vhsub_s8(a, b) }
}
///Signed halving subtract
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vhsubq_s8)
pub fn vhsubq_s8(a: int8x16_t, b: int8x16_t) -> int8x16_t {
    { _vhsubq_s8(a, b) }
}
///Signed halving subtract
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vhsub_u8)
pub fn vhsub_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
    { _vhsub_u8(a, b) }
}
///Signed halving subtract
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vhsubq_u8)
pub fn vhsubq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
    { _vhsubq_u8(a, b) }
}
///Signed halving subtract
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vhsub_u16)
pub fn vhsub_u16(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
    { _vhsub_u16(a, b) }
}
///Signed halving subtract
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vhsubq_u16)
pub fn vhsubq_u16(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
    { _vhsubq_u16(a, b) }
}
///Signed halving subtract
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vhsub_u32)
pub fn vhsub_u32(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
    { _vhsub_u32(a, b) }
}
///Signed halving subtract
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vhsubq_u32)
pub fn vhsubq_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
    { _vhsubq_u32(a, b) }
}
///Maximum (vector)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmax_f16)
pub fn vmax_f16(a: float16x4_t, b: float16x4_t) -> float16x4_t {
    { _vmax_f16(a, b) }
}
///Maximum (vector)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmaxq_f16)
pub fn vmaxq_f16(a: float16x8_t, b: float16x8_t) -> float16x8_t {
    { _vmaxq_f16(a, b) }
}
///Maximum (vector)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmax_f32)
pub fn vmax_f32(a: float32x2_t, b: float32x2_t) -> float32x2_t {
    { _vmax_f32(a, b) }
}
///Maximum (vector)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmaxq_f32)
pub fn vmaxq_f32(a: float32x4_t, b: float32x4_t) -> float32x4_t {
    { _vmaxq_f32(a, b) }
}
///Maximum (vector)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmax_s8)
pub fn vmax_s8(a: int8x8_t, b: int8x8_t) -> int8x8_t {
    { _vmax_s8(a, b) }
}
///Maximum (vector)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmaxq_s8)
pub fn vmaxq_s8(a: int8x16_t, b: int8x16_t) -> int8x16_t {
    { _vmaxq_s8(a, b) }
}
///Maximum (vector)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmax_s16)
pub fn vmax_s16(a: int16x4_t, b: int16x4_t) -> int16x4_t {
    { _vmax_s16(a, b) }
}
///Maximum (vector)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmaxq_s16)
pub fn vmaxq_s16(a: int16x8_t, b: int16x8_t) -> int16x8_t {
    { _vmaxq_s16(a, b) }
}
///Maximum (vector)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmax_s32)
pub fn vmax_s32(a: int32x2_t, b: int32x2_t) -> int32x2_t {
    { _vmax_s32(a, b) }
}
///Maximum (vector)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmaxq_s32)
pub fn vmaxq_s32(a: int32x4_t, b: int32x4_t) -> int32x4_t {
    { _vmaxq_s32(a, b) }
}
///Maximum (vector)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmax_u8)
pub fn vmax_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
    { _vmax_u8(a, b) }
}
///Maximum (vector)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmaxq_u8)
pub fn vmaxq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
    { _vmaxq_u8(a, b) }
}
///Maximum (vector)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmax_u16)
pub fn vmax_u16(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
    { _vmax_u16(a, b) }
}
///Maximum (vector)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmaxq_u16)
pub fn vmaxq_u16(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
    { _vmaxq_u16(a, b) }
}
///Maximum (vector)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmax_u32)
pub fn vmax_u32(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
    { _vmax_u32(a, b) }
}
///Maximum (vector)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmaxq_u32)
pub fn vmaxq_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
    { _vmaxq_u32(a, b) }
}
///Floating-point Maximum Number (vector)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmaxnm_f16)
pub fn vmaxnm_f16(a: float16x4_t, b: float16x4_t) -> float16x4_t {
    { _vmaxnm_f16(a, b) }
}
///Floating-point Maximum Number (vector)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmaxnmq_f16)
pub fn vmaxnmq_f16(a: float16x8_t, b: float16x8_t) -> float16x8_t {
    { _vmaxnmq_f16(a, b) }
}
///Floating-point Maximum Number (vector)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmaxnm_f32)
pub fn vmaxnm_f32(a: float32x2_t, b: float32x2_t) -> float32x2_t {
    { _vmaxnm_f32(a, b) }
}
///Floating-point Maximum Number (vector)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmaxnmq_f32)
pub fn vmaxnmq_f32(a: float32x4_t, b: float32x4_t) -> float32x4_t {
    { _vmaxnmq_f32(a, b) }
}
///Minimum (vector)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmin_f16)
pub fn vmin_f16(a: float16x4_t, b: float16x4_t) -> float16x4_t {
    { _vmin_f16(a, b) }
}
///Minimum (vector)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vminq_f16)
pub fn vminq_f16(a: float16x8_t, b: float16x8_t) -> float16x8_t {
    { _vminq_f16(a, b) }
}
///Minimum (vector)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmin_f32)
pub fn vmin_f32(a: float32x2_t, b: float32x2_t) -> float32x2_t {
    { _vmin_f32(a, b) }
}
///Minimum (vector)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vminq_f32)
pub fn vminq_f32(a: float32x4_t, b: float32x4_t) -> float32x4_t {
    { _vminq_f32(a, b) }
}
///Minimum (vector)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmin_s8)
pub fn vmin_s8(a: int8x8_t, b: int8x8_t) -> int8x8_t {
    { _vmin_s8(a, b) }
}
///Minimum (vector)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vminq_s8)
pub fn vminq_s8(a: int8x16_t, b: int8x16_t) -> int8x16_t {
    { _vminq_s8(a, b) }
}
///Minimum (vector)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmin_s16)
pub fn vmin_s16(a: int16x4_t, b: int16x4_t) -> int16x4_t {
    { _vmin_s16(a, b) }
}
///Minimum (vector)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vminq_s16)
pub fn vminq_s16(a: int16x8_t, b: int16x8_t) -> int16x8_t {
    { _vminq_s16(a, b) }
}
///Minimum (vector)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmin_s32)
pub fn vmin_s32(a: int32x2_t, b: int32x2_t) -> int32x2_t {
    { _vmin_s32(a, b) }
}
///Minimum (vector)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vminq_s32)
pub fn vminq_s32(a: int32x4_t, b: int32x4_t) -> int32x4_t {
    { _vminq_s32(a, b) }
}
///Minimum (vector)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmin_u8)
pub fn vmin_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
    { _vmin_u8(a, b) }
}
///Minimum (vector)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vminq_u8)
pub fn vminq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
    { _vminq_u8(a, b) }
}
///Minimum (vector)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmin_u16)
pub fn vmin_u16(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
    { _vmin_u16(a, b) }
}
///Minimum (vector)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vminq_u16)
pub fn vminq_u16(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
    { _vminq_u16(a, b) }
}
///Minimum (vector)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmin_u32)
pub fn vmin_u32(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
    { _vmin_u32(a, b) }
}
///Minimum (vector)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vminq_u32)
pub fn vminq_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
    { _vminq_u32(a, b) }
}
///Floating-point Minimum Number (vector)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vminnm_f16)
pub fn vminnm_f16(a: float16x4_t, b: float16x4_t) -> float16x4_t {
    { _vminnm_f16(a, b) }
}
///Floating-point Minimum Number (vector)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vminnmq_f16)
pub fn vminnmq_f16(a: float16x8_t, b: float16x8_t) -> float16x8_t {
    { _vminnmq_f16(a, b) }
}
///Floating-point Minimum Number (vector)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vminnm_f32)
pub fn vminnm_f32(a: float32x2_t, b: float32x2_t) -> float32x2_t {
    { _vminnm_f32(a, b) }
}
///Floating-point Minimum Number (vector)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vminnmq_f32)
pub fn vminnmq_f32(a: float32x4_t, b: float32x4_t) -> float32x4_t {
    { _vminnmq_f32(a, b) }
}
///Floating-point multiply-add to accumulator
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmla_f32)
pub fn vmla_f32(a: float32x2_t, b: float32x2_t, c: float32x2_t) -> float32x2_t {
    { simd_add(a, simd_mul(b, c)) }
}
///Floating-point multiply-add to accumulator
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmlaq_f32)
pub fn vmlaq_f32(a: float32x4_t, b: float32x4_t, c: float32x4_t) -> float32x4_t {
    { simd_add(a, simd_mul(b, c)) }
}
///Vector multiply accumulate with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmla_lane_f32)
pub fn vmla_lane_f32<const LANE: i32>(
    a: float32x2_t,
    b: float32x2_t,
    c: float32x2_t,
) -> float32x2_t {
    static_assert_uimm_bits!(LANE, 1);
    { vmla_f32(a, b, simd_shuffle(c, c, [LANE as u32, LANE as u32])) }
}
///Vector multiply accumulate with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmla_laneq_f32)
pub fn vmla_laneq_f32<const LANE: i32>(
    a: float32x2_t,
    b: float32x2_t,
    c: float32x4_t,
) -> float32x2_t {
    static_assert_uimm_bits!(LANE, 2);
    { vmla_f32(a, b, simd_shuffle(c, c, [LANE as u32, LANE as u32])) }
}
///Vector multiply accumulate with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmlaq_lane_f32)
pub fn vmlaq_lane_f32<const LANE: i32>(
    a: float32x4_t,
    b: float32x4_t,
    c: float32x2_t,
) -> float32x4_t {
    static_assert_uimm_bits!(LANE, 1);
    {
        vmlaq_f32(
            a,
            b,
            simd_shuffle(c, c, [LANE as u32, LANE as u32, LANE as u32, LANE as u32]),
        )
    }
}
///Vector multiply accumulate with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmlaq_laneq_f32)
pub fn vmlaq_laneq_f32<const LANE: i32>(
    a: float32x4_t,
    b: float32x4_t,
    c: float32x4_t,
) -> float32x4_t {
    static_assert_uimm_bits!(LANE, 2);
    {
        vmlaq_f32(
            a,
            b,
            simd_shuffle(c, c, [LANE as u32, LANE as u32, LANE as u32, LANE as u32]),
        )
    }
}
///Vector multiply accumulate with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmla_lane_s16)
pub fn vmla_lane_s16<const LANE: i32>(
    a: int16x4_t,
    b: int16x4_t,
    c: int16x4_t,
) -> int16x4_t {
    static_assert_uimm_bits!(LANE, 2);
    {
        vmla_s16(
            a,
            b,
            simd_shuffle(c, c, [LANE as u32, LANE as u32, LANE as u32, LANE as u32]),
        )
    }
}
///Vector multiply accumulate with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmla_lane_u16)
pub fn vmla_lane_u16<const LANE: i32>(
    a: uint16x4_t,
    b: uint16x4_t,
    c: uint16x4_t,
) -> uint16x4_t {
    static_assert_uimm_bits!(LANE, 2);
    {
        vmla_u16(
            a,
            b,
            simd_shuffle(c, c, [LANE as u32, LANE as u32, LANE as u32, LANE as u32]),
        )
    }
}
///Vector multiply accumulate with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmla_laneq_s16)
pub fn vmla_laneq_s16<const LANE: i32>(
    a: int16x4_t,
    b: int16x4_t,
    c: int16x8_t,
) -> int16x4_t {
    static_assert_uimm_bits!(LANE, 3);
    {
        vmla_s16(
            a,
            b,
            simd_shuffle(c, c, [LANE as u32, LANE as u32, LANE as u32, LANE as u32]),
        )
    }
}
///Vector multiply accumulate with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmla_laneq_u16)
pub fn vmla_laneq_u16<const LANE: i32>(
    a: uint16x4_t,
    b: uint16x4_t,
    c: uint16x8_t,
) -> uint16x4_t {
    static_assert_uimm_bits!(LANE, 3);
    {
        vmla_u16(
            a,
            b,
            simd_shuffle(c, c, [LANE as u32, LANE as u32, LANE as u32, LANE as u32]),
        )
    }
}
///Vector multiply accumulate with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmlaq_lane_s16)
pub fn vmlaq_lane_s16<const LANE: i32>(
    a: int16x8_t,
    b: int16x8_t,
    c: int16x4_t,
) -> int16x8_t {
    static_assert_uimm_bits!(LANE, 2);
    {
        vmlaq_s16(
            a,
            b,
            simd_shuffle(
                c, c, [LANE as u32, LANE as u32, LANE as u32, LANE as u32, LANE as u32,
                LANE as u32, LANE as u32, LANE as u32]
            ),
        )
    }
}
///Vector multiply accumulate with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmlaq_lane_u16)
pub fn vmlaq_lane_u16<const LANE: i32>(
    a: uint16x8_t,
    b: uint16x8_t,
    c: uint16x4_t,
) -> uint16x8_t {
    static_assert_uimm_bits!(LANE, 2);
    {
        vmlaq_u16(
            a,
            b,
            simd_shuffle(
                c, c, [LANE as u32, LANE as u32, LANE as u32, LANE as u32, LANE as u32,
                LANE as u32, LANE as u32, LANE as u32]
            ),
        )
    }
}
///Vector multiply accumulate with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmlaq_laneq_s16)
pub fn vmlaq_laneq_s16<const LANE: i32>(
    a: int16x8_t,
    b: int16x8_t,
    c: int16x8_t,
) -> int16x8_t {
    static_assert_uimm_bits!(LANE, 3);
    {
        vmlaq_s16(
            a,
            b,
            simd_shuffle(
                c, c, [LANE as u32, LANE as u32, LANE as u32, LANE as u32, LANE as u32,
                LANE as u32, LANE as u32, LANE as u32]
            ),
        )
    }
}
///Vector multiply accumulate with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmlaq_laneq_u16)
pub fn vmlaq_laneq_u16<const LANE: i32>(
    a: uint16x8_t,
    b: uint16x8_t,
    c: uint16x8_t,
) -> uint16x8_t {
    static_assert_uimm_bits!(LANE, 3);
    {
        vmlaq_u16(
            a,
            b,
            simd_shuffle(
                c, c, [LANE as u32, LANE as u32, LANE as u32, LANE as u32, LANE as u32,
                LANE as u32, LANE as u32, LANE as u32]
            ),
        )
    }
}
///Vector multiply accumulate with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmla_lane_s32)
pub fn vmla_lane_s32<const LANE: i32>(
    a: int32x2_t,
    b: int32x2_t,
    c: int32x2_t,
) -> int32x2_t {
    static_assert_uimm_bits!(LANE, 1);
    { vmla_s32(a, b, simd_shuffle(c, c, [LANE as u32, LANE as u32])) }
}
///Vector multiply accumulate with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmla_lane_u32)
pub fn vmla_lane_u32<const LANE: i32>(
    a: uint32x2_t,
    b: uint32x2_t,
    c: uint32x2_t,
) -> uint32x2_t {
    static_assert_uimm_bits!(LANE, 1);
    { vmla_u32(a, b, simd_shuffle(c, c, [LANE as u32, LANE as u32])) }
}
///Vector multiply accumulate with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmla_laneq_s32)
pub fn vmla_laneq_s32<const LANE: i32>(
    a: int32x2_t,
    b: int32x2_t,
    c: int32x4_t,
) -> int32x2_t {
    static_assert_uimm_bits!(LANE, 2);
    { vmla_s32(a, b, simd_shuffle(c, c, [LANE as u32, LANE as u32])) }
}
///Vector multiply accumulate with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmla_laneq_u32)
pub fn vmla_laneq_u32<const LANE: i32>(
    a: uint32x2_t,
    b: uint32x2_t,
    c: uint32x4_t,
) -> uint32x2_t {
    static_assert_uimm_bits!(LANE, 2);
    { vmla_u32(a, b, simd_shuffle(c, c, [LANE as u32, LANE as u32])) }
}
///Vector multiply accumulate with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmlaq_lane_s32)
pub fn vmlaq_lane_s32<const LANE: i32>(
    a: int32x4_t,
    b: int32x4_t,
    c: int32x2_t,
) -> int32x4_t {
    static_assert_uimm_bits!(LANE, 1);
    {
        vmlaq_s32(
            a,
            b,
            simd_shuffle(c, c, [LANE as u32, LANE as u32, LANE as u32, LANE as u32]),
        )
    }
}
///Vector multiply accumulate with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmlaq_lane_u32)
pub fn vmlaq_lane_u32<const LANE: i32>(
    a: uint32x4_t,
    b: uint32x4_t,
    c: uint32x2_t,
) -> uint32x4_t {
    static_assert_uimm_bits!(LANE, 1);
    {
        vmlaq_u32(
            a,
            b,
            simd_shuffle(c, c, [LANE as u32, LANE as u32, LANE as u32, LANE as u32]),
        )
    }
}
///Vector multiply accumulate with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmlaq_laneq_s32)
pub fn vmlaq_laneq_s32<const LANE: i32>(
    a: int32x4_t,
    b: int32x4_t,
    c: int32x4_t,
) -> int32x4_t {
    static_assert_uimm_bits!(LANE, 2);
    {
        vmlaq_s32(
            a,
            b,
            simd_shuffle(c, c, [LANE as u32, LANE as u32, LANE as u32, LANE as u32]),
        )
    }
}
///Vector multiply accumulate with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmlaq_laneq_u32)
pub fn vmlaq_laneq_u32<const LANE: i32>(
    a: uint32x4_t,
    b: uint32x4_t,
    c: uint32x4_t,
) -> uint32x4_t {
    static_assert_uimm_bits!(LANE, 2);
    {
        vmlaq_u32(
            a,
            b,
            simd_shuffle(c, c, [LANE as u32, LANE as u32, LANE as u32, LANE as u32]),
        )
    }
}
///Vector multiply accumulate with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmla_n_f32)
pub fn vmla_n_f32(a: float32x2_t, b: float32x2_t, c: f32) -> float32x2_t {
    vmla_f32(a, b, vdup_n_f32(c))
}
///Vector multiply accumulate with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmlaq_n_f32)
pub fn vmlaq_n_f32(a: float32x4_t, b: float32x4_t, c: f32) -> float32x4_t {
    vmlaq_f32(a, b, vdupq_n_f32(c))
}
///Vector multiply accumulate with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmla_n_s16)
pub fn vmla_n_s16(a: int16x4_t, b: int16x4_t, c: i16) -> int16x4_t {
    vmla_s16(a, b, vdup_n_s16(c))
}
///Vector multiply accumulate with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmlaq_n_s16)
pub fn vmlaq_n_s16(a: int16x8_t, b: int16x8_t, c: i16) -> int16x8_t {
    vmlaq_s16(a, b, vdupq_n_s16(c))
}
///Vector multiply accumulate with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmla_n_u16)
pub fn vmla_n_u16(a: uint16x4_t, b: uint16x4_t, c: u16) -> uint16x4_t {
    vmla_u16(a, b, vdup_n_u16(c))
}
///Vector multiply accumulate with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmlaq_n_u16)
pub fn vmlaq_n_u16(a: uint16x8_t, b: uint16x8_t, c: u16) -> uint16x8_t {
    vmlaq_u16(a, b, vdupq_n_u16(c))
}
///Vector multiply accumulate with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmla_n_s32)
pub fn vmla_n_s32(a: int32x2_t, b: int32x2_t, c: i32) -> int32x2_t {
    vmla_s32(a, b, vdup_n_s32(c))
}
///Vector multiply accumulate with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmlaq_n_s32)
pub fn vmlaq_n_s32(a: int32x4_t, b: int32x4_t, c: i32) -> int32x4_t {
    vmlaq_s32(a, b, vdupq_n_s32(c))
}
///Vector multiply accumulate with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmla_n_u32)
pub fn vmla_n_u32(a: uint32x2_t, b: uint32x2_t, c: u32) -> uint32x2_t {
    vmla_u32(a, b, vdup_n_u32(c))
}
///Vector multiply accumulate with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmlaq_n_u32)
pub fn vmlaq_n_u32(a: uint32x4_t, b: uint32x4_t, c: u32) -> uint32x4_t {
    vmlaq_u32(a, b, vdupq_n_u32(c))
}
///Multiply-add to accumulator
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmla_s8)
pub fn vmla_s8(a: int8x8_t, b: int8x8_t, c: int8x8_t) -> int8x8_t {
    { simd_add(a, simd_mul(b, c)) }
}
///Multiply-add to accumulator
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmlaq_s8)
pub fn vmlaq_s8(a: int8x16_t, b: int8x16_t, c: int8x16_t) -> int8x16_t {
    { simd_add(a, simd_mul(b, c)) }
}
///Multiply-add to accumulator
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmla_s16)
pub fn vmla_s16(a: int16x4_t, b: int16x4_t, c: int16x4_t) -> int16x4_t {
    { simd_add(a, simd_mul(b, c)) }
}
///Multiply-add to accumulator
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmlaq_s16)
pub fn vmlaq_s16(a: int16x8_t, b: int16x8_t, c: int16x8_t) -> int16x8_t {
    { simd_add(a, simd_mul(b, c)) }
}
///Multiply-add to accumulator
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmla_s32)
pub fn vmla_s32(a: int32x2_t, b: int32x2_t, c: int32x2_t) -> int32x2_t {
    { simd_add(a, simd_mul(b, c)) }
}
///Multiply-add to accumulator
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmlaq_s32)
pub fn vmlaq_s32(a: int32x4_t, b: int32x4_t, c: int32x4_t) -> int32x4_t {
    { simd_add(a, simd_mul(b, c)) }
}
///Multiply-add to accumulator
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmla_u8)
pub fn vmla_u8(a: uint8x8_t, b: uint8x8_t, c: uint8x8_t) -> uint8x8_t {
    { simd_add(a, simd_mul(b, c)) }
}
///Multiply-add to accumulator
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmlaq_u8)
pub fn vmlaq_u8(a: uint8x16_t, b: uint8x16_t, c: uint8x16_t) -> uint8x16_t {
    { simd_add(a, simd_mul(b, c)) }
}
///Multiply-add to accumulator
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmla_u16)
pub fn vmla_u16(a: uint16x4_t, b: uint16x4_t, c: uint16x4_t) -> uint16x4_t {
    { simd_add(a, simd_mul(b, c)) }
}
///Multiply-add to accumulator
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmlaq_u16)
pub fn vmlaq_u16(a: uint16x8_t, b: uint16x8_t, c: uint16x8_t) -> uint16x8_t {
    { simd_add(a, simd_mul(b, c)) }
}
///Multiply-add to accumulator
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmla_u32)
pub fn vmla_u32(a: uint32x2_t, b: uint32x2_t, c: uint32x2_t) -> uint32x2_t {
    { simd_add(a, simd_mul(b, c)) }
}
///Multiply-add to accumulator
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmlaq_u32)
pub fn vmlaq_u32(a: uint32x4_t, b: uint32x4_t, c: uint32x4_t) -> uint32x4_t {
    { simd_add(a, simd_mul(b, c)) }
}
///Vector widening multiply accumulate with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmlal_lane_s16)
pub fn vmlal_lane_s16<const LANE: i32>(
    a: int32x4_t,
    b: int16x4_t,
    c: int16x4_t,
) -> int32x4_t {
    static_assert_uimm_bits!(LANE, 2);
    {
        vmlal_s16(
            a,
            b,
            simd_shuffle(c, c, [LANE as u32, LANE as u32, LANE as u32, LANE as u32]),
        )
    }
}
///Vector widening multiply accumulate with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmlal_laneq_s16)
pub fn vmlal_laneq_s16<const LANE: i32>(
    a: int32x4_t,
    b: int16x4_t,
    c: int16x8_t,
) -> int32x4_t {
    static_assert_uimm_bits!(LANE, 3);
    {
        vmlal_s16(
            a,
            b,
            simd_shuffle(c, c, [LANE as u32, LANE as u32, LANE as u32, LANE as u32]),
        )
    }
}
///Vector widening multiply accumulate with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmlal_lane_s32)
pub fn vmlal_lane_s32<const LANE: i32>(
    a: int64x2_t,
    b: int32x2_t,
    c: int32x2_t,
) -> int64x2_t {
    static_assert_uimm_bits!(LANE, 1);
    { vmlal_s32(a, b, simd_shuffle(c, c, [LANE as u32, LANE as u32])) }
}
///Vector widening multiply accumulate with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmlal_laneq_s32)
pub fn vmlal_laneq_s32<const LANE: i32>(
    a: int64x2_t,
    b: int32x2_t,
    c: int32x4_t,
) -> int64x2_t {
    static_assert_uimm_bits!(LANE, 2);
    { vmlal_s32(a, b, simd_shuffle(c, c, [LANE as u32, LANE as u32])) }
}
///Vector widening multiply accumulate with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmlal_lane_u16)
pub fn vmlal_lane_u16<const LANE: i32>(
    a: uint32x4_t,
    b: uint16x4_t,
    c: uint16x4_t,
) -> uint32x4_t {
    static_assert_uimm_bits!(LANE, 2);
    {
        vmlal_u16(
            a,
            b,
            simd_shuffle(c, c, [LANE as u32, LANE as u32, LANE as u32, LANE as u32]),
        )
    }
}
///Vector widening multiply accumulate with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmlal_laneq_u16)
pub fn vmlal_laneq_u16<const LANE: i32>(
    a: uint32x4_t,
    b: uint16x4_t,
    c: uint16x8_t,
) -> uint32x4_t {
    static_assert_uimm_bits!(LANE, 3);
    {
        vmlal_u16(
            a,
            b,
            simd_shuffle(c, c, [LANE as u32, LANE as u32, LANE as u32, LANE as u32]),
        )
    }
}
///Vector widening multiply accumulate with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmlal_lane_u32)
pub fn vmlal_lane_u32<const LANE: i32>(
    a: uint64x2_t,
    b: uint32x2_t,
    c: uint32x2_t,
) -> uint64x2_t {
    static_assert_uimm_bits!(LANE, 1);
    { vmlal_u32(a, b, simd_shuffle(c, c, [LANE as u32, LANE as u32])) }
}
///Vector widening multiply accumulate with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmlal_laneq_u32)
pub fn vmlal_laneq_u32<const LANE: i32>(
    a: uint64x2_t,
    b: uint32x2_t,
    c: uint32x4_t,
) -> uint64x2_t {
    static_assert_uimm_bits!(LANE, 2);
    { vmlal_u32(a, b, simd_shuffle(c, c, [LANE as u32, LANE as u32])) }
}
///Vector widening multiply accumulate with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmlal_n_s16)
pub fn vmlal_n_s16(a: int32x4_t, b: int16x4_t, c: i16) -> int32x4_t {
    vmlal_s16(a, b, vdup_n_s16(c))
}
///Vector widening multiply accumulate with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmlal_n_s32)
pub fn vmlal_n_s32(a: int64x2_t, b: int32x2_t, c: i32) -> int64x2_t {
    vmlal_s32(a, b, vdup_n_s32(c))
}
///Vector widening multiply accumulate with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmlal_n_u16)
pub fn vmlal_n_u16(a: uint32x4_t, b: uint16x4_t, c: u16) -> uint32x4_t {
    vmlal_u16(a, b, vdup_n_u16(c))
}
///Vector widening multiply accumulate with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmlal_n_u32)
pub fn vmlal_n_u32(a: uint64x2_t, b: uint32x2_t, c: u32) -> uint64x2_t {
    vmlal_u32(a, b, vdup_n_u32(c))
}
///Signed multiply-add long
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmlal_s8)
pub fn vmlal_s8(a: int16x8_t, b: int8x8_t, c: int8x8_t) -> int16x8_t {
    { simd_add(a, vmull_s8(b, c)) }
}
///Signed multiply-add long
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmlal_s16)
pub fn vmlal_s16(a: int32x4_t, b: int16x4_t, c: int16x4_t) -> int32x4_t {
    { simd_add(a, vmull_s16(b, c)) }
}
///Signed multiply-add long
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmlal_s32)
pub fn vmlal_s32(a: int64x2_t, b: int32x2_t, c: int32x2_t) -> int64x2_t {
    { simd_add(a, vmull_s32(b, c)) }
}
///Unsigned multiply-add long
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmlal_u8)
pub fn vmlal_u8(a: uint16x8_t, b: uint8x8_t, c: uint8x8_t) -> uint16x8_t {
    { simd_add(a, vmull_u8(b, c)) }
}
///Unsigned multiply-add long
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmlal_u16)
pub fn vmlal_u16(a: uint32x4_t, b: uint16x4_t, c: uint16x4_t) -> uint32x4_t {
    { simd_add(a, vmull_u16(b, c)) }
}
///Unsigned multiply-add long
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmlal_u32)
pub fn vmlal_u32(a: uint64x2_t, b: uint32x2_t, c: uint32x2_t) -> uint64x2_t {
    { simd_add(a, vmull_u32(b, c)) }
}
///Floating-point multiply-subtract from accumulator
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmls_f32)
pub fn vmls_f32(a: float32x2_t, b: float32x2_t, c: float32x2_t) -> float32x2_t {
    { simd_sub(a, simd_mul(b, c)) }
}
///Floating-point multiply-subtract from accumulator
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmlsq_f32)
pub fn vmlsq_f32(a: float32x4_t, b: float32x4_t, c: float32x4_t) -> float32x4_t {
    { simd_sub(a, simd_mul(b, c)) }
}
///Vector multiply subtract with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmls_lane_f32)
pub fn vmls_lane_f32<const LANE: i32>(
    a: float32x2_t,
    b: float32x2_t,
    c: float32x2_t,
) -> float32x2_t {
    static_assert_uimm_bits!(LANE, 1);
    { vmls_f32(a, b, simd_shuffle(c, c, [LANE as u32, LANE as u32])) }
}
///Vector multiply subtract with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmls_laneq_f32)
pub fn vmls_laneq_f32<const LANE: i32>(
    a: float32x2_t,
    b: float32x2_t,
    c: float32x4_t,
) -> float32x2_t {
    static_assert_uimm_bits!(LANE, 2);
    { vmls_f32(a, b, simd_shuffle(c, c, [LANE as u32, LANE as u32])) }
}
///Vector multiply subtract with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmlsq_lane_f32)
pub fn vmlsq_lane_f32<const LANE: i32>(
    a: float32x4_t,
    b: float32x4_t,
    c: float32x2_t,
) -> float32x4_t {
    static_assert_uimm_bits!(LANE, 1);
    {
        vmlsq_f32(
            a,
            b,
            simd_shuffle(c, c, [LANE as u32, LANE as u32, LANE as u32, LANE as u32]),
        )
    }
}
///Vector multiply subtract with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmlsq_laneq_f32)
pub fn vmlsq_laneq_f32<const LANE: i32>(
    a: float32x4_t,
    b: float32x4_t,
    c: float32x4_t,
) -> float32x4_t {
    static_assert_uimm_bits!(LANE, 2);
    {
        vmlsq_f32(
            a,
            b,
            simd_shuffle(c, c, [LANE as u32, LANE as u32, LANE as u32, LANE as u32]),
        )
    }
}
///Vector multiply subtract with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmls_lane_s16)
pub fn vmls_lane_s16<const LANE: i32>(
    a: int16x4_t,
    b: int16x4_t,
    c: int16x4_t,
) -> int16x4_t {
    static_assert_uimm_bits!(LANE, 2);
    {
        vmls_s16(
            a,
            b,
            simd_shuffle(c, c, [LANE as u32, LANE as u32, LANE as u32, LANE as u32]),
        )
    }
}
///Vector multiply subtract with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmls_lane_u16)
pub fn vmls_lane_u16<const LANE: i32>(
    a: uint16x4_t,
    b: uint16x4_t,
    c: uint16x4_t,
) -> uint16x4_t {
    static_assert_uimm_bits!(LANE, 2);
    {
        vmls_u16(
            a,
            b,
            simd_shuffle(c, c, [LANE as u32, LANE as u32, LANE as u32, LANE as u32]),
        )
    }
}
///Vector multiply subtract with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmls_laneq_s16)
pub fn vmls_laneq_s16<const LANE: i32>(
    a: int16x4_t,
    b: int16x4_t,
    c: int16x8_t,
) -> int16x4_t {
    static_assert_uimm_bits!(LANE, 3);
    {
        vmls_s16(
            a,
            b,
            simd_shuffle(c, c, [LANE as u32, LANE as u32, LANE as u32, LANE as u32]),
        )
    }
}
///Vector multiply subtract with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmls_laneq_u16)
pub fn vmls_laneq_u16<const LANE: i32>(
    a: uint16x4_t,
    b: uint16x4_t,
    c: uint16x8_t,
) -> uint16x4_t {
    static_assert_uimm_bits!(LANE, 3);
    {
        vmls_u16(
            a,
            b,
            simd_shuffle(c, c, [LANE as u32, LANE as u32, LANE as u32, LANE as u32]),
        )
    }
}
///Vector multiply subtract with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmlsq_lane_s16)
pub fn vmlsq_lane_s16<const LANE: i32>(
    a: int16x8_t,
    b: int16x8_t,
    c: int16x4_t,
) -> int16x8_t {
    static_assert_uimm_bits!(LANE, 2);
    {
        vmlsq_s16(
            a,
            b,
            simd_shuffle(
                c, c, [LANE as u32, LANE as u32, LANE as u32, LANE as u32, LANE as u32,
                LANE as u32, LANE as u32, LANE as u32]
            ),
        )
    }
}
///Vector multiply subtract with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmlsq_lane_u16)
pub fn vmlsq_lane_u16<const LANE: i32>(
    a: uint16x8_t,
    b: uint16x8_t,
    c: uint16x4_t,
) -> uint16x8_t {
    static_assert_uimm_bits!(LANE, 2);
    {
        vmlsq_u16(
            a,
            b,
            simd_shuffle(
                c, c, [LANE as u32, LANE as u32, LANE as u32, LANE as u32, LANE as u32,
                LANE as u32, LANE as u32, LANE as u32]
            ),
        )
    }
}
///Vector multiply subtract with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmlsq_laneq_s16)
pub fn vmlsq_laneq_s16<const LANE: i32>(
    a: int16x8_t,
    b: int16x8_t,
    c: int16x8_t,
) -> int16x8_t {
    static_assert_uimm_bits!(LANE, 3);
    {
        vmlsq_s16(
            a,
            b,
            simd_shuffle(
                c, c, [LANE as u32, LANE as u32, LANE as u32, LANE as u32, LANE as u32,
                LANE as u32, LANE as u32, LANE as u32]
            ),
        )
    }
}
///Vector multiply subtract with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmlsq_laneq_u16)
pub fn vmlsq_laneq_u16<const LANE: i32>(
    a: uint16x8_t,
    b: uint16x8_t,
    c: uint16x8_t,
) -> uint16x8_t {
    static_assert_uimm_bits!(LANE, 3);
    {
        vmlsq_u16(
            a,
            b,
            simd_shuffle(
                c, c, [LANE as u32, LANE as u32, LANE as u32, LANE as u32, LANE as u32,
                LANE as u32, LANE as u32, LANE as u32]
            ),
        )
    }
}
///Vector multiply subtract with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmls_lane_s32)
pub fn vmls_lane_s32<const LANE: i32>(
    a: int32x2_t,
    b: int32x2_t,
    c: int32x2_t,
) -> int32x2_t {
    static_assert_uimm_bits!(LANE, 1);
    { vmls_s32(a, b, simd_shuffle(c, c, [LANE as u32, LANE as u32])) }
}
///Vector multiply subtract with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmls_lane_u32)
pub fn vmls_lane_u32<const LANE: i32>(
    a: uint32x2_t,
    b: uint32x2_t,
    c: uint32x2_t,
) -> uint32x2_t {
    static_assert_uimm_bits!(LANE, 1);
    { vmls_u32(a, b, simd_shuffle(c, c, [LANE as u32, LANE as u32])) }
}
///Vector multiply subtract with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmls_laneq_s32)
pub fn vmls_laneq_s32<const LANE: i32>(
    a: int32x2_t,
    b: int32x2_t,
    c: int32x4_t,
) -> int32x2_t {
    static_assert_uimm_bits!(LANE, 2);
    { vmls_s32(a, b, simd_shuffle(c, c, [LANE as u32, LANE as u32])) }
}
///Vector multiply subtract with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmls_laneq_u32)
pub fn vmls_laneq_u32<const LANE: i32>(
    a: uint32x2_t,
    b: uint32x2_t,
    c: uint32x4_t,
) -> uint32x2_t {
    static_assert_uimm_bits!(LANE, 2);
    { vmls_u32(a, b, simd_shuffle(c, c, [LANE as u32, LANE as u32])) }
}
///Vector multiply subtract with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmlsq_lane_s32)
pub fn vmlsq_lane_s32<const LANE: i32>(
    a: int32x4_t,
    b: int32x4_t,
    c: int32x2_t,
) -> int32x4_t {
    static_assert_uimm_bits!(LANE, 1);
    {
        vmlsq_s32(
            a,
            b,
            simd_shuffle(c, c, [LANE as u32, LANE as u32, LANE as u32, LANE as u32]),
        )
    }
}
///Vector multiply subtract with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmlsq_lane_u32)
pub fn vmlsq_lane_u32<const LANE: i32>(
    a: uint32x4_t,
    b: uint32x4_t,
    c: uint32x2_t,
) -> uint32x4_t {
    static_assert_uimm_bits!(LANE, 1);
    {
        vmlsq_u32(
            a,
            b,
            simd_shuffle(c, c, [LANE as u32, LANE as u32, LANE as u32, LANE as u32]),
        )
    }
}
///Vector multiply subtract with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmlsq_laneq_s32)
pub fn vmlsq_laneq_s32<const LANE: i32>(
    a: int32x4_t,
    b: int32x4_t,
    c: int32x4_t,
) -> int32x4_t {
    static_assert_uimm_bits!(LANE, 2);
    {
        vmlsq_s32(
            a,
            b,
            simd_shuffle(c, c, [LANE as u32, LANE as u32, LANE as u32, LANE as u32]),
        )
    }
}
///Vector multiply subtract with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmlsq_laneq_u32)
pub fn vmlsq_laneq_u32<const LANE: i32>(
    a: uint32x4_t,
    b: uint32x4_t,
    c: uint32x4_t,
) -> uint32x4_t {
    static_assert_uimm_bits!(LANE, 2);
    {
        vmlsq_u32(
            a,
            b,
            simd_shuffle(c, c, [LANE as u32, LANE as u32, LANE as u32, LANE as u32]),
        )
    }
}
///Vector multiply subtract with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmls_n_f32)
pub fn vmls_n_f32(a: float32x2_t, b: float32x2_t, c: f32) -> float32x2_t {
    vmls_f32(a, b, vdup_n_f32(c))
}
///Vector multiply subtract with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmlsq_n_f32)
pub fn vmlsq_n_f32(a: float32x4_t, b: float32x4_t, c: f32) -> float32x4_t {
    vmlsq_f32(a, b, vdupq_n_f32(c))
}
///Vector multiply subtract with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmls_n_s16)
pub fn vmls_n_s16(a: int16x4_t, b: int16x4_t, c: i16) -> int16x4_t {
    vmls_s16(a, b, vdup_n_s16(c))
}
///Vector multiply subtract with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmlsq_n_s16)
pub fn vmlsq_n_s16(a: int16x8_t, b: int16x8_t, c: i16) -> int16x8_t {
    vmlsq_s16(a, b, vdupq_n_s16(c))
}
///Vector multiply subtract with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmls_n_u16)
pub fn vmls_n_u16(a: uint16x4_t, b: uint16x4_t, c: u16) -> uint16x4_t {
    vmls_u16(a, b, vdup_n_u16(c))
}
///Vector multiply subtract with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmlsq_n_u16)
pub fn vmlsq_n_u16(a: uint16x8_t, b: uint16x8_t, c: u16) -> uint16x8_t {
    vmlsq_u16(a, b, vdupq_n_u16(c))
}
///Vector multiply subtract with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmls_n_s32)
pub fn vmls_n_s32(a: int32x2_t, b: int32x2_t, c: i32) -> int32x2_t {
    vmls_s32(a, b, vdup_n_s32(c))
}
///Vector multiply subtract with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmlsq_n_s32)
pub fn vmlsq_n_s32(a: int32x4_t, b: int32x4_t, c: i32) -> int32x4_t {
    vmlsq_s32(a, b, vdupq_n_s32(c))
}
///Vector multiply subtract with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmls_n_u32)
pub fn vmls_n_u32(a: uint32x2_t, b: uint32x2_t, c: u32) -> uint32x2_t {
    vmls_u32(a, b, vdup_n_u32(c))
}
///Vector multiply subtract with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmlsq_n_u32)
pub fn vmlsq_n_u32(a: uint32x4_t, b: uint32x4_t, c: u32) -> uint32x4_t {
    vmlsq_u32(a, b, vdupq_n_u32(c))
}
///Multiply-subtract from accumulator
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmls_s8)
pub fn vmls_s8(a: int8x8_t, b: int8x8_t, c: int8x8_t) -> int8x8_t {
    { simd_sub(a, simd_mul(b, c)) }
}
///Multiply-subtract from accumulator
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmlsq_s8)
pub fn vmlsq_s8(a: int8x16_t, b: int8x16_t, c: int8x16_t) -> int8x16_t {
    { simd_sub(a, simd_mul(b, c)) }
}
///Multiply-subtract from accumulator
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmls_s16)
pub fn vmls_s16(a: int16x4_t, b: int16x4_t, c: int16x4_t) -> int16x4_t {
    { simd_sub(a, simd_mul(b, c)) }
}
///Multiply-subtract from accumulator
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmlsq_s16)
pub fn vmlsq_s16(a: int16x8_t, b: int16x8_t, c: int16x8_t) -> int16x8_t {
    { simd_sub(a, simd_mul(b, c)) }
}
///Multiply-subtract from accumulator
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmls_s32)
pub fn vmls_s32(a: int32x2_t, b: int32x2_t, c: int32x2_t) -> int32x2_t {
    { simd_sub(a, simd_mul(b, c)) }
}
///Multiply-subtract from accumulator
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmlsq_s32)
pub fn vmlsq_s32(a: int32x4_t, b: int32x4_t, c: int32x4_t) -> int32x4_t {
    { simd_sub(a, simd_mul(b, c)) }
}
///Multiply-subtract from accumulator
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmls_u8)
pub fn vmls_u8(a: uint8x8_t, b: uint8x8_t, c: uint8x8_t) -> uint8x8_t {
    { simd_sub(a, simd_mul(b, c)) }
}
///Multiply-subtract from accumulator
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmlsq_u8)
pub fn vmlsq_u8(a: uint8x16_t, b: uint8x16_t, c: uint8x16_t) -> uint8x16_t {
    { simd_sub(a, simd_mul(b, c)) }
}
///Multiply-subtract from accumulator
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmls_u16)
pub fn vmls_u16(a: uint16x4_t, b: uint16x4_t, c: uint16x4_t) -> uint16x4_t {
    { simd_sub(a, simd_mul(b, c)) }
}
///Multiply-subtract from accumulator
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmlsq_u16)
pub fn vmlsq_u16(a: uint16x8_t, b: uint16x8_t, c: uint16x8_t) -> uint16x8_t {
    { simd_sub(a, simd_mul(b, c)) }
}
///Multiply-subtract from accumulator
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmls_u32)
pub fn vmls_u32(a: uint32x2_t, b: uint32x2_t, c: uint32x2_t) -> uint32x2_t {
    { simd_sub(a, simd_mul(b, c)) }
}
///Multiply-subtract from accumulator
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmlsq_u32)
pub fn vmlsq_u32(a: uint32x4_t, b: uint32x4_t, c: uint32x4_t) -> uint32x4_t {
    { simd_sub(a, simd_mul(b, c)) }
}
///Vector widening multiply subtract with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmlsl_lane_s16)
pub fn vmlsl_lane_s16<const LANE: i32>(
    a: int32x4_t,
    b: int16x4_t,
    c: int16x4_t,
) -> int32x4_t {
    static_assert_uimm_bits!(LANE, 2);
    {
        vmlsl_s16(
            a,
            b,
            simd_shuffle(c, c, [LANE as u32, LANE as u32, LANE as u32, LANE as u32]),
        )
    }
}
///Vector widening multiply subtract with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmlsl_laneq_s16)
pub fn vmlsl_laneq_s16<const LANE: i32>(
    a: int32x4_t,
    b: int16x4_t,
    c: int16x8_t,
) -> int32x4_t {
    static_assert_uimm_bits!(LANE, 3);
    {
        vmlsl_s16(
            a,
            b,
            simd_shuffle(c, c, [LANE as u32, LANE as u32, LANE as u32, LANE as u32]),
        )
    }
}
///Vector widening multiply subtract with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmlsl_lane_s32)
pub fn vmlsl_lane_s32<const LANE: i32>(
    a: int64x2_t,
    b: int32x2_t,
    c: int32x2_t,
) -> int64x2_t {
    static_assert_uimm_bits!(LANE, 1);
    { vmlsl_s32(a, b, simd_shuffle(c, c, [LANE as u32, LANE as u32])) }
}
///Vector widening multiply subtract with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmlsl_laneq_s32)
pub fn vmlsl_laneq_s32<const LANE: i32>(
    a: int64x2_t,
    b: int32x2_t,
    c: int32x4_t,
) -> int64x2_t {
    static_assert_uimm_bits!(LANE, 2);
    { vmlsl_s32(a, b, simd_shuffle(c, c, [LANE as u32, LANE as u32])) }
}
///Vector widening multiply subtract with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmlsl_lane_u16)
pub fn vmlsl_lane_u16<const LANE: i32>(
    a: uint32x4_t,
    b: uint16x4_t,
    c: uint16x4_t,
) -> uint32x4_t {
    static_assert_uimm_bits!(LANE, 2);
    {
        vmlsl_u16(
            a,
            b,
            simd_shuffle(c, c, [LANE as u32, LANE as u32, LANE as u32, LANE as u32]),
        )
    }
}
///Vector widening multiply subtract with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmlsl_laneq_u16)
pub fn vmlsl_laneq_u16<const LANE: i32>(
    a: uint32x4_t,
    b: uint16x4_t,
    c: uint16x8_t,
) -> uint32x4_t {
    static_assert_uimm_bits!(LANE, 3);
    {
        vmlsl_u16(
            a,
            b,
            simd_shuffle(c, c, [LANE as u32, LANE as u32, LANE as u32, LANE as u32]),
        )
    }
}
///Vector widening multiply subtract with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmlsl_lane_u32)
pub fn vmlsl_lane_u32<const LANE: i32>(
    a: uint64x2_t,
    b: uint32x2_t,
    c: uint32x2_t,
) -> uint64x2_t {
    static_assert_uimm_bits!(LANE, 1);
    { vmlsl_u32(a, b, simd_shuffle(c, c, [LANE as u32, LANE as u32])) }
}
///Vector widening multiply subtract with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmlsl_laneq_u32)
pub fn vmlsl_laneq_u32<const LANE: i32>(
    a: uint64x2_t,
    b: uint32x2_t,
    c: uint32x4_t,
) -> uint64x2_t {
    static_assert_uimm_bits!(LANE, 2);
    { vmlsl_u32(a, b, simd_shuffle(c, c, [LANE as u32, LANE as u32])) }
}
///Vector widening multiply subtract with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmlsl_n_s16)
pub fn vmlsl_n_s16(a: int32x4_t, b: int16x4_t, c: i16) -> int32x4_t {
    vmlsl_s16(a, b, vdup_n_s16(c))
}
///Vector widening multiply subtract with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmlsl_n_s32)
pub fn vmlsl_n_s32(a: int64x2_t, b: int32x2_t, c: i32) -> int64x2_t {
    vmlsl_s32(a, b, vdup_n_s32(c))
}
///Vector widening multiply subtract with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmlsl_n_u16)
pub fn vmlsl_n_u16(a: uint32x4_t, b: uint16x4_t, c: u16) -> uint32x4_t {
    vmlsl_u16(a, b, vdup_n_u16(c))
}
///Vector widening multiply subtract with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmlsl_n_u32)
pub fn vmlsl_n_u32(a: uint64x2_t, b: uint32x2_t, c: u32) -> uint64x2_t {
    vmlsl_u32(a, b, vdup_n_u32(c))
}
///Signed multiply-subtract long
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmlsl_s8)
pub fn vmlsl_s8(a: int16x8_t, b: int8x8_t, c: int8x8_t) -> int16x8_t {
    { simd_sub(a, vmull_s8(b, c)) }
}
///Signed multiply-subtract long
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmlsl_s16)
pub fn vmlsl_s16(a: int32x4_t, b: int16x4_t, c: int16x4_t) -> int32x4_t {
    { simd_sub(a, vmull_s16(b, c)) }
}
///Signed multiply-subtract long
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmlsl_s32)
pub fn vmlsl_s32(a: int64x2_t, b: int32x2_t, c: int32x2_t) -> int64x2_t {
    { simd_sub(a, vmull_s32(b, c)) }
}
///Unsigned multiply-subtract long
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmlsl_u8)
pub fn vmlsl_u8(a: uint16x8_t, b: uint8x8_t, c: uint8x8_t) -> uint16x8_t {
    { simd_sub(a, vmull_u8(b, c)) }
}
///Unsigned multiply-subtract long
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmlsl_u16)
pub fn vmlsl_u16(a: uint32x4_t, b: uint16x4_t, c: uint16x4_t) -> uint32x4_t {
    { simd_sub(a, vmull_u16(b, c)) }
}
///Unsigned multiply-subtract long
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmlsl_u32)
pub fn vmlsl_u32(a: uint64x2_t, b: uint32x2_t, c: uint32x2_t) -> uint64x2_t {
    { simd_sub(a, vmull_u32(b, c)) }
}
///8-bit integer matrix multiply-accumulate
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmmlaq_s32)
pub fn vmmlaq_s32(a: int32x4_t, b: int8x16_t, c: int8x16_t) -> int32x4_t {
    { _vmmlaq_s32(a, b, c) }
}
///8-bit integer matrix multiply-accumulate
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmmlaq_u32)
pub fn vmmlaq_u32(a: uint32x4_t, b: uint8x16_t, c: uint8x16_t) -> uint32x4_t {
    { _vmmlaq_u32(a, b, c) }
}
///Duplicate element to vector
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmov_n_f16)
pub fn vmov_n_f16(a: f16) -> float16x4_t {
    vdup_n_f16(a)
}
///Duplicate element to vector
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmovq_n_f16)
pub fn vmovq_n_f16(a: f16) -> float16x8_t {
    vdupq_n_f16(a)
}
///Duplicate vector element to vector or scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmov_n_f32)
pub fn vmov_n_f32(value: f32) -> float32x2_t {
    vdup_n_f32(value)
}
///Duplicate vector element to vector or scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmov_n_p16)
pub fn vmov_n_p16(value: p16) -> poly16x4_t {
    vdup_n_p16(value)
}
///Duplicate vector element to vector or scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmov_n_p8)
pub fn vmov_n_p8(value: p8) -> poly8x8_t {
    vdup_n_p8(value)
}
///Duplicate vector element to vector or scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmov_n_s16)
pub fn vmov_n_s16(value: i16) -> int16x4_t {
    vdup_n_s16(value)
}
///Duplicate vector element to vector or scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmov_n_s32)
pub fn vmov_n_s32(value: i32) -> int32x2_t {
    vdup_n_s32(value)
}
///Duplicate vector element to vector or scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmov_n_s64)
pub fn vmov_n_s64(value: i64) -> int64x1_t {
    vdup_n_s64(value)
}
///Duplicate vector element to vector or scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmov_n_s8)
pub fn vmov_n_s8(value: i8) -> int8x8_t {
    vdup_n_s8(value)
}
///Duplicate vector element to vector or scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmov_n_u16)
pub fn vmov_n_u16(value: u16) -> uint16x4_t {
    vdup_n_u16(value)
}
///Duplicate vector element to vector or scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmov_n_u32)
pub fn vmov_n_u32(value: u32) -> uint32x2_t {
    vdup_n_u32(value)
}
///Duplicate vector element to vector or scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmov_n_u64)
pub fn vmov_n_u64(value: u64) -> uint64x1_t {
    vdup_n_u64(value)
}
///Duplicate vector element to vector or scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmov_n_u8)
pub fn vmov_n_u8(value: u8) -> uint8x8_t {
    vdup_n_u8(value)
}
///Duplicate vector element to vector or scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmovq_n_f32)
pub fn vmovq_n_f32(value: f32) -> float32x4_t {
    vdupq_n_f32(value)
}
///Duplicate vector element to vector or scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmovq_n_p16)
pub fn vmovq_n_p16(value: p16) -> poly16x8_t {
    vdupq_n_p16(value)
}
///Duplicate vector element to vector or scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmovq_n_p8)
pub fn vmovq_n_p8(value: p8) -> poly8x16_t {
    vdupq_n_p8(value)
}
///Duplicate vector element to vector or scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmovq_n_s16)
pub fn vmovq_n_s16(value: i16) -> int16x8_t {
    vdupq_n_s16(value)
}
///Duplicate vector element to vector or scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmovq_n_s32)
pub fn vmovq_n_s32(value: i32) -> int32x4_t {
    vdupq_n_s32(value)
}
///Duplicate vector element to vector or scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmovq_n_s64)
pub fn vmovq_n_s64(value: i64) -> int64x2_t {
    vdupq_n_s64(value)
}
///Duplicate vector element to vector or scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmovq_n_s8)
pub fn vmovq_n_s8(value: i8) -> int8x16_t {
    vdupq_n_s8(value)
}
///Duplicate vector element to vector or scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmovq_n_u16)
pub fn vmovq_n_u16(value: u16) -> uint16x8_t {
    vdupq_n_u16(value)
}
///Duplicate vector element to vector or scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmovq_n_u32)
pub fn vmovq_n_u32(value: u32) -> uint32x4_t {
    vdupq_n_u32(value)
}
///Duplicate vector element to vector or scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmovq_n_u64)
pub fn vmovq_n_u64(value: u64) -> uint64x2_t {
    vdupq_n_u64(value)
}
///Duplicate vector element to vector or scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmovq_n_u8)
pub fn vmovq_n_u8(value: u8) -> uint8x16_t {
    vdupq_n_u8(value)
}
///Vector long move.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmovl_s16)
pub fn vmovl_s16(a: int16x4_t) -> int32x4_t {
    { simd_cast(a) }
}
///Vector long move.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmovl_s32)
pub fn vmovl_s32(a: int32x2_t) -> int64x2_t {
    { simd_cast(a) }
}
///Vector long move.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmovl_s8)
pub fn vmovl_s8(a: int8x8_t) -> int16x8_t {
    { simd_cast(a) }
}
///Vector long move.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmovl_u16)
pub fn vmovl_u16(a: uint16x4_t) -> uint32x4_t {
    { simd_cast(a) }
}
///Vector long move.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmovl_u32)
pub fn vmovl_u32(a: uint32x2_t) -> uint64x2_t {
    { simd_cast(a) }
}
///Vector long move.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmovl_u8)
pub fn vmovl_u8(a: uint8x8_t) -> uint16x8_t {
    { simd_cast(a) }
}
///Vector narrow integer.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmovn_s16)
pub fn vmovn_s16(a: int16x8_t) -> int8x8_t {
    { simd_cast(a) }
}
///Vector narrow integer.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmovn_s32)
pub fn vmovn_s32(a: int32x4_t) -> int16x4_t {
    { simd_cast(a) }
}
///Vector narrow integer.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmovn_s64)
pub fn vmovn_s64(a: int64x2_t) -> int32x2_t {
    { simd_cast(a) }
}
///Vector narrow integer.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmovn_u16)
pub fn vmovn_u16(a: uint16x8_t) -> uint8x8_t {
    { simd_cast(a) }
}
///Vector narrow integer.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmovn_u32)
pub fn vmovn_u32(a: uint32x4_t) -> uint16x4_t {
    { simd_cast(a) }
}
///Vector narrow integer.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmovn_u64)
pub fn vmovn_u64(a: uint64x2_t) -> uint32x2_t {
    { simd_cast(a) }
}
///Multiply
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmul_f16)
pub fn vmul_f16(a: float16x4_t, b: float16x4_t) -> float16x4_t {
    { simd_mul(a, b) }
}
///Multiply
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmulq_f16)
pub fn vmulq_f16(a: float16x8_t, b: float16x8_t) -> float16x8_t {
    { simd_mul(a, b) }
}
///Multiply
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmul_f32)
pub fn vmul_f32(a: float32x2_t, b: float32x2_t) -> float32x2_t {
    { simd_mul(a, b) }
}
///Multiply
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmulq_f32)
pub fn vmulq_f32(a: float32x4_t, b: float32x4_t) -> float32x4_t {
    { simd_mul(a, b) }
}
///Multiply
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmul_lane_f16)
pub fn vmul_lane_f16<const LANE: i32>(a: float16x4_t, v: float16x4_t) -> float16x4_t {
    static_assert_uimm_bits!(LANE, 2);
    {
        simd_mul(
            a,
            simd_shuffle(v, v, [LANE as u32, LANE as u32, LANE as u32, LANE as u32]),
        )
    }
}
///Multiply
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmulq_lane_f16)
pub fn vmulq_lane_f16<const LANE: i32>(a: float16x8_t, v: float16x4_t) -> float16x8_t {
    static_assert_uimm_bits!(LANE, 2);
    {
        simd_mul(
            a,
            simd_shuffle(
                v, v, [LANE as u32, LANE as u32, LANE as u32, LANE as u32, LANE as u32,
                LANE as u32, LANE as u32, LANE as u32]
            ),
        )
    }
}
///Floating-point multiply
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmul_lane_f32)
pub fn vmul_lane_f32<const LANE: i32>(a: float32x2_t, b: float32x2_t) -> float32x2_t {
    static_assert_uimm_bits!(LANE, 1);
    { simd_mul(a, simd_shuffle(b, b, [LANE as u32, LANE as u32])) }
}
///Floating-point multiply
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmul_laneq_f32)
pub fn vmul_laneq_f32<const LANE: i32>(a: float32x2_t, b: float32x4_t) -> float32x2_t {
    static_assert_uimm_bits!(LANE, 2);
    { simd_mul(a, simd_shuffle(b, b, [LANE as u32, LANE as u32])) }
}
///Floating-point multiply
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmulq_lane_f32)
pub fn vmulq_lane_f32<const LANE: i32>(a: float32x4_t, b: float32x2_t) -> float32x4_t {
    static_assert_uimm_bits!(LANE, 1);
    {
        simd_mul(
            a,
            simd_shuffle(b, b, [LANE as u32, LANE as u32, LANE as u32, LANE as u32]),
        )
    }
}
///Floating-point multiply
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmulq_laneq_f32)
pub fn vmulq_laneq_f32<const LANE: i32>(a: float32x4_t, b: float32x4_t) -> float32x4_t {
    static_assert_uimm_bits!(LANE, 2);
    {
        simd_mul(
            a,
            simd_shuffle(b, b, [LANE as u32, LANE as u32, LANE as u32, LANE as u32]),
        )
    }
}
///Multiply
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmul_lane_s16)
pub fn vmul_lane_s16<const LANE: i32>(a: int16x4_t, b: int16x4_t) -> int16x4_t {
    static_assert_uimm_bits!(LANE, 2);
    {
        simd_mul(
            a,
            simd_shuffle(b, b, [LANE as u32, LANE as u32, LANE as u32, LANE as u32]),
        )
    }
}
///Multiply
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmulq_lane_s16)
pub fn vmulq_lane_s16<const LANE: i32>(a: int16x8_t, b: int16x4_t) -> int16x8_t {
    static_assert_uimm_bits!(LANE, 2);
    {
        simd_mul(
            a,
            simd_shuffle(
                b, b, [LANE as u32, LANE as u32, LANE as u32, LANE as u32, LANE as u32,
                LANE as u32, LANE as u32, LANE as u32]
            ),
        )
    }
}
///Multiply
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmul_lane_s32)
pub fn vmul_lane_s32<const LANE: i32>(a: int32x2_t, b: int32x2_t) -> int32x2_t {
    static_assert_uimm_bits!(LANE, 1);
    { simd_mul(a, simd_shuffle(b, b, [LANE as u32, LANE as u32])) }
}
///Multiply
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmulq_lane_s32)
pub fn vmulq_lane_s32<const LANE: i32>(a: int32x4_t, b: int32x2_t) -> int32x4_t {
    static_assert_uimm_bits!(LANE, 1);
    {
        simd_mul(
            a,
            simd_shuffle(b, b, [LANE as u32, LANE as u32, LANE as u32, LANE as u32]),
        )
    }
}
///Multiply
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmul_lane_u16)
pub fn vmul_lane_u16<const LANE: i32>(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
    static_assert_uimm_bits!(LANE, 2);
    {
        simd_mul(
            a,
            simd_shuffle(b, b, [LANE as u32, LANE as u32, LANE as u32, LANE as u32]),
        )
    }
}
///Multiply
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmulq_lane_u16)
pub fn vmulq_lane_u16<const LANE: i32>(a: uint16x8_t, b: uint16x4_t) -> uint16x8_t {
    static_assert_uimm_bits!(LANE, 2);
    {
        simd_mul(
            a,
            simd_shuffle(
                b, b, [LANE as u32, LANE as u32, LANE as u32, LANE as u32, LANE as u32,
                LANE as u32, LANE as u32, LANE as u32]
            ),
        )
    }
}
///Multiply
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmul_lane_u32)
pub fn vmul_lane_u32<const LANE: i32>(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
    static_assert_uimm_bits!(LANE, 1);
    { simd_mul(a, simd_shuffle(b, b, [LANE as u32, LANE as u32])) }
}
///Multiply
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmulq_lane_u32)
pub fn vmulq_lane_u32<const LANE: i32>(a: uint32x4_t, b: uint32x2_t) -> uint32x4_t {
    static_assert_uimm_bits!(LANE, 1);
    {
        simd_mul(
            a,
            simd_shuffle(b, b, [LANE as u32, LANE as u32, LANE as u32, LANE as u32]),
        )
    }
}
///Multiply
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmul_laneq_s16)
pub fn vmul_laneq_s16<const LANE: i32>(a: int16x4_t, b: int16x8_t) -> int16x4_t {
    static_assert_uimm_bits!(LANE, 3);
    {
        simd_mul(
            a,
            simd_shuffle(b, b, [LANE as u32, LANE as u32, LANE as u32, LANE as u32]),
        )
    }
}
///Multiply
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmulq_laneq_s16)
pub fn vmulq_laneq_s16<const LANE: i32>(a: int16x8_t, b: int16x8_t) -> int16x8_t {
    static_assert_uimm_bits!(LANE, 3);
    {
        simd_mul(
            a,
            simd_shuffle(
                b, b, [LANE as u32, LANE as u32, LANE as u32, LANE as u32, LANE as u32,
                LANE as u32, LANE as u32, LANE as u32]
            ),
        )
    }
}
///Multiply
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmul_laneq_s32)
pub fn vmul_laneq_s32<const LANE: i32>(a: int32x2_t, b: int32x4_t) -> int32x2_t {
    static_assert_uimm_bits!(LANE, 2);
    { simd_mul(a, simd_shuffle(b, b, [LANE as u32, LANE as u32])) }
}
///Multiply
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmulq_laneq_s32)
pub fn vmulq_laneq_s32<const LANE: i32>(a: int32x4_t, b: int32x4_t) -> int32x4_t {
    static_assert_uimm_bits!(LANE, 2);
    {
        simd_mul(
            a,
            simd_shuffle(b, b, [LANE as u32, LANE as u32, LANE as u32, LANE as u32]),
        )
    }
}
///Multiply
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmul_laneq_u16)
pub fn vmul_laneq_u16<const LANE: i32>(a: uint16x4_t, b: uint16x8_t) -> uint16x4_t {
    static_assert_uimm_bits!(LANE, 3);
    {
        simd_mul(
            a,
            simd_shuffle(b, b, [LANE as u32, LANE as u32, LANE as u32, LANE as u32]),
        )
    }
}
///Multiply
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmulq_laneq_u16)
pub fn vmulq_laneq_u16<const LANE: i32>(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
    static_assert_uimm_bits!(LANE, 3);
    {
        simd_mul(
            a,
            simd_shuffle(
                b, b, [LANE as u32, LANE as u32, LANE as u32, LANE as u32, LANE as u32,
                LANE as u32, LANE as u32, LANE as u32]
            ),
        )
    }
}
///Multiply
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmul_laneq_u32)
pub fn vmul_laneq_u32<const LANE: i32>(a: uint32x2_t, b: uint32x4_t) -> uint32x2_t {
    static_assert_uimm_bits!(LANE, 2);
    { simd_mul(a, simd_shuffle(b, b, [LANE as u32, LANE as u32])) }
}
///Multiply
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmulq_laneq_u32)
pub fn vmulq_laneq_u32<const LANE: i32>(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
    static_assert_uimm_bits!(LANE, 2);
    {
        simd_mul(
            a,
            simd_shuffle(b, b, [LANE as u32, LANE as u32, LANE as u32, LANE as u32]),
        )
    }
}
///Vector multiply by scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmul_n_f16)
pub fn vmul_n_f16(a: float16x4_t, b: f16) -> float16x4_t {
    { simd_mul(a, vdup_n_f16(b)) }
}
///Vector multiply by scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmulq_n_f16)
pub fn vmulq_n_f16(a: float16x8_t, b: f16) -> float16x8_t {
    { simd_mul(a, vdupq_n_f16(b)) }
}
///Vector multiply by scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmul_n_f32)
pub fn vmul_n_f32(a: float32x2_t, b: f32) -> float32x2_t {
    { simd_mul(a, vdup_n_f32(b)) }
}
///Vector multiply by scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmulq_n_f32)
pub fn vmulq_n_f32(a: float32x4_t, b: f32) -> float32x4_t {
    { simd_mul(a, vdupq_n_f32(b)) }
}
///Vector multiply by scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmul_n_s16)
pub fn vmul_n_s16(a: int16x4_t, b: i16) -> int16x4_t {
    { simd_mul(a, vdup_n_s16(b)) }
}
///Vector multiply by scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmulq_n_s16)
pub fn vmulq_n_s16(a: int16x8_t, b: i16) -> int16x8_t {
    { simd_mul(a, vdupq_n_s16(b)) }
}
///Vector multiply by scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmul_n_s32)
pub fn vmul_n_s32(a: int32x2_t, b: i32) -> int32x2_t {
    { simd_mul(a, vdup_n_s32(b)) }
}
///Vector multiply by scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmulq_n_s32)
pub fn vmulq_n_s32(a: int32x4_t, b: i32) -> int32x4_t {
    { simd_mul(a, vdupq_n_s32(b)) }
}
///Vector multiply by scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmul_n_u16)
pub fn vmul_n_u16(a: uint16x4_t, b: u16) -> uint16x4_t {
    { simd_mul(a, vdup_n_u16(b)) }
}
///Vector multiply by scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmulq_n_u16)
pub fn vmulq_n_u16(a: uint16x8_t, b: u16) -> uint16x8_t {
    { simd_mul(a, vdupq_n_u16(b)) }
}
///Vector multiply by scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmul_n_u32)
pub fn vmul_n_u32(a: uint32x2_t, b: u32) -> uint32x2_t {
    { simd_mul(a, vdup_n_u32(b)) }
}
///Vector multiply by scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmulq_n_u32)
pub fn vmulq_n_u32(a: uint32x4_t, b: u32) -> uint32x4_t {
    { simd_mul(a, vdupq_n_u32(b)) }
}
///Polynomial multiply
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmul_p8)
pub fn vmul_p8(a: poly8x8_t, b: poly8x8_t) -> poly8x8_t {
    { _vmul_p8(a, b) }
}
///Polynomial multiply
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmulq_p8)
pub fn vmulq_p8(a: poly8x16_t, b: poly8x16_t) -> poly8x16_t {
    { _vmulq_p8(a, b) }
}
///Multiply
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmul_s16)
pub fn vmul_s16(a: int16x4_t, b: int16x4_t) -> int16x4_t {
    { simd_mul(a, b) }
}
///Multiply
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmulq_s16)
pub fn vmulq_s16(a: int16x8_t, b: int16x8_t) -> int16x8_t {
    { simd_mul(a, b) }
}
///Multiply
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmul_u16)
pub fn vmul_u16(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
    { simd_mul(a, b) }
}
///Multiply
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmulq_u16)
pub fn vmulq_u16(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
    { simd_mul(a, b) }
}
///Multiply
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmul_s32)
pub fn vmul_s32(a: int32x2_t, b: int32x2_t) -> int32x2_t {
    { simd_mul(a, b) }
}
///Multiply
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmulq_s32)
pub fn vmulq_s32(a: int32x4_t, b: int32x4_t) -> int32x4_t {
    { simd_mul(a, b) }
}
///Multiply
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmul_u32)
pub fn vmul_u32(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
    { simd_mul(a, b) }
}
///Multiply
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmulq_u32)
pub fn vmulq_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
    { simd_mul(a, b) }
}
///Multiply
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmul_s8)
pub fn vmul_s8(a: int8x8_t, b: int8x8_t) -> int8x8_t {
    { simd_mul(a, b) }
}
///Multiply
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmulq_s8)
pub fn vmulq_s8(a: int8x16_t, b: int8x16_t) -> int8x16_t {
    { simd_mul(a, b) }
}
///Multiply
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmul_u8)
pub fn vmul_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
    { simd_mul(a, b) }
}
///Multiply
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmulq_u8)
pub fn vmulq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
    { simd_mul(a, b) }
}
///Vector long multiply by scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmull_lane_s16)
pub fn vmull_lane_s16<const LANE: i32>(a: int16x4_t, b: int16x4_t) -> int32x4_t {
    static_assert_uimm_bits!(LANE, 2);
    {
        vmull_s16(
            a,
            simd_shuffle(b, b, [LANE as u32, LANE as u32, LANE as u32, LANE as u32]),
        )
    }
}
///Vector long multiply by scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmull_laneq_s16)
pub fn vmull_laneq_s16<const LANE: i32>(a: int16x4_t, b: int16x8_t) -> int32x4_t {
    static_assert_uimm_bits!(LANE, 3);
    {
        vmull_s16(
            a,
            simd_shuffle(b, b, [LANE as u32, LANE as u32, LANE as u32, LANE as u32]),
        )
    }
}
///Vector long multiply by scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmull_lane_s32)
pub fn vmull_lane_s32<const LANE: i32>(a: int32x2_t, b: int32x2_t) -> int64x2_t {
    static_assert_uimm_bits!(LANE, 1);
    { vmull_s32(a, simd_shuffle(b, b, [LANE as u32, LANE as u32])) }
}
///Vector long multiply by scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmull_laneq_s32)
pub fn vmull_laneq_s32<const LANE: i32>(a: int32x2_t, b: int32x4_t) -> int64x2_t {
    static_assert_uimm_bits!(LANE, 2);
    { vmull_s32(a, simd_shuffle(b, b, [LANE as u32, LANE as u32])) }
}
///Vector long multiply by scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmull_lane_u16)
pub fn vmull_lane_u16<const LANE: i32>(a: uint16x4_t, b: uint16x4_t) -> uint32x4_t {
    static_assert_uimm_bits!(LANE, 2);
    {
        vmull_u16(
            a,
            simd_shuffle(b, b, [LANE as u32, LANE as u32, LANE as u32, LANE as u32]),
        )
    }
}
///Vector long multiply by scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmull_laneq_u16)
pub fn vmull_laneq_u16<const LANE: i32>(a: uint16x4_t, b: uint16x8_t) -> uint32x4_t {
    static_assert_uimm_bits!(LANE, 3);
    {
        vmull_u16(
            a,
            simd_shuffle(b, b, [LANE as u32, LANE as u32, LANE as u32, LANE as u32]),
        )
    }
}
///Vector long multiply by scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmull_lane_u32)
pub fn vmull_lane_u32<const LANE: i32>(a: uint32x2_t, b: uint32x2_t) -> uint64x2_t {
    static_assert_uimm_bits!(LANE, 1);
    { vmull_u32(a, simd_shuffle(b, b, [LANE as u32, LANE as u32])) }
}
///Vector long multiply by scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmull_laneq_u32)
pub fn vmull_laneq_u32<const LANE: i32>(a: uint32x2_t, b: uint32x4_t) -> uint64x2_t {
    static_assert_uimm_bits!(LANE, 2);
    { vmull_u32(a, simd_shuffle(b, b, [LANE as u32, LANE as u32])) }
}
///Vector long multiply with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmull_n_s16)
pub fn vmull_n_s16(a: int16x4_t, b: i16) -> int32x4_t {
    vmull_s16(a, vdup_n_s16(b))
}
///Vector long multiply with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmull_n_s32)
pub fn vmull_n_s32(a: int32x2_t, b: i32) -> int64x2_t {
    vmull_s32(a, vdup_n_s32(b))
}
///Vector long multiply with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmull_n_u16)
pub fn vmull_n_u16(a: uint16x4_t, b: u16) -> uint32x4_t {
    vmull_u16(a, vdup_n_u16(b))
}
///Vector long multiply with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmull_n_u32)
pub fn vmull_n_u32(a: uint32x2_t, b: u32) -> uint64x2_t {
    vmull_u32(a, vdup_n_u32(b))
}
///Polynomial multiply long
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmull_p8)
pub fn vmull_p8(a: poly8x8_t, b: poly8x8_t) -> poly16x8_t {
    { _vmull_p8(a, b) }
}
///Signed multiply long
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmull_s16)
pub fn vmull_s16(a: int16x4_t, b: int16x4_t) -> int32x4_t {
    { _vmull_s16(a, b) }
}
///Signed multiply long
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmull_s32)
pub fn vmull_s32(a: int32x2_t, b: int32x2_t) -> int64x2_t {
    { _vmull_s32(a, b) }
}
///Signed multiply long
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmull_s8)
pub fn vmull_s8(a: int8x8_t, b: int8x8_t) -> int16x8_t {
    { _vmull_s8(a, b) }
}
///Unsigned multiply long
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmull_u8)
pub fn vmull_u8(a: uint8x8_t, b: uint8x8_t) -> uint16x8_t {
    { _vmull_u8(a, b) }
}
///Unsigned multiply long
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmull_u16)
pub fn vmull_u16(a: uint16x4_t, b: uint16x4_t) -> uint32x4_t {
    { _vmull_u16(a, b) }
}
///Unsigned multiply long
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmull_u32)
pub fn vmull_u32(a: uint32x2_t, b: uint32x2_t) -> uint64x2_t {
    { _vmull_u32(a, b) }
}
///Vector bitwise not.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmvn_p8)
pub fn vmvn_p8(a: poly8x8_t) -> poly8x8_t {
    let b = poly8x8_t::splat(255);
    { simd_xor(a, b) }
}
///Vector bitwise not.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmvn_s16)
pub fn vmvn_s16(a: int16x4_t) -> int16x4_t {
    let b = int16x4_t::splat(-1);
    { simd_xor(a, b) }
}
///Vector bitwise not.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmvn_s32)
pub fn vmvn_s32(a: int32x2_t) -> int32x2_t {
    let b = int32x2_t::splat(-1);
    { simd_xor(a, b) }
}
///Vector bitwise not.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmvn_s8)
pub fn vmvn_s8(a: int8x8_t) -> int8x8_t {
    let b = int8x8_t::splat(-1);
    { simd_xor(a, b) }
}
///Vector bitwise not.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmvn_u16)
pub fn vmvn_u16(a: uint16x4_t) -> uint16x4_t {
    let b = uint16x4_t::splat(65_535);
    { simd_xor(a, b) }
}
///Vector bitwise not.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmvn_u32)
pub fn vmvn_u32(a: uint32x2_t) -> uint32x2_t {
    let b = uint32x2_t::splat(4_294_967_295);
    { simd_xor(a, b) }
}
///Vector bitwise not.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmvn_u8)
pub fn vmvn_u8(a: uint8x8_t) -> uint8x8_t {
    let b = uint8x8_t::splat(255);
    { simd_xor(a, b) }
}
///Vector bitwise not.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmvnq_p8)
pub fn vmvnq_p8(a: poly8x16_t) -> poly8x16_t {
    let b = poly8x16_t::splat(255);
    { simd_xor(a, b) }
}
///Vector bitwise not.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmvnq_s16)
pub fn vmvnq_s16(a: int16x8_t) -> int16x8_t {
    let b = int16x8_t::splat(-1);
    { simd_xor(a, b) }
}
///Vector bitwise not.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmvnq_s32)
pub fn vmvnq_s32(a: int32x4_t) -> int32x4_t {
    let b = int32x4_t::splat(-1);
    { simd_xor(a, b) }
}
///Vector bitwise not.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmvnq_s8)
pub fn vmvnq_s8(a: int8x16_t) -> int8x16_t {
    let b = int8x16_t::splat(-1);
    { simd_xor(a, b) }
}
///Vector bitwise not.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmvnq_u16)
pub fn vmvnq_u16(a: uint16x8_t) -> uint16x8_t {
    let b = uint16x8_t::splat(65_535);
    { simd_xor(a, b) }
}
///Vector bitwise not.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmvnq_u32)
pub fn vmvnq_u32(a: uint32x4_t) -> uint32x4_t {
    let b = uint32x4_t::splat(4_294_967_295);
    { simd_xor(a, b) }
}
///Vector bitwise not.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmvnq_u8)
pub fn vmvnq_u8(a: uint8x16_t) -> uint8x16_t {
    let b = uint8x16_t::splat(255);
    { simd_xor(a, b) }
}
///Negate
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vneg_f16)
pub fn vneg_f16(a: float16x4_t) -> float16x4_t {
    { simd_neg(a) }
}
///Negate
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vnegq_f16)
pub fn vnegq_f16(a: float16x8_t) -> float16x8_t {
    { simd_neg(a) }
}
///Negate
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vneg_f32)
pub fn vneg_f32(a: float32x2_t) -> float32x2_t {
    { simd_neg(a) }
}
///Negate
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vnegq_f32)
pub fn vnegq_f32(a: float32x4_t) -> float32x4_t {
    { simd_neg(a) }
}
///Negate
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vneg_s8)
pub fn vneg_s8(a: int8x8_t) -> int8x8_t {
    { simd_neg(a) }
}
///Negate
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vnegq_s8)
pub fn vnegq_s8(a: int8x16_t) -> int8x16_t {
    { simd_neg(a) }
}
///Negate
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vneg_s16)
pub fn vneg_s16(a: int16x4_t) -> int16x4_t {
    { simd_neg(a) }
}
///Negate
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vnegq_s16)
pub fn vnegq_s16(a: int16x8_t) -> int16x8_t {
    { simd_neg(a) }
}
///Negate
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vneg_s32)
pub fn vneg_s32(a: int32x2_t) -> int32x2_t {
    { simd_neg(a) }
}
///Negate
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vnegq_s32)
pub fn vnegq_s32(a: int32x4_t) -> int32x4_t {
    { simd_neg(a) }
}
///Vector bitwise inclusive OR NOT
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vorn_s16)
pub fn vorn_s16(a: int16x4_t, b: int16x4_t) -> int16x4_t {
    let c = int16x4_t::splat(-1);
    { simd_or(simd_xor(b, c), a) }
}
///Vector bitwise inclusive OR NOT
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vorn_s32)
pub fn vorn_s32(a: int32x2_t, b: int32x2_t) -> int32x2_t {
    let c = int32x2_t::splat(-1);
    { simd_or(simd_xor(b, c), a) }
}
///Vector bitwise inclusive OR NOT
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vorn_s64)
pub fn vorn_s64(a: int64x1_t, b: int64x1_t) -> int64x1_t {
    let c = int64x1_t::splat(-1);
    { simd_or(simd_xor(b, c), a) }
}
///Vector bitwise inclusive OR NOT
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vorn_s8)
pub fn vorn_s8(a: int8x8_t, b: int8x8_t) -> int8x8_t {
    let c = int8x8_t::splat(-1);
    { simd_or(simd_xor(b, c), a) }
}
///Vector bitwise inclusive OR NOT
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vornq_s16)
pub fn vornq_s16(a: int16x8_t, b: int16x8_t) -> int16x8_t {
    let c = int16x8_t::splat(-1);
    { simd_or(simd_xor(b, c), a) }
}
///Vector bitwise inclusive OR NOT
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vornq_s32)
pub fn vornq_s32(a: int32x4_t, b: int32x4_t) -> int32x4_t {
    let c = int32x4_t::splat(-1);
    { simd_or(simd_xor(b, c), a) }
}
///Vector bitwise inclusive OR NOT
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vornq_s64)
pub fn vornq_s64(a: int64x2_t, b: int64x2_t) -> int64x2_t {
    let c = int64x2_t::splat(-1);
    { simd_or(simd_xor(b, c), a) }
}
///Vector bitwise inclusive OR NOT
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vornq_s8)
pub fn vornq_s8(a: int8x16_t, b: int8x16_t) -> int8x16_t {
    let c = int8x16_t::splat(-1);
    { simd_or(simd_xor(b, c), a) }
}
///Vector bitwise inclusive OR NOT
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vorn_u16)
pub fn vorn_u16(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
    let c = int16x4_t::splat(-1);
    { simd_or(simd_xor(b, transmute(c)), a) }
}
///Vector bitwise inclusive OR NOT
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vorn_u32)
pub fn vorn_u32(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
    let c = int32x2_t::splat(-1);
    { simd_or(simd_xor(b, transmute(c)), a) }
}
///Vector bitwise inclusive OR NOT
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vorn_u64)
pub fn vorn_u64(a: uint64x1_t, b: uint64x1_t) -> uint64x1_t {
    let c = int64x1_t::splat(-1);
    { simd_or(simd_xor(b, transmute(c)), a) }
}
///Vector bitwise inclusive OR NOT
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vorn_u8)
pub fn vorn_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
    let c = int8x8_t::splat(-1);
    { simd_or(simd_xor(b, transmute(c)), a) }
}
///Vector bitwise inclusive OR NOT
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vornq_u16)
pub fn vornq_u16(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
    let c = int16x8_t::splat(-1);
    { simd_or(simd_xor(b, transmute(c)), a) }
}
///Vector bitwise inclusive OR NOT
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vornq_u32)
pub fn vornq_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
    let c = int32x4_t::splat(-1);
    { simd_or(simd_xor(b, transmute(c)), a) }
}
///Vector bitwise inclusive OR NOT
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vornq_u64)
pub fn vornq_u64(a: uint64x2_t, b: uint64x2_t) -> uint64x2_t {
    let c = int64x2_t::splat(-1);
    { simd_or(simd_xor(b, transmute(c)), a) }
}
///Vector bitwise inclusive OR NOT
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vornq_u8)
pub fn vornq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
    let c = int8x16_t::splat(-1);
    { simd_or(simd_xor(b, transmute(c)), a) }
}
///Vector bitwise or (immediate, inclusive)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vorr_s8)
pub fn vorr_s8(a: int8x8_t, b: int8x8_t) -> int8x8_t {
    { simd_or(a, b) }
}
///Vector bitwise or (immediate, inclusive)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vorrq_s8)
pub fn vorrq_s8(a: int8x16_t, b: int8x16_t) -> int8x16_t {
    { simd_or(a, b) }
}
///Vector bitwise or (immediate, inclusive)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vorr_s16)
pub fn vorr_s16(a: int16x4_t, b: int16x4_t) -> int16x4_t {
    { simd_or(a, b) }
}
///Vector bitwise or (immediate, inclusive)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vorrq_s16)
pub fn vorrq_s16(a: int16x8_t, b: int16x8_t) -> int16x8_t {
    { simd_or(a, b) }
}
///Vector bitwise or (immediate, inclusive)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vorr_s32)
pub fn vorr_s32(a: int32x2_t, b: int32x2_t) -> int32x2_t {
    { simd_or(a, b) }
}
///Vector bitwise or (immediate, inclusive)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vorrq_s32)
pub fn vorrq_s32(a: int32x4_t, b: int32x4_t) -> int32x4_t {
    { simd_or(a, b) }
}
///Vector bitwise or (immediate, inclusive)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vorr_s64)
pub fn vorr_s64(a: int64x1_t, b: int64x1_t) -> int64x1_t {
    { simd_or(a, b) }
}
///Vector bitwise or (immediate, inclusive)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vorrq_s64)
pub fn vorrq_s64(a: int64x2_t, b: int64x2_t) -> int64x2_t {
    { simd_or(a, b) }
}
///Vector bitwise or (immediate, inclusive)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vorr_u8)
pub fn vorr_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
    { simd_or(a, b) }
}
///Vector bitwise or (immediate, inclusive)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vorrq_u8)
pub fn vorrq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
    { simd_or(a, b) }
}
///Vector bitwise or (immediate, inclusive)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vorr_u16)
pub fn vorr_u16(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
    { simd_or(a, b) }
}
///Vector bitwise or (immediate, inclusive)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vorrq_u16)
pub fn vorrq_u16(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
    { simd_or(a, b) }
}
///Vector bitwise or (immediate, inclusive)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vorr_u32)
pub fn vorr_u32(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
    { simd_or(a, b) }
}
///Vector bitwise or (immediate, inclusive)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vorrq_u32)
pub fn vorrq_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
    { simd_or(a, b) }
}
///Vector bitwise or (immediate, inclusive)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vorr_u64)
pub fn vorr_u64(a: uint64x1_t, b: uint64x1_t) -> uint64x1_t {
    { simd_or(a, b) }
}
///Vector bitwise or (immediate, inclusive)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vorrq_u64)
pub fn vorrq_u64(a: uint64x2_t, b: uint64x2_t) -> uint64x2_t {
    { simd_or(a, b) }
}
///Signed Add and Accumulate Long Pairwise.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vpadal_s8)
pub fn vpadal_s8(a: int16x4_t, b: int8x8_t) -> int16x4_t {
    let x: int16x4_t;
    #[cfg(target_arch = "arm")]
    {
        x = priv_vpadal_s8(a, b);
    }
    {
        x = simd_add(vpaddl_s8(b), a);
    };
    x
}
///Signed Add and Accumulate Long Pairwise.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vpadalq_s8)
pub fn vpadalq_s8(a: int16x8_t, b: int8x16_t) -> int16x8_t {
    let x: int16x8_t;
    #[cfg(target_arch = "arm")]
    {
        x = priv_vpadalq_s8(a, b);
    }
    {
        x = simd_add(vpaddlq_s8(b), a);
    };
    x
}
///Signed Add and Accumulate Long Pairwise.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vpadal_s16)
pub fn vpadal_s16(a: int32x2_t, b: int16x4_t) -> int32x2_t {
    let x: int32x2_t;
    #[cfg(target_arch = "arm")]
    {
        x = priv_vpadal_s16(a, b);
    }
    {
        x = simd_add(vpaddl_s16(b), a);
    };
    x
}
///Signed Add and Accumulate Long Pairwise.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vpadalq_s16)
pub fn vpadalq_s16(a: int32x4_t, b: int16x8_t) -> int32x4_t {
    let x: int32x4_t;
    #[cfg(target_arch = "arm")]
    {
        x = priv_vpadalq_s16(a, b);
    }
    {
        x = simd_add(vpaddlq_s16(b), a);
    };
    x
}
///Signed Add and Accumulate Long Pairwise.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vpadal_s32)
pub fn vpadal_s32(a: int64x1_t, b: int32x2_t) -> int64x1_t {
    let x: int64x1_t;
    #[cfg(target_arch = "arm")]
    {
        x = priv_vpadal_s32(a, b);
    }
    {
        x = simd_add(vpaddl_s32(b), a);
    };
    x
}
///Signed Add and Accumulate Long Pairwise.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vpadalq_s32)
pub fn vpadalq_s32(a: int64x2_t, b: int32x4_t) -> int64x2_t {
    let x: int64x2_t;
    #[cfg(target_arch = "arm")]
    {
        x = priv_vpadalq_s32(a, b);
    }
    {
        x = simd_add(vpaddlq_s32(b), a);
    };
    x
}
///Unsigned Add and Accumulate Long Pairwise.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vpadal_u8)
pub fn vpadal_u8(a: uint16x4_t, b: uint8x8_t) -> uint16x4_t {
    let x: uint16x4_t;
    #[cfg(target_arch = "arm")]
    {
        x = priv_vpadal_u8(a, b);
    }
    {
        x = simd_add(vpaddl_u8(b), a);
    };
    x
}
///Unsigned Add and Accumulate Long Pairwise.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vpadalq_u8)
pub fn vpadalq_u8(a: uint16x8_t, b: uint8x16_t) -> uint16x8_t {
    let x: uint16x8_t;
    #[cfg(target_arch = "arm")]
    {
        x = priv_vpadalq_u8(a, b);
    }
    {
        x = simd_add(vpaddlq_u8(b), a);
    };
    x
}
///Unsigned Add and Accumulate Long Pairwise.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vpadal_u16)
pub fn vpadal_u16(a: uint32x2_t, b: uint16x4_t) -> uint32x2_t {
    let x: uint32x2_t;
    #[cfg(target_arch = "arm")]
    {
        x = priv_vpadal_u16(a, b);
    }
    {
        x = simd_add(vpaddl_u16(b), a);
    };
    x
}
///Unsigned Add and Accumulate Long Pairwise.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vpadalq_u16)
pub fn vpadalq_u16(a: uint32x4_t, b: uint16x8_t) -> uint32x4_t {
    let x: uint32x4_t;
    #[cfg(target_arch = "arm")]
    {
        x = priv_vpadalq_u16(a, b);
    }
    {
        x = simd_add(vpaddlq_u16(b), a);
    };
    x
}
///Unsigned Add and Accumulate Long Pairwise.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vpadal_u32)
pub fn vpadal_u32(a: uint64x1_t, b: uint32x2_t) -> uint64x1_t {
    let x: uint64x1_t;
    #[cfg(target_arch = "arm")]
    {
        x = priv_vpadal_u32(a, b);
    }
    {
        x = simd_add(vpaddl_u32(b), a);
    };
    x
}
///Unsigned Add and Accumulate Long Pairwise.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vpadalq_u32)
pub fn vpadalq_u32(a: uint64x2_t, b: uint32x4_t) -> uint64x2_t {
    let x: uint64x2_t;
    #[cfg(target_arch = "arm")]
    {
        x = priv_vpadalq_u32(a, b);
    }
    {
        x = simd_add(vpaddlq_u32(b), a);
    };
    x
}
///Floating-point add pairwise
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vpadd_f16)
pub fn vpadd_f16(a: float16x4_t, b: float16x4_t) -> float16x4_t {
    { _vpadd_f16(a, b) }
}
///Floating-point add pairwise
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vpadd_f32)
pub fn vpadd_f32(a: float32x2_t, b: float32x2_t) -> float32x2_t {
    { _vpadd_f32(a, b) }
}
///Add pairwise.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vpadd_s8)
pub fn vpadd_s8(a: int8x8_t, b: int8x8_t) -> int8x8_t {
    { _vpadd_s8(a, b) }
}
///Add pairwise.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vpadd_s16)
pub fn vpadd_s16(a: int16x4_t, b: int16x4_t) -> int16x4_t {
    { _vpadd_s16(a, b) }
}
///Add pairwise.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vpadd_s32)
pub fn vpadd_s32(a: int32x2_t, b: int32x2_t) -> int32x2_t {
    { _vpadd_s32(a, b) }
}
///Add pairwise.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vpadd_u8)
pub fn vpadd_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
    { transmute(vpadd_s8(transmute(a), transmute(b))) }
}
///Add pairwise.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vpadd_u8)
pub fn vpadd_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
    let a: uint8x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    let b: uint8x8_t = unsafe { simd_shuffle(b, b, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: uint8x8_t = transmute(vpadd_s8(transmute(a), transmute(b)));
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Add pairwise.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vpadd_u16)
pub fn vpadd_u16(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
    { transmute(vpadd_s16(transmute(a), transmute(b))) }
}
///Add pairwise.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vpadd_u16)
pub fn vpadd_u16(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
    let a: uint16x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    let b: uint16x4_t = unsafe { simd_shuffle(b, b, [3, 2, 1, 0]) };
    {
        let ret_val: uint16x4_t = transmute(vpadd_s16(transmute(a), transmute(b)));
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Add pairwise.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vpadd_u32)
pub fn vpadd_u32(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
    { transmute(vpadd_s32(transmute(a), transmute(b))) }
}
///Add pairwise.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vpadd_u32)
pub fn vpadd_u32(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
    let a: uint32x2_t = unsafe { simd_shuffle(a, a, [1, 0]) };
    let b: uint32x2_t = unsafe { simd_shuffle(b, b, [1, 0]) };
    {
        let ret_val: uint32x2_t = transmute(vpadd_s32(transmute(a), transmute(b)));
        simd_shuffle(ret_val, ret_val, [1, 0])
    }
}
///Signed Add and Accumulate Long Pairwise.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vpaddl_s8)
pub fn vpaddl_s8(a: int8x8_t) -> int16x4_t {
    { _vpaddl_s8(a) }
}
///Signed Add and Accumulate Long Pairwise.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vpaddlq_s8)
pub fn vpaddlq_s8(a: int8x16_t) -> int16x8_t {
    { _vpaddlq_s8(a) }
}
///Signed Add and Accumulate Long Pairwise.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vpaddl_s16)
pub fn vpaddl_s16(a: int16x4_t) -> int32x2_t {
    { _vpaddl_s16(a) }
}
///Signed Add and Accumulate Long Pairwise.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vpaddlq_s16)
pub fn vpaddlq_s16(a: int16x8_t) -> int32x4_t {
    { _vpaddlq_s16(a) }
}
///Signed Add and Accumulate Long Pairwise.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vpaddl_s32)
pub fn vpaddl_s32(a: int32x2_t) -> int64x1_t {
    { _vpaddl_s32(a) }
}
///Signed Add and Accumulate Long Pairwise.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vpaddlq_s32)
pub fn vpaddlq_s32(a: int32x4_t) -> int64x2_t {
    { _vpaddlq_s32(a) }
}
///Unsigned Add and Accumulate Long Pairwise.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vpaddl_u8)
pub fn vpaddl_u8(a: uint8x8_t) -> uint16x4_t {
    { _vpaddl_u8(a) }
}
///Unsigned Add and Accumulate Long Pairwise.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vpaddlq_u8)
pub fn vpaddlq_u8(a: uint8x16_t) -> uint16x8_t {
    { _vpaddlq_u8(a) }
}
///Unsigned Add and Accumulate Long Pairwise.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vpaddl_u16)
pub fn vpaddl_u16(a: uint16x4_t) -> uint32x2_t {
    { _vpaddl_u16(a) }
}
///Unsigned Add and Accumulate Long Pairwise.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vpaddlq_u16)
pub fn vpaddlq_u16(a: uint16x8_t) -> uint32x4_t {
    { _vpaddlq_u16(a) }
}
///Unsigned Add and Accumulate Long Pairwise.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vpaddl_u32)
pub fn vpaddl_u32(a: uint32x2_t) -> uint64x1_t {
    { _vpaddl_u32(a) }
}
///Unsigned Add and Accumulate Long Pairwise.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vpaddlq_u32)
pub fn vpaddlq_u32(a: uint32x4_t) -> uint64x2_t {
    { _vpaddlq_u32(a) }
}
///Folding maximum of adjacent pairs
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vpmax_f32)
pub fn vpmax_f32(a: float32x2_t, b: float32x2_t) -> float32x2_t {
    { _vpmax_f32(a, b) }
}
///Folding maximum of adjacent pairs
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vpmax_s8)
pub fn vpmax_s8(a: int8x8_t, b: int8x8_t) -> int8x8_t {
    { _vpmax_s8(a, b) }
}
///Folding maximum of adjacent pairs
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vpmax_s16)
pub fn vpmax_s16(a: int16x4_t, b: int16x4_t) -> int16x4_t {
    { _vpmax_s16(a, b) }
}
///Folding maximum of adjacent pairs
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vpmax_s32)
pub fn vpmax_s32(a: int32x2_t, b: int32x2_t) -> int32x2_t {
    { _vpmax_s32(a, b) }
}
///Folding maximum of adjacent pairs
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vpmax_u8)
pub fn vpmax_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
    { _vpmax_u8(a, b) }
}
///Folding maximum of adjacent pairs
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vpmax_u16)
pub fn vpmax_u16(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
    { _vpmax_u16(a, b) }
}
///Folding maximum of adjacent pairs
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vpmax_u32)
pub fn vpmax_u32(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
    { _vpmax_u32(a, b) }
}
///Folding minimum of adjacent pairs
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vpmin_f32)
pub fn vpmin_f32(a: float32x2_t, b: float32x2_t) -> float32x2_t {
    { _vpmin_f32(a, b) }
}
///Folding minimum of adjacent pairs
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vpmin_s8)
pub fn vpmin_s8(a: int8x8_t, b: int8x8_t) -> int8x8_t {
    { _vpmin_s8(a, b) }
}
///Folding minimum of adjacent pairs
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vpmin_s16)
pub fn vpmin_s16(a: int16x4_t, b: int16x4_t) -> int16x4_t {
    { _vpmin_s16(a, b) }
}
///Folding minimum of adjacent pairs
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vpmin_s32)
pub fn vpmin_s32(a: int32x2_t, b: int32x2_t) -> int32x2_t {
    { _vpmin_s32(a, b) }
}
///Folding minimum of adjacent pairs
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vpmin_u8)
pub fn vpmin_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
    { _vpmin_u8(a, b) }
}
///Folding minimum of adjacent pairs
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vpmin_u16)
pub fn vpmin_u16(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
    { _vpmin_u16(a, b) }
}
///Folding minimum of adjacent pairs
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vpmin_u32)
pub fn vpmin_u32(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
    { _vpmin_u32(a, b) }
}
///Signed saturating Absolute value
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqabs_s8)
pub fn vqabs_s8(a: int8x8_t) -> int8x8_t {
    { _vqabs_s8(a) }
}
///Signed saturating Absolute value
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqabsq_s8)
pub fn vqabsq_s8(a: int8x16_t) -> int8x16_t {
    { _vqabsq_s8(a) }
}
///Signed saturating Absolute value
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqabs_s16)
pub fn vqabs_s16(a: int16x4_t) -> int16x4_t {
    { _vqabs_s16(a) }
}
///Signed saturating Absolute value
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqabsq_s16)
pub fn vqabsq_s16(a: int16x8_t) -> int16x8_t {
    { _vqabsq_s16(a) }
}
///Signed saturating Absolute value
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqabs_s32)
pub fn vqabs_s32(a: int32x2_t) -> int32x2_t {
    { _vqabs_s32(a) }
}
///Signed saturating Absolute value
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqabsq_s32)
pub fn vqabsq_s32(a: int32x4_t) -> int32x4_t {
    { _vqabsq_s32(a) }
}
///Saturating add
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqadd_s8)
pub fn vqadd_s8(a: int8x8_t, b: int8x8_t) -> int8x8_t {
    { _vqadd_s8(a, b) }
}
///Saturating add
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqaddq_s8)
pub fn vqaddq_s8(a: int8x16_t, b: int8x16_t) -> int8x16_t {
    { _vqaddq_s8(a, b) }
}
///Saturating add
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqadd_s16)
pub fn vqadd_s16(a: int16x4_t, b: int16x4_t) -> int16x4_t {
    { _vqadd_s16(a, b) }
}
///Saturating add
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqaddq_s16)
pub fn vqaddq_s16(a: int16x8_t, b: int16x8_t) -> int16x8_t {
    { _vqaddq_s16(a, b) }
}
///Saturating add
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqadd_s32)
pub fn vqadd_s32(a: int32x2_t, b: int32x2_t) -> int32x2_t {
    { _vqadd_s32(a, b) }
}
///Saturating add
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqaddq_s32)
pub fn vqaddq_s32(a: int32x4_t, b: int32x4_t) -> int32x4_t {
    { _vqaddq_s32(a, b) }
}
///Saturating add
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqadd_s64)
pub fn vqadd_s64(a: int64x1_t, b: int64x1_t) -> int64x1_t {
    { _vqadd_s64(a, b) }
}
///Saturating add
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqaddq_s64)
pub fn vqaddq_s64(a: int64x2_t, b: int64x2_t) -> int64x2_t {
    { _vqaddq_s64(a, b) }
}
///Saturating add
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqadd_u8)
pub fn vqadd_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
    { _vqadd_u8(a, b) }
}
///Saturating add
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqaddq_u8)
pub fn vqaddq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
    { _vqaddq_u8(a, b) }
}
///Saturating add
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqadd_u16)
pub fn vqadd_u16(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
    { _vqadd_u16(a, b) }
}
///Saturating add
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqaddq_u16)
pub fn vqaddq_u16(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
    { _vqaddq_u16(a, b) }
}
///Saturating add
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqadd_u32)
pub fn vqadd_u32(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
    { _vqadd_u32(a, b) }
}
///Saturating add
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqaddq_u32)
pub fn vqaddq_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
    { _vqaddq_u32(a, b) }
}
///Saturating add
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqadd_u64)
pub fn vqadd_u64(a: uint64x1_t, b: uint64x1_t) -> uint64x1_t {
    { _vqadd_u64(a, b) }
}
///Saturating add
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqaddq_u64)
pub fn vqaddq_u64(a: uint64x2_t, b: uint64x2_t) -> uint64x2_t {
    { _vqaddq_u64(a, b) }
}
///Vector widening saturating doubling multiply accumulate with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqdmlal_lane_s16)
pub fn vqdmlal_lane_s16<const N: i32>(
    a: int32x4_t,
    b: int16x4_t,
    c: int16x4_t,
) -> int32x4_t {
    static_assert_uimm_bits!(N, 2);
    vqaddq_s32(a, vqdmull_lane_s16::<N>(b, c))
}
///Vector widening saturating doubling multiply accumulate with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqdmlal_lane_s32)
pub fn vqdmlal_lane_s32<const N: i32>(
    a: int64x2_t,
    b: int32x2_t,
    c: int32x2_t,
) -> int64x2_t {
    static_assert_uimm_bits!(N, 1);
    vqaddq_s64(a, vqdmull_lane_s32::<N>(b, c))
}
///Vector widening saturating doubling multiply accumulate with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqdmlal_n_s16)
pub fn vqdmlal_n_s16(a: int32x4_t, b: int16x4_t, c: i16) -> int32x4_t {
    vqaddq_s32(a, vqdmull_n_s16(b, c))
}
///Vector widening saturating doubling multiply accumulate with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqdmlal_n_s32)
pub fn vqdmlal_n_s32(a: int64x2_t, b: int32x2_t, c: i32) -> int64x2_t {
    vqaddq_s64(a, vqdmull_n_s32(b, c))
}
///Signed saturating doubling multiply-add long
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqdmlal_s16)
pub fn vqdmlal_s16(a: int32x4_t, b: int16x4_t, c: int16x4_t) -> int32x4_t {
    vqaddq_s32(a, vqdmull_s16(b, c))
}
///Signed saturating doubling multiply-add long
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqdmlal_s32)
pub fn vqdmlal_s32(a: int64x2_t, b: int32x2_t, c: int32x2_t) -> int64x2_t {
    vqaddq_s64(a, vqdmull_s32(b, c))
}
///Vector widening saturating doubling multiply subtract with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqdmlsl_lane_s16)
pub fn vqdmlsl_lane_s16<const N: i32>(
    a: int32x4_t,
    b: int16x4_t,
    c: int16x4_t,
) -> int32x4_t {
    static_assert_uimm_bits!(N, 2);
    vqsubq_s32(a, vqdmull_lane_s16::<N>(b, c))
}
///Vector widening saturating doubling multiply subtract with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqdmlsl_lane_s32)
pub fn vqdmlsl_lane_s32<const N: i32>(
    a: int64x2_t,
    b: int32x2_t,
    c: int32x2_t,
) -> int64x2_t {
    static_assert_uimm_bits!(N, 1);
    vqsubq_s64(a, vqdmull_lane_s32::<N>(b, c))
}
///Vector widening saturating doubling multiply subtract with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqdmlsl_n_s16)
pub fn vqdmlsl_n_s16(a: int32x4_t, b: int16x4_t, c: i16) -> int32x4_t {
    vqsubq_s32(a, vqdmull_n_s16(b, c))
}
///Vector widening saturating doubling multiply subtract with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqdmlsl_n_s32)
pub fn vqdmlsl_n_s32(a: int64x2_t, b: int32x2_t, c: i32) -> int64x2_t {
    vqsubq_s64(a, vqdmull_n_s32(b, c))
}
///Signed saturating doubling multiply-subtract long
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqdmlsl_s16)
pub fn vqdmlsl_s16(a: int32x4_t, b: int16x4_t, c: int16x4_t) -> int32x4_t {
    vqsubq_s32(a, vqdmull_s16(b, c))
}
///Signed saturating doubling multiply-subtract long
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqdmlsl_s32)
pub fn vqdmlsl_s32(a: int64x2_t, b: int32x2_t, c: int32x2_t) -> int64x2_t {
    vqsubq_s64(a, vqdmull_s32(b, c))
}
///Vector saturating doubling multiply high by scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqdmulh_laneq_s16)
pub fn vqdmulh_laneq_s16<const LANE: i32>(a: int16x4_t, b: int16x8_t) -> int16x4_t {
    static_assert_uimm_bits!(LANE, 3);
    { vqdmulh_s16(a, vdup_n_s16(simd_extract(b, LANE as u32))) }
}
///Vector saturating doubling multiply high by scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqdmulhq_laneq_s16)
pub fn vqdmulhq_laneq_s16<const LANE: i32>(a: int16x8_t, b: int16x8_t) -> int16x8_t {
    static_assert_uimm_bits!(LANE, 3);
    { vqdmulhq_s16(a, vdupq_n_s16(simd_extract(b, LANE as u32))) }
}
///Vector saturating doubling multiply high by scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqdmulh_laneq_s32)
pub fn vqdmulh_laneq_s32<const LANE: i32>(a: int32x2_t, b: int32x4_t) -> int32x2_t {
    static_assert_uimm_bits!(LANE, 2);
    { vqdmulh_s32(a, vdup_n_s32(simd_extract(b, LANE as u32))) }
}
///Vector saturating doubling multiply high by scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqdmulhq_laneq_s32)
pub fn vqdmulhq_laneq_s32<const LANE: i32>(a: int32x4_t, b: int32x4_t) -> int32x4_t {
    static_assert_uimm_bits!(LANE, 2);
    { vqdmulhq_s32(a, vdupq_n_s32(simd_extract(b, LANE as u32))) }
}
///Vector saturating doubling multiply high with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqdmulh_n_s16)
pub fn vqdmulh_n_s16(a: int16x4_t, b: i16) -> int16x4_t {
    let b: int16x4_t = vdup_n_s16(b);
    vqdmulh_s16(a, b)
}
///Vector saturating doubling multiply high with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqdmulhq_n_s16)
pub fn vqdmulhq_n_s16(a: int16x8_t, b: i16) -> int16x8_t {
    let b: int16x8_t = vdupq_n_s16(b);
    vqdmulhq_s16(a, b)
}
///Vector saturating doubling multiply high with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqdmulh_n_s32)
pub fn vqdmulh_n_s32(a: int32x2_t, b: i32) -> int32x2_t {
    let b: int32x2_t = vdup_n_s32(b);
    vqdmulh_s32(a, b)
}
///Vector saturating doubling multiply high with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqdmulhq_n_s32)
pub fn vqdmulhq_n_s32(a: int32x4_t, b: i32) -> int32x4_t {
    let b: int32x4_t = vdupq_n_s32(b);
    vqdmulhq_s32(a, b)
}
///Signed saturating doubling multiply returning high half
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqdmulh_s16)
pub fn vqdmulh_s16(a: int16x4_t, b: int16x4_t) -> int16x4_t {
    { _vqdmulh_s16(a, b) }
}
///Signed saturating doubling multiply returning high half
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqdmulhq_s16)
pub fn vqdmulhq_s16(a: int16x8_t, b: int16x8_t) -> int16x8_t {
    { _vqdmulhq_s16(a, b) }
}
///Signed saturating doubling multiply returning high half
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqdmulh_s32)
pub fn vqdmulh_s32(a: int32x2_t, b: int32x2_t) -> int32x2_t {
    { _vqdmulh_s32(a, b) }
}
///Signed saturating doubling multiply returning high half
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqdmulhq_s32)
pub fn vqdmulhq_s32(a: int32x4_t, b: int32x4_t) -> int32x4_t {
    { _vqdmulhq_s32(a, b) }
}
///Vector saturating doubling long multiply by scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqdmull_lane_s16)
pub fn vqdmull_lane_s16<const N: i32>(a: int16x4_t, b: int16x4_t) -> int32x4_t {
    static_assert_uimm_bits!(N, 2);
    {
        let b: int16x4_t = simd_shuffle(b, b, [N as u32, N as u32, N as u32, N as u32]);
        vqdmull_s16(a, b)
    }
}
///Vector saturating doubling long multiply by scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqdmull_lane_s32)
pub fn vqdmull_lane_s32<const N: i32>(a: int32x2_t, b: int32x2_t) -> int64x2_t {
    static_assert_uimm_bits!(N, 1);
    {
        let b: int32x2_t = simd_shuffle(b, b, [N as u32, N as u32]);
        vqdmull_s32(a, b)
    }
}
///Vector saturating doubling long multiply with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqdmull_n_s16)
pub fn vqdmull_n_s16(a: int16x4_t, b: i16) -> int32x4_t {
    vqdmull_s16(a, vdup_n_s16(b))
}
///Vector saturating doubling long multiply with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqdmull_n_s32)
pub fn vqdmull_n_s32(a: int32x2_t, b: i32) -> int64x2_t {
    vqdmull_s32(a, vdup_n_s32(b))
}
///Signed saturating doubling multiply long
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqdmull_s16)
pub fn vqdmull_s16(a: int16x4_t, b: int16x4_t) -> int32x4_t {
    { _vqdmull_s16(a, b) }
}
///Signed saturating doubling multiply long
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqdmull_s32)
pub fn vqdmull_s32(a: int32x2_t, b: int32x2_t) -> int64x2_t {
    { _vqdmull_s32(a, b) }
}
///Signed saturating extract narrow
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqmovn_s16)
pub fn vqmovn_s16(a: int16x8_t) -> int8x8_t {
    { _vqmovn_s16(a) }
}
///Signed saturating extract narrow
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqmovn_s32)
pub fn vqmovn_s32(a: int32x4_t) -> int16x4_t {
    { _vqmovn_s32(a) }
}
///Signed saturating extract narrow
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqmovn_s64)
pub fn vqmovn_s64(a: int64x2_t) -> int32x2_t {
    { _vqmovn_s64(a) }
}
///Unsigned saturating extract narrow
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqmovn_u16)
pub fn vqmovn_u16(a: uint16x8_t) -> uint8x8_t {
    { _vqmovn_u16(a) }
}
///Unsigned saturating extract narrow
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqmovn_u32)
pub fn vqmovn_u32(a: uint32x4_t) -> uint16x4_t {
    { _vqmovn_u32(a) }
}
///Unsigned saturating extract narrow
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqmovn_u64)
pub fn vqmovn_u64(a: uint64x2_t) -> uint32x2_t {
    { _vqmovn_u64(a) }
}
///Signed saturating extract unsigned narrow
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqmovun_s16)
pub fn vqmovun_s16(a: int16x8_t) -> uint8x8_t {
    { _vqmovun_s16(a) }
}
///Signed saturating extract unsigned narrow
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqmovun_s32)
pub fn vqmovun_s32(a: int32x4_t) -> uint16x4_t {
    { _vqmovun_s32(a) }
}
///Signed saturating extract unsigned narrow
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqmovun_s64)
pub fn vqmovun_s64(a: int64x2_t) -> uint32x2_t {
    { _vqmovun_s64(a) }
}
///Signed saturating negate
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqneg_s8)
pub fn vqneg_s8(a: int8x8_t) -> int8x8_t {
    { _vqneg_s8(a) }
}
///Signed saturating negate
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqnegq_s8)
pub fn vqnegq_s8(a: int8x16_t) -> int8x16_t {
    { _vqnegq_s8(a) }
}
///Signed saturating negate
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqneg_s16)
pub fn vqneg_s16(a: int16x4_t) -> int16x4_t {
    { _vqneg_s16(a) }
}
///Signed saturating negate
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqnegq_s16)
pub fn vqnegq_s16(a: int16x8_t) -> int16x8_t {
    { _vqnegq_s16(a) }
}
///Signed saturating negate
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqneg_s32)
pub fn vqneg_s32(a: int32x2_t) -> int32x2_t {
    { _vqneg_s32(a) }
}
///Signed saturating negate
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqnegq_s32)
pub fn vqnegq_s32(a: int32x4_t) -> int32x4_t {
    { _vqnegq_s32(a) }
}
///Vector rounding saturating doubling multiply high by scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqrdmulh_lane_s16)
pub fn vqrdmulh_lane_s16<const LANE: i32>(a: int16x4_t, b: int16x4_t) -> int16x4_t {
    static_assert_uimm_bits!(LANE, 2);
    {
        let b: int16x4_t = simd_shuffle(
            b, b, [LANE as u32, LANE as u32, LANE as u32, LANE as u32]
        );
        vqrdmulh_s16(a, b)
    }
}
///Vector rounding saturating doubling multiply high by scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqrdmulh_lane_s32)
pub fn vqrdmulh_lane_s32<const LANE: i32>(a: int32x2_t, b: int32x2_t) -> int32x2_t {
    static_assert_uimm_bits!(LANE, 1);
    {
        let b: int32x2_t = simd_shuffle(b, b, [LANE as u32, LANE as u32]);
        vqrdmulh_s32(a, b)
    }
}
///Vector rounding saturating doubling multiply high by scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqrdmulh_laneq_s16)
pub fn vqrdmulh_laneq_s16<const LANE: i32>(a: int16x4_t, b: int16x8_t) -> int16x4_t {
    static_assert_uimm_bits!(LANE, 3);
    {
        let b: int16x4_t = simd_shuffle(
            b, b, [LANE as u32, LANE as u32, LANE as u32, LANE as u32]
        );
        vqrdmulh_s16(a, b)
    }
}
///Vector rounding saturating doubling multiply high by scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqrdmulh_laneq_s32)
pub fn vqrdmulh_laneq_s32<const LANE: i32>(a: int32x2_t, b: int32x4_t) -> int32x2_t {
    static_assert_uimm_bits!(LANE, 2);
    {
        let b: int32x2_t = simd_shuffle(b, b, [LANE as u32, LANE as u32]);
        vqrdmulh_s32(a, b)
    }
}
///Vector rounding saturating doubling multiply high by scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqrdmulhq_lane_s16)
pub fn vqrdmulhq_lane_s16<const LANE: i32>(a: int16x8_t, b: int16x4_t) -> int16x8_t {
    static_assert_uimm_bits!(LANE, 2);
    {
        let b: int16x8_t = simd_shuffle(
            b, b, [LANE as u32, LANE as u32, LANE as u32, LANE as u32, LANE as u32, LANE
            as u32, LANE as u32, LANE as u32]
        );
        vqrdmulhq_s16(a, b)
    }
}
///Vector rounding saturating doubling multiply high by scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqrdmulhq_lane_s32)
pub fn vqrdmulhq_lane_s32<const LANE: i32>(a: int32x4_t, b: int32x2_t) -> int32x4_t {
    static_assert_uimm_bits!(LANE, 1);
    {
        let b: int32x4_t = simd_shuffle(
            b, b, [LANE as u32, LANE as u32, LANE as u32, LANE as u32]
        );
        vqrdmulhq_s32(a, b)
    }
}
///Vector rounding saturating doubling multiply high by scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqrdmulhq_laneq_s16)
pub fn vqrdmulhq_laneq_s16<const LANE: i32>(a: int16x8_t, b: int16x8_t) -> int16x8_t {
    static_assert_uimm_bits!(LANE, 3);
    {
        let b: int16x8_t = simd_shuffle(
            b, b, [LANE as u32, LANE as u32, LANE as u32, LANE as u32, LANE as u32, LANE
            as u32, LANE as u32, LANE as u32]
        );
        vqrdmulhq_s16(a, b)
    }
}
///Vector rounding saturating doubling multiply high by scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqrdmulhq_laneq_s32)
pub fn vqrdmulhq_laneq_s32<const LANE: i32>(a: int32x4_t, b: int32x4_t) -> int32x4_t {
    static_assert_uimm_bits!(LANE, 2);
    {
        let b: int32x4_t = simd_shuffle(
            b, b, [LANE as u32, LANE as u32, LANE as u32, LANE as u32]
        );
        vqrdmulhq_s32(a, b)
    }
}
///Vector saturating rounding doubling multiply high with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqrdmulh_n_s16)
pub fn vqrdmulh_n_s16(a: int16x4_t, b: i16) -> int16x4_t {
    vqrdmulh_s16(a, vdup_n_s16(b))
}
///Vector saturating rounding doubling multiply high with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqrdmulhq_n_s16)
pub fn vqrdmulhq_n_s16(a: int16x8_t, b: i16) -> int16x8_t {
    vqrdmulhq_s16(a, vdupq_n_s16(b))
}
///Vector saturating rounding doubling multiply high with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqrdmulh_n_s32)
pub fn vqrdmulh_n_s32(a: int32x2_t, b: i32) -> int32x2_t {
    vqrdmulh_s32(a, vdup_n_s32(b))
}
///Vector saturating rounding doubling multiply high with scalar
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqrdmulhq_n_s32)
pub fn vqrdmulhq_n_s32(a: int32x4_t, b: i32) -> int32x4_t {
    vqrdmulhq_s32(a, vdupq_n_s32(b))
}
///Signed saturating rounding doubling multiply returning high half
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqrdmulh_s16)
pub fn vqrdmulh_s16(a: int16x4_t, b: int16x4_t) -> int16x4_t {
    { _vqrdmulh_s16(a, b) }
}
///Signed saturating rounding doubling multiply returning high half
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqrdmulhq_s16)
pub fn vqrdmulhq_s16(a: int16x8_t, b: int16x8_t) -> int16x8_t {
    { _vqrdmulhq_s16(a, b) }
}
///Signed saturating rounding doubling multiply returning high half
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqrdmulh_s32)
pub fn vqrdmulh_s32(a: int32x2_t, b: int32x2_t) -> int32x2_t {
    { _vqrdmulh_s32(a, b) }
}
///Signed saturating rounding doubling multiply returning high half
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqrdmulhq_s32)
pub fn vqrdmulhq_s32(a: int32x4_t, b: int32x4_t) -> int32x4_t {
    { _vqrdmulhq_s32(a, b) }
}
///Signed saturating rounding shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqrshl_s8)
pub fn vqrshl_s8(a: int8x8_t, b: int8x8_t) -> int8x8_t {
    { _vqrshl_s8(a, b) }
}
///Signed saturating rounding shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqrshlq_s8)
pub fn vqrshlq_s8(a: int8x16_t, b: int8x16_t) -> int8x16_t {
    { _vqrshlq_s8(a, b) }
}
///Signed saturating rounding shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqrshl_s16)
pub fn vqrshl_s16(a: int16x4_t, b: int16x4_t) -> int16x4_t {
    { _vqrshl_s16(a, b) }
}
///Signed saturating rounding shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqrshlq_s16)
pub fn vqrshlq_s16(a: int16x8_t, b: int16x8_t) -> int16x8_t {
    { _vqrshlq_s16(a, b) }
}
///Signed saturating rounding shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqrshl_s32)
pub fn vqrshl_s32(a: int32x2_t, b: int32x2_t) -> int32x2_t {
    { _vqrshl_s32(a, b) }
}
///Signed saturating rounding shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqrshlq_s32)
pub fn vqrshlq_s32(a: int32x4_t, b: int32x4_t) -> int32x4_t {
    { _vqrshlq_s32(a, b) }
}
///Signed saturating rounding shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqrshl_s64)
pub fn vqrshl_s64(a: int64x1_t, b: int64x1_t) -> int64x1_t {
    { _vqrshl_s64(a, b) }
}
///Signed saturating rounding shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqrshlq_s64)
pub fn vqrshlq_s64(a: int64x2_t, b: int64x2_t) -> int64x2_t {
    { _vqrshlq_s64(a, b) }
}
///Unsigned signed saturating rounding shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqrshl_u8)
pub fn vqrshl_u8(a: uint8x8_t, b: int8x8_t) -> uint8x8_t {
    { _vqrshl_u8(a, b) }
}
///Unsigned signed saturating rounding shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqrshlq_u8)
pub fn vqrshlq_u8(a: uint8x16_t, b: int8x16_t) -> uint8x16_t {
    { _vqrshlq_u8(a, b) }
}
///Unsigned signed saturating rounding shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqrshl_u16)
pub fn vqrshl_u16(a: uint16x4_t, b: int16x4_t) -> uint16x4_t {
    { _vqrshl_u16(a, b) }
}
///Unsigned signed saturating rounding shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqrshlq_u16)
pub fn vqrshlq_u16(a: uint16x8_t, b: int16x8_t) -> uint16x8_t {
    { _vqrshlq_u16(a, b) }
}
///Unsigned signed saturating rounding shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqrshl_u32)
pub fn vqrshl_u32(a: uint32x2_t, b: int32x2_t) -> uint32x2_t {
    { _vqrshl_u32(a, b) }
}
///Unsigned signed saturating rounding shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqrshlq_u32)
pub fn vqrshlq_u32(a: uint32x4_t, b: int32x4_t) -> uint32x4_t {
    { _vqrshlq_u32(a, b) }
}
///Unsigned signed saturating rounding shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqrshl_u64)
pub fn vqrshl_u64(a: uint64x1_t, b: int64x1_t) -> uint64x1_t {
    { _vqrshl_u64(a, b) }
}
///Unsigned signed saturating rounding shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqrshlq_u64)
pub fn vqrshlq_u64(a: uint64x2_t, b: int64x2_t) -> uint64x2_t {
    { _vqrshlq_u64(a, b) }
}
///Signed saturating rounded shift right narrow
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqrshrn_n_s16)
pub fn vqrshrn_n_s16<const N: i32>(a: int16x8_t) -> int8x8_t {
    static_assert!(N >= 1 && N <= 8);
    {
        _vqrshrn_n_s16(
            a,
            const {
                int16x8_t([
                    -N as i16,
                    -N as i16,
                    -N as i16,
                    -N as i16,
                    -N as i16,
                    -N as i16,
                    -N as i16,
                    -N as i16,
                ])
            },
        )
    }
}
///Signed saturating rounded shift right narrow
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqrshrn_n_s32)
pub fn vqrshrn_n_s32<const N: i32>(a: int32x4_t) -> int16x4_t {
    static_assert!(N >= 1 && N <= 16);
    {
        _vqrshrn_n_s32(
            a,
            const { int32x4_t([-N as i32, -N as i32, -N as i32, -N as i32]) },
        )
    }
}
///Signed saturating rounded shift right narrow
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqrshrn_n_s64)
pub fn vqrshrn_n_s64<const N: i32>(a: int64x2_t) -> int32x2_t {
    static_assert!(N >= 1 && N <= 32);
    { _vqrshrn_n_s64(a, const { int64x2_t([-N as i64, -N as i64]) }) }
}

///Unsigned signed saturating rounded shift right narrow
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqrshrn_n_u16)
pub fn vqrshrn_n_u16<const N: i32>(a: uint16x8_t) -> uint8x8_t {
    static_assert!(N >= 1 && N <= 8);
    {
        _vqrshrn_n_u16(
            a,
            const {
                uint16x8_t([
                    -N as u16,
                    -N as u16,
                    -N as u16,
                    -N as u16,
                    -N as u16,
                    -N as u16,
                    -N as u16,
                    -N as u16,
                ])
            },
        )
    }
}
///Unsigned signed saturating rounded shift right narrow
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqrshrn_n_u32)
pub fn vqrshrn_n_u32<const N: i32>(a: uint32x4_t) -> uint16x4_t {
    static_assert!(N >= 1 && N <= 16);
    {
        _vqrshrn_n_u32(
            a,
            const { uint32x4_t([-N as u32, -N as u32, -N as u32, -N as u32]) },
        )
    }
}
///Unsigned signed saturating rounded shift right narrow
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqrshrn_n_u64)
pub fn vqrshrn_n_u64<const N: i32>(a: uint64x2_t) -> uint32x2_t {
    static_assert!(N >= 1 && N <= 32);
    { _vqrshrn_n_u64(a, const { uint64x2_t([-N as u64, -N as u64]) }) }
}

///Signed saturating rounded shift right unsigned narrow
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqrshrun_n_s16)
pub fn vqrshrun_n_s16<const N: i32>(a: int16x8_t) -> uint8x8_t {
    static_assert!(N >= 1 && N <= 8);
    {
        _vqrshrun_n_s16(
            a,
            const {
                int16x8_t([
                    -N as i16,
                    -N as i16,
                    -N as i16,
                    -N as i16,
                    -N as i16,
                    -N as i16,
                    -N as i16,
                    -N as i16,
                ])
            },
        )
    }
}
///Signed saturating rounded shift right unsigned narrow
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqrshrun_n_s32)
pub fn vqrshrun_n_s32<const N: i32>(a: int32x4_t) -> uint16x4_t {
    static_assert!(N >= 1 && N <= 16);
    {
        _vqrshrun_n_s32(
            a,
            const { int32x4_t([-N as i32, -N as i32, -N as i32, -N as i32]) },
        )
    }
}
///Signed saturating rounded shift right unsigned narrow
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqrshrun_n_s64)
pub fn vqrshrun_n_s64<const N: i32>(a: int64x2_t) -> uint32x2_t {
    static_assert!(N >= 1 && N <= 32);
    { _vqrshrun_n_s64(a, const { int64x2_t([-N as i64, -N as i64]) }) }
}

///Signed saturating shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqshl_n_s8)
pub fn vqshl_n_s8<const N: i32>(a: int8x8_t) -> int8x8_t {
    static_assert_uimm_bits!(N, 3);
    vqshl_s8(a, vdup_n_s8(N as _))
}
///Signed saturating shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqshlq_n_s8)
pub fn vqshlq_n_s8<const N: i32>(a: int8x16_t) -> int8x16_t {
    static_assert_uimm_bits!(N, 3);
    vqshlq_s8(a, vdupq_n_s8(N as _))
}
///Signed saturating shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqshl_n_s16)
pub fn vqshl_n_s16<const N: i32>(a: int16x4_t) -> int16x4_t {
    static_assert_uimm_bits!(N, 4);
    vqshl_s16(a, vdup_n_s16(N as _))
}
///Signed saturating shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqshlq_n_s16)
pub fn vqshlq_n_s16<const N: i32>(a: int16x8_t) -> int16x8_t {
    static_assert_uimm_bits!(N, 4);
    vqshlq_s16(a, vdupq_n_s16(N as _))
}
///Signed saturating shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqshl_n_s32)
pub fn vqshl_n_s32<const N: i32>(a: int32x2_t) -> int32x2_t {
    static_assert_uimm_bits!(N, 5);
    vqshl_s32(a, vdup_n_s32(N as _))
}
///Signed saturating shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqshlq_n_s32)
pub fn vqshlq_n_s32<const N: i32>(a: int32x4_t) -> int32x4_t {
    static_assert_uimm_bits!(N, 5);
    vqshlq_s32(a, vdupq_n_s32(N as _))
}
///Signed saturating shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqshl_n_s64)
pub fn vqshl_n_s64<const N: i32>(a: int64x1_t) -> int64x1_t {
    static_assert_uimm_bits!(N, 6);
    vqshl_s64(a, vdup_n_s64(N as _))
}
///Signed saturating shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqshlq_n_s64)
pub fn vqshlq_n_s64<const N: i32>(a: int64x2_t) -> int64x2_t {
    static_assert_uimm_bits!(N, 6);
    vqshlq_s64(a, vdupq_n_s64(N as _))
}
///Unsigned saturating shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqshl_n_u8)
pub fn vqshl_n_u8<const N: i32>(a: uint8x8_t) -> uint8x8_t {
    static_assert_uimm_bits!(N, 3);
    vqshl_u8(a, vdup_n_s8(N as _))
}
///Unsigned saturating shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqshlq_n_u8)
pub fn vqshlq_n_u8<const N: i32>(a: uint8x16_t) -> uint8x16_t {
    static_assert_uimm_bits!(N, 3);
    vqshlq_u8(a, vdupq_n_s8(N as _))
}
///Unsigned saturating shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqshl_n_u16)
pub fn vqshl_n_u16<const N: i32>(a: uint16x4_t) -> uint16x4_t {
    static_assert_uimm_bits!(N, 4);
    vqshl_u16(a, vdup_n_s16(N as _))
}
///Unsigned saturating shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqshlq_n_u16)
pub fn vqshlq_n_u16<const N: i32>(a: uint16x8_t) -> uint16x8_t {
    static_assert_uimm_bits!(N, 4);
    vqshlq_u16(a, vdupq_n_s16(N as _))
}
///Unsigned saturating shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqshl_n_u32)
pub fn vqshl_n_u32<const N: i32>(a: uint32x2_t) -> uint32x2_t {
    static_assert_uimm_bits!(N, 5);
    vqshl_u32(a, vdup_n_s32(N as _))
}
///Unsigned saturating shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqshlq_n_u32)
pub fn vqshlq_n_u32<const N: i32>(a: uint32x4_t) -> uint32x4_t {
    static_assert_uimm_bits!(N, 5);
    vqshlq_u32(a, vdupq_n_s32(N as _))
}
///Unsigned saturating shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqshl_n_u64)
pub fn vqshl_n_u64<const N: i32>(a: uint64x1_t) -> uint64x1_t {
    static_assert_uimm_bits!(N, 6);
    vqshl_u64(a, vdup_n_s64(N as _))
}
///Unsigned saturating shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqshlq_n_u64)
pub fn vqshlq_n_u64<const N: i32>(a: uint64x2_t) -> uint64x2_t {
    static_assert_uimm_bits!(N, 6);
    vqshlq_u64(a, vdupq_n_s64(N as _))
}
///Signed saturating shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqshl_s8)
pub fn vqshl_s8(a: int8x8_t, b: int8x8_t) -> int8x8_t {
    { _vqshl_s8(a, b) }
}
///Signed saturating shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqshlq_s8)
pub fn vqshlq_s8(a: int8x16_t, b: int8x16_t) -> int8x16_t {
    { _vqshlq_s8(a, b) }
}
///Signed saturating shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqshl_s16)
pub fn vqshl_s16(a: int16x4_t, b: int16x4_t) -> int16x4_t {
    { _vqshl_s16(a, b) }
}
///Signed saturating shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqshlq_s16)
pub fn vqshlq_s16(a: int16x8_t, b: int16x8_t) -> int16x8_t {
    { _vqshlq_s16(a, b) }
}
///Signed saturating shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqshl_s32)
pub fn vqshl_s32(a: int32x2_t, b: int32x2_t) -> int32x2_t {
    { _vqshl_s32(a, b) }
}
///Signed saturating shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqshlq_s32)
pub fn vqshlq_s32(a: int32x4_t, b: int32x4_t) -> int32x4_t {
    { _vqshlq_s32(a, b) }
}
///Signed saturating shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqshl_s64)
pub fn vqshl_s64(a: int64x1_t, b: int64x1_t) -> int64x1_t {
    { _vqshl_s64(a, b) }
}
///Signed saturating shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqshlq_s64)
pub fn vqshlq_s64(a: int64x2_t, b: int64x2_t) -> int64x2_t {
    { _vqshlq_s64(a, b) }
}
///Unsigned saturating shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqshl_u8)
pub fn vqshl_u8(a: uint8x8_t, b: int8x8_t) -> uint8x8_t {
    { _vqshl_u8(a, b) }
}
///Unsigned saturating shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqshlq_u8)
pub fn vqshlq_u8(a: uint8x16_t, b: int8x16_t) -> uint8x16_t {
    { _vqshlq_u8(a, b) }
}
///Unsigned saturating shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqshl_u16)
pub fn vqshl_u16(a: uint16x4_t, b: int16x4_t) -> uint16x4_t {
    { _vqshl_u16(a, b) }
}
///Unsigned saturating shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqshlq_u16)
pub fn vqshlq_u16(a: uint16x8_t, b: int16x8_t) -> uint16x8_t {
    { _vqshlq_u16(a, b) }
}
///Unsigned saturating shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqshl_u32)
pub fn vqshl_u32(a: uint32x2_t, b: int32x2_t) -> uint32x2_t {
    { _vqshl_u32(a, b) }
}
///Unsigned saturating shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqshlq_u32)
pub fn vqshlq_u32(a: uint32x4_t, b: int32x4_t) -> uint32x4_t {
    { _vqshlq_u32(a, b) }
}
///Unsigned saturating shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqshl_u64)
pub fn vqshl_u64(a: uint64x1_t, b: int64x1_t) -> uint64x1_t {
    { _vqshl_u64(a, b) }
}
///Unsigned saturating shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqshlq_u64)
pub fn vqshlq_u64(a: uint64x2_t, b: int64x2_t) -> uint64x2_t {
    { _vqshlq_u64(a, b) }
}
///Signed saturating shift left unsigned
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqshlu_n_s8)
pub fn vqshlu_n_s8<const N: i32>(a: int8x8_t) -> uint8x8_t {
    static_assert_uimm_bits!(N, 3);
    { _vqshlu_n_s8(a, const { int8x8_t([N as i8; 8]) }) }
}
///Signed saturating shift left unsigned
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqshluq_n_s8)
pub fn vqshluq_n_s8<const N: i32>(a: int8x16_t) -> uint8x16_t {
    static_assert_uimm_bits!(N, 3);
    { _vqshluq_n_s8(a, const { int8x16_t([N as i8; 16]) }) }
}
///Signed saturating shift left unsigned
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqshlu_n_s16)
pub fn vqshlu_n_s16<const N: i32>(a: int16x4_t) -> uint16x4_t {
    static_assert_uimm_bits!(N, 4);
    { _vqshlu_n_s16(a, const { int16x4_t([N as i16; 4]) }) }
}
///Signed saturating shift left unsigned
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqshluq_n_s16)
pub fn vqshluq_n_s16<const N: i32>(a: int16x8_t) -> uint16x8_t {
    static_assert_uimm_bits!(N, 4);
    { _vqshluq_n_s16(a, const { int16x8_t([N as i16; 8]) }) }
}
///Signed saturating shift left unsigned
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqshlu_n_s32)
pub fn vqshlu_n_s32<const N: i32>(a: int32x2_t) -> uint32x2_t {
    static_assert_uimm_bits!(N, 5);
    { _vqshlu_n_s32(a, const { int32x2_t([N; 2]) }) }
}
///Signed saturating shift left unsigned
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqshluq_n_s32)
pub fn vqshluq_n_s32<const N: i32>(a: int32x4_t) -> uint32x4_t {
    static_assert_uimm_bits!(N, 5);
    { _vqshluq_n_s32(a, const { int32x4_t([N; 4]) }) }
}
///Signed saturating shift left unsigned
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqshlu_n_s64)
pub fn vqshlu_n_s64<const N: i32>(a: int64x1_t) -> uint64x1_t {
    static_assert_uimm_bits!(N, 6);
    { _vqshlu_n_s64(a, const { int64x1_t([N as i64]) }) }
}
///Signed saturating shift left unsigned
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqshluq_n_s64)
pub fn vqshluq_n_s64<const N: i32>(a: int64x2_t) -> uint64x2_t {
    static_assert_uimm_bits!(N, 6);
    { _vqshluq_n_s64(a, const { int64x2_t([N as i64; 2]) }) }
}

///Signed saturating shift right narrow
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqshrn_n_s16)
pub fn vqshrn_n_s16<const N: i32>(a: int16x8_t) -> int8x8_t {
    static_assert!(N >= 1 && N <= 8);
    {
        _vqshrn_n_s16(
            a,
            const {
                int16x8_t([
                    -N as i16,
                    -N as i16,
                    -N as i16,
                    -N as i16,
                    -N as i16,
                    -N as i16,
                    -N as i16,
                    -N as i16,
                ])
            },
        )
    }
}
///Signed saturating shift right narrow
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqshrn_n_s32)
pub fn vqshrn_n_s32<const N: i32>(a: int32x4_t) -> int16x4_t {
    static_assert!(N >= 1 && N <= 16);
    {
        _vqshrn_n_s32(
            a,
            const { int32x4_t([-N as i32, -N as i32, -N as i32, -N as i32]) },
        )
    }
}
///Signed saturating shift right narrow
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqshrn_n_s64)
pub fn vqshrn_n_s64<const N: i32>(a: int64x2_t) -> int32x2_t {
    static_assert!(N >= 1 && N <= 32);
    { _vqshrn_n_s64(a, const { int64x2_t([-N as i64, -N as i64]) }) }
}

///Unsigned saturating shift right narrow
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqshrn_n_u16)
pub fn vqshrn_n_u16<const N: i32>(a: uint16x8_t) -> uint8x8_t {
    static_assert!(N >= 1 && N <= 8);
    {
        _vqshrn_n_u16(
            a,
            const {
                uint16x8_t([
                    -N as u16,
                    -N as u16,
                    -N as u16,
                    -N as u16,
                    -N as u16,
                    -N as u16,
                    -N as u16,
                    -N as u16,
                ])
            },
        )
    }
}
///Unsigned saturating shift right narrow
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqshrn_n_u32)
pub fn vqshrn_n_u32<const N: i32>(a: uint32x4_t) -> uint16x4_t {
    static_assert!(N >= 1 && N <= 16);
    {
        _vqshrn_n_u32(
            a,
            const { uint32x4_t([-N as u32, -N as u32, -N as u32, -N as u32]) },
        )
    }
}
///Unsigned saturating shift right narrow
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqshrn_n_u64)
pub fn vqshrn_n_u64<const N: i32>(a: uint64x2_t) -> uint32x2_t {
    static_assert!(N >= 1 && N <= 32);
    { _vqshrn_n_u64(a, const { uint64x2_t([-N as u64, -N as u64]) }) }
}

///Signed saturating shift right unsigned narrow
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqshrun_n_s16)
pub fn vqshrun_n_s16<const N: i32>(a: int16x8_t) -> uint8x8_t {
    static_assert!(N >= 1 && N <= 8);
    {
        _vqshrun_n_s16(
            a,
            const {
                int16x8_t([
                    -N as i16,
                    -N as i16,
                    -N as i16,
                    -N as i16,
                    -N as i16,
                    -N as i16,
                    -N as i16,
                    -N as i16,
                ])
            },
        )
    }
}
///Signed saturating shift right unsigned narrow
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqshrun_n_s32)
pub fn vqshrun_n_s32<const N: i32>(a: int32x4_t) -> uint16x4_t {
    static_assert!(N >= 1 && N <= 16);
    {
        _vqshrun_n_s32(
            a,
            const { int32x4_t([-N as i32, -N as i32, -N as i32, -N as i32]) },
        )
    }
}
///Signed saturating shift right unsigned narrow
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqshrun_n_s64)
pub fn vqshrun_n_s64<const N: i32>(a: int64x2_t) -> uint32x2_t {
    static_assert!(N >= 1 && N <= 32);
    { _vqshrun_n_s64(a, const { int64x2_t([-N as i64, -N as i64]) }) }
}

///Saturating subtract
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqsub_s8)
pub fn vqsub_s8(a: int8x8_t, b: int8x8_t) -> int8x8_t {
    { _vqsub_s8(a, b) }
}
///Saturating subtract
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqsubq_s8)
pub fn vqsubq_s8(a: int8x16_t, b: int8x16_t) -> int8x16_t {
    { _vqsubq_s8(a, b) }
}
///Saturating subtract
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqsub_s16)
pub fn vqsub_s16(a: int16x4_t, b: int16x4_t) -> int16x4_t {
    { _vqsub_s16(a, b) }
}
///Saturating subtract
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqsubq_s16)
pub fn vqsubq_s16(a: int16x8_t, b: int16x8_t) -> int16x8_t {
    { _vqsubq_s16(a, b) }
}
///Saturating subtract
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqsub_s32)
pub fn vqsub_s32(a: int32x2_t, b: int32x2_t) -> int32x2_t {
    { _vqsub_s32(a, b) }
}
///Saturating subtract
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqsubq_s32)
pub fn vqsubq_s32(a: int32x4_t, b: int32x4_t) -> int32x4_t {
    { _vqsubq_s32(a, b) }
}
///Saturating subtract
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqsub_s64)
pub fn vqsub_s64(a: int64x1_t, b: int64x1_t) -> int64x1_t {
    { _vqsub_s64(a, b) }
}
///Saturating subtract
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqsubq_s64)
pub fn vqsubq_s64(a: int64x2_t, b: int64x2_t) -> int64x2_t {
    { _vqsubq_s64(a, b) }
}
///Saturating subtract
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqsub_u8)
pub fn vqsub_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
    { _vqsub_u8(a, b) }
}
///Saturating subtract
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqsubq_u8)
pub fn vqsubq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
    { _vqsubq_u8(a, b) }
}
///Saturating subtract
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqsub_u16)
pub fn vqsub_u16(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
    { _vqsub_u16(a, b) }
}
///Saturating subtract
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqsubq_u16)
pub fn vqsubq_u16(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
    { _vqsubq_u16(a, b) }
}
///Saturating subtract
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqsub_u32)
pub fn vqsub_u32(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
    { _vqsub_u32(a, b) }
}
///Saturating subtract
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqsubq_u32)
pub fn vqsubq_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
    { _vqsubq_u32(a, b) }
}
///Saturating subtract
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqsub_u64)
pub fn vqsub_u64(a: uint64x1_t, b: uint64x1_t) -> uint64x1_t {
    { _vqsub_u64(a, b) }
}
///Saturating subtract
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vqsubq_u64)
pub fn vqsubq_u64(a: uint64x2_t, b: uint64x2_t) -> uint64x2_t {
    { _vqsubq_u64(a, b) }
}

///Rounding Add returning High Narrow (high half).
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vraddhn_high_s16)
pub fn vraddhn_high_s16(a: int8x8_t, b: int16x8_t, c: int16x8_t) -> int8x16_t {
    let x = vraddhn_s16(b, c);
    { simd_shuffle(a, x, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]) }
}
///Rounding Add returning High Narrow (high half).
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vraddhn_high_s32)
pub fn vraddhn_high_s32(a: int16x4_t, b: int32x4_t, c: int32x4_t) -> int16x8_t {
    let x = vraddhn_s32(b, c);
    { simd_shuffle(a, x, [0, 1, 2, 3, 4, 5, 6, 7]) }
}
///Rounding Add returning High Narrow (high half).
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vraddhn_high_s64)
pub fn vraddhn_high_s64(a: int32x2_t, b: int64x2_t, c: int64x2_t) -> int32x4_t {
    let x = vraddhn_s64(b, c);
    { simd_shuffle(a, x, [0, 1, 2, 3]) }
}
///Rounding Add returning High Narrow (high half).
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vraddhn_high_u16)
pub fn vraddhn_high_u16(a: uint8x8_t, b: uint16x8_t, c: uint16x8_t) -> uint8x16_t {
    {
        let x: uint8x8_t = transmute(vraddhn_s16(transmute(b), transmute(c)));
        simd_shuffle(a, x, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15])
    }
}
///Rounding Add returning High Narrow (high half).
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vraddhn_high_u32)
pub fn vraddhn_high_u32(a: uint16x4_t, b: uint32x4_t, c: uint32x4_t) -> uint16x8_t {
    {
        let x: uint16x4_t = transmute(vraddhn_s32(transmute(b), transmute(c)));
        simd_shuffle(a, x, [0, 1, 2, 3, 4, 5, 6, 7])
    }
}
///Rounding Add returning High Narrow (high half).
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vraddhn_high_u64)
pub fn vraddhn_high_u64(a: uint32x2_t, b: uint64x2_t, c: uint64x2_t) -> uint32x4_t {
    {
        let x: uint32x2_t = transmute(vraddhn_s64(transmute(b), transmute(c)));
        simd_shuffle(a, x, [0, 1, 2, 3])
    }
}
///Rounding Add returning High Narrow.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vraddhn_s16)
pub fn vraddhn_s16(a: int16x8_t, b: int16x8_t) -> int8x8_t {
    { _vraddhn_s16(a, b) }
}
///Rounding Add returning High Narrow.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vraddhn_s32)
pub fn vraddhn_s32(a: int32x4_t, b: int32x4_t) -> int16x4_t {
    { _vraddhn_s32(a, b) }
}
///Rounding Add returning High Narrow.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vraddhn_s64)
pub fn vraddhn_s64(a: int64x2_t, b: int64x2_t) -> int32x2_t {
    { _vraddhn_s64(a, b) }
}
///Rounding Add returning High Narrow.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vraddhn_u16)
pub fn vraddhn_u16(a: uint16x8_t, b: uint16x8_t) -> uint8x8_t {
    { transmute(vraddhn_s16(transmute(a), transmute(b))) }
}
///Rounding Add returning High Narrow.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vraddhn_u16)
pub fn vraddhn_u16(a: uint16x8_t, b: uint16x8_t) -> uint8x8_t {
    let a: uint16x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    let b: uint16x8_t = unsafe { simd_shuffle(b, b, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: uint8x8_t = transmute(vraddhn_s16(transmute(a), transmute(b)));
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Rounding Add returning High Narrow.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vraddhn_u32)
pub fn vraddhn_u32(a: uint32x4_t, b: uint32x4_t) -> uint16x4_t {
    { transmute(vraddhn_s32(transmute(a), transmute(b))) }
}
///Rounding Add returning High Narrow.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vraddhn_u32)
pub fn vraddhn_u32(a: uint32x4_t, b: uint32x4_t) -> uint16x4_t {
    let a: uint32x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    let b: uint32x4_t = unsafe { simd_shuffle(b, b, [3, 2, 1, 0]) };
    {
        let ret_val: uint16x4_t = transmute(vraddhn_s32(transmute(a), transmute(b)));
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Rounding Add returning High Narrow.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vraddhn_u64)
pub fn vraddhn_u64(a: uint64x2_t, b: uint64x2_t) -> uint32x2_t {
    { transmute(vraddhn_s64(transmute(a), transmute(b))) }
}
///Rounding Add returning High Narrow.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vraddhn_u64)
pub fn vraddhn_u64(a: uint64x2_t, b: uint64x2_t) -> uint32x2_t {
    let a: uint64x2_t = unsafe { simd_shuffle(a, a, [1, 0]) };
    let b: uint64x2_t = unsafe { simd_shuffle(b, b, [1, 0]) };
    {
        let ret_val: uint32x2_t = transmute(vraddhn_s64(transmute(a), transmute(b)));
        simd_shuffle(ret_val, ret_val, [1, 0])
    }
}
///Reciprocal estimate.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrecpe_f16)
pub fn vrecpe_f16(a: float16x4_t) -> float16x4_t {
    { _vrecpe_f16(a) }
}
///Reciprocal estimate.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrecpeq_f16)
pub fn vrecpeq_f16(a: float16x8_t) -> float16x8_t {
    { _vrecpeq_f16(a) }
}
///Reciprocal estimate.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrecpe_f32)
pub fn vrecpe_f32(a: float32x2_t) -> float32x2_t {
    { _vrecpe_f32(a) }
}
///Reciprocal estimate.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrecpeq_f32)
pub fn vrecpeq_f32(a: float32x4_t) -> float32x4_t {
    { _vrecpeq_f32(a) }
}
///Unsigned reciprocal estimate
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrecpe_u32)
pub fn vrecpe_u32(a: uint32x2_t) -> uint32x2_t {
    { _vrecpe_u32(a) }
}
///Unsigned reciprocal estimate
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrecpeq_u32)
pub fn vrecpeq_u32(a: uint32x4_t) -> uint32x4_t {
    { _vrecpeq_u32(a) }
}
///Floating-point reciprocal step
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrecps_f16)
pub fn vrecps_f16(a: float16x4_t, b: float16x4_t) -> float16x4_t {
    { _vrecps_f16(a, b) }
}
///Floating-point reciprocal step
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrecpsq_f16)
pub fn vrecpsq_f16(a: float16x8_t, b: float16x8_t) -> float16x8_t {
    { _vrecpsq_f16(a, b) }
}
///Floating-point reciprocal step
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrecps_f32)
pub fn vrecps_f32(a: float32x2_t, b: float32x2_t) -> float32x2_t {
    { _vrecps_f32(a, b) }
}
///Floating-point reciprocal step
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrecpsq_f32)
pub fn vrecpsq_f32(a: float32x4_t, b: float32x4_t) -> float32x4_t {
    { _vrecpsq_f32(a, b) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_f32_f16)
pub fn vreinterpret_f32_f16(a: float16x4_t) -> float32x2_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_f32_f16)
pub fn vreinterpret_f32_f16(a: float16x4_t) -> float32x2_t {
    let a: float16x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    {
        let ret_val: float32x2_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s8_f16)
pub fn vreinterpret_s8_f16(a: float16x4_t) -> int8x8_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s8_f16)
pub fn vreinterpret_s8_f16(a: float16x4_t) -> int8x8_t {
    let a: float16x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    {
        let ret_val: int8x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s16_f16)
pub fn vreinterpret_s16_f16(a: float16x4_t) -> int16x4_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s16_f16)
pub fn vreinterpret_s16_f16(a: float16x4_t) -> int16x4_t {
    let a: float16x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    {
        let ret_val: int16x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s32_f16)
pub fn vreinterpret_s32_f16(a: float16x4_t) -> int32x2_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s32_f16)
pub fn vreinterpret_s32_f16(a: float16x4_t) -> int32x2_t {
    let a: float16x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    {
        let ret_val: int32x2_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s64_f16)
pub fn vreinterpret_s64_f16(a: float16x4_t) -> int64x1_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s64_f16)
pub fn vreinterpret_s64_f16(a: float16x4_t) -> int64x1_t {
    let a: float16x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u8_f16)
pub fn vreinterpret_u8_f16(a: float16x4_t) -> uint8x8_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u8_f16)
pub fn vreinterpret_u8_f16(a: float16x4_t) -> uint8x8_t {
    let a: float16x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    {
        let ret_val: uint8x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u16_f16)
pub fn vreinterpret_u16_f16(a: float16x4_t) -> uint16x4_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u16_f16)
pub fn vreinterpret_u16_f16(a: float16x4_t) -> uint16x4_t {
    let a: float16x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    {
        let ret_val: uint16x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u32_f16)
pub fn vreinterpret_u32_f16(a: float16x4_t) -> uint32x2_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u32_f16)
pub fn vreinterpret_u32_f16(a: float16x4_t) -> uint32x2_t {
    let a: float16x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    {
        let ret_val: uint32x2_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u64_f16)
pub fn vreinterpret_u64_f16(a: float16x4_t) -> uint64x1_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u64_f16)
pub fn vreinterpret_u64_f16(a: float16x4_t) -> uint64x1_t {
    let a: float16x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_p8_f16)
pub fn vreinterpret_p8_f16(a: float16x4_t) -> poly8x8_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_p8_f16)
pub fn vreinterpret_p8_f16(a: float16x4_t) -> poly8x8_t {
    let a: float16x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    {
        let ret_val: poly8x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_p16_f16)
pub fn vreinterpret_p16_f16(a: float16x4_t) -> poly16x4_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_p16_f16)
pub fn vreinterpret_p16_f16(a: float16x4_t) -> poly16x4_t {
    let a: float16x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    {
        let ret_val: poly16x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_f32_f16)
pub fn vreinterpretq_f32_f16(a: float16x8_t) -> float32x4_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_f32_f16)
pub fn vreinterpretq_f32_f16(a: float16x8_t) -> float32x4_t {
    let a: float16x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: float32x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s8_f16)
pub fn vreinterpretq_s8_f16(a: float16x8_t) -> int8x16_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s8_f16)
pub fn vreinterpretq_s8_f16(a: float16x8_t) -> int8x16_t {
    let a: float16x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: int8x16_t = transmute(a);
        simd_shuffle(
            ret_val, ret_val, [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
        )
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s16_f16)
pub fn vreinterpretq_s16_f16(a: float16x8_t) -> int16x8_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s16_f16)
pub fn vreinterpretq_s16_f16(a: float16x8_t) -> int16x8_t {
    let a: float16x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: int16x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s32_f16)
pub fn vreinterpretq_s32_f16(a: float16x8_t) -> int32x4_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s32_f16)
pub fn vreinterpretq_s32_f16(a: float16x8_t) -> int32x4_t {
    let a: float16x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: int32x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s64_f16)
pub fn vreinterpretq_s64_f16(a: float16x8_t) -> int64x2_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s64_f16)
pub fn vreinterpretq_s64_f16(a: float16x8_t) -> int64x2_t {
    let a: float16x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: int64x2_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u8_f16)
pub fn vreinterpretq_u8_f16(a: float16x8_t) -> uint8x16_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u8_f16)
pub fn vreinterpretq_u8_f16(a: float16x8_t) -> uint8x16_t {
    let a: float16x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: uint8x16_t = transmute(a);
        simd_shuffle(
            ret_val, ret_val, [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
        )
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u16_f16)
pub fn vreinterpretq_u16_f16(a: float16x8_t) -> uint16x8_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u16_f16)
pub fn vreinterpretq_u16_f16(a: float16x8_t) -> uint16x8_t {
    let a: float16x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: uint16x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u32_f16)
pub fn vreinterpretq_u32_f16(a: float16x8_t) -> uint32x4_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u32_f16)
pub fn vreinterpretq_u32_f16(a: float16x8_t) -> uint32x4_t {
    let a: float16x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: uint32x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u64_f16)
pub fn vreinterpretq_u64_f16(a: float16x8_t) -> uint64x2_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u64_f16)
pub fn vreinterpretq_u64_f16(a: float16x8_t) -> uint64x2_t {
    let a: float16x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: uint64x2_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p8_f16)
pub fn vreinterpretq_p8_f16(a: float16x8_t) -> poly8x16_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p8_f16)
pub fn vreinterpretq_p8_f16(a: float16x8_t) -> poly8x16_t {
    let a: float16x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: poly8x16_t = transmute(a);
        simd_shuffle(
            ret_val, ret_val, [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
        )
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p16_f16)
pub fn vreinterpretq_p16_f16(a: float16x8_t) -> poly16x8_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p16_f16)
pub fn vreinterpretq_p16_f16(a: float16x8_t) -> poly16x8_t {
    let a: float16x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: poly16x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_f16_f32)
pub fn vreinterpret_f16_f32(a: float32x2_t) -> float16x4_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_f16_f32)
pub fn vreinterpret_f16_f32(a: float32x2_t) -> float16x4_t {
    let a: float32x2_t = unsafe { simd_shuffle(a, a, [1, 0]) };
    {
        let ret_val: float16x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_f16_f32)
pub fn vreinterpretq_f16_f32(a: float32x4_t) -> float16x8_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_f16_f32)
pub fn vreinterpretq_f16_f32(a: float32x4_t) -> float16x8_t {
    let a: float32x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    {
        let ret_val: float16x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_f16_s8)
pub fn vreinterpret_f16_s8(a: int8x8_t) -> float16x4_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_f16_s8)
pub fn vreinterpret_f16_s8(a: int8x8_t) -> float16x4_t {
    let a: int8x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: float16x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_f16_s8)
pub fn vreinterpretq_f16_s8(a: int8x16_t) -> float16x8_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_f16_s8)
pub fn vreinterpretq_f16_s8(a: int8x16_t) -> float16x8_t {
    let a: int8x16_t = unsafe {
        simd_shuffle(a, a, [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0])
    };
    {
        let ret_val: float16x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_f16_s16)
pub fn vreinterpret_f16_s16(a: int16x4_t) -> float16x4_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_f16_s16)
pub fn vreinterpret_f16_s16(a: int16x4_t) -> float16x4_t {
    let a: int16x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    {
        let ret_val: float16x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_f16_s16)
pub fn vreinterpretq_f16_s16(a: int16x8_t) -> float16x8_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_f16_s16)
pub fn vreinterpretq_f16_s16(a: int16x8_t) -> float16x8_t {
    let a: int16x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: float16x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_f16_s32)
pub fn vreinterpret_f16_s32(a: int32x2_t) -> float16x4_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_f16_s32)
pub fn vreinterpret_f16_s32(a: int32x2_t) -> float16x4_t {
    let a: int32x2_t = unsafe { simd_shuffle(a, a, [1, 0]) };
    {
        let ret_val: float16x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_f16_s32)
pub fn vreinterpretq_f16_s32(a: int32x4_t) -> float16x8_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_f16_s32)
pub fn vreinterpretq_f16_s32(a: int32x4_t) -> float16x8_t {
    let a: int32x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    {
        let ret_val: float16x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_f16_s64)
pub fn vreinterpret_f16_s64(a: int64x1_t) -> float16x4_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_f16_s64)
pub fn vreinterpret_f16_s64(a: int64x1_t) -> float16x4_t {
    {
        let ret_val: float16x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_f16_s64)
pub fn vreinterpretq_f16_s64(a: int64x2_t) -> float16x8_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_f16_s64)
pub fn vreinterpretq_f16_s64(a: int64x2_t) -> float16x8_t {
    let a: int64x2_t = unsafe { simd_shuffle(a, a, [1, 0]) };
    {
        let ret_val: float16x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_f16_u8)
pub fn vreinterpret_f16_u8(a: uint8x8_t) -> float16x4_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_f16_u8)
pub fn vreinterpret_f16_u8(a: uint8x8_t) -> float16x4_t {
    let a: uint8x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: float16x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_f16_u8)
pub fn vreinterpretq_f16_u8(a: uint8x16_t) -> float16x8_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_f16_u8)
pub fn vreinterpretq_f16_u8(a: uint8x16_t) -> float16x8_t {
    let a: uint8x16_t = unsafe {
        simd_shuffle(a, a, [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0])
    };
    {
        let ret_val: float16x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_f16_u16)
pub fn vreinterpret_f16_u16(a: uint16x4_t) -> float16x4_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_f16_u16)
pub fn vreinterpret_f16_u16(a: uint16x4_t) -> float16x4_t {
    let a: uint16x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    {
        let ret_val: float16x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_f16_u16)
pub fn vreinterpretq_f16_u16(a: uint16x8_t) -> float16x8_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_f16_u16)
pub fn vreinterpretq_f16_u16(a: uint16x8_t) -> float16x8_t {
    let a: uint16x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: float16x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_f16_u32)
pub fn vreinterpret_f16_u32(a: uint32x2_t) -> float16x4_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_f16_u32)
pub fn vreinterpret_f16_u32(a: uint32x2_t) -> float16x4_t {
    let a: uint32x2_t = unsafe { simd_shuffle(a, a, [1, 0]) };
    {
        let ret_val: float16x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_f16_u32)
pub fn vreinterpretq_f16_u32(a: uint32x4_t) -> float16x8_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_f16_u32)
pub fn vreinterpretq_f16_u32(a: uint32x4_t) -> float16x8_t {
    let a: uint32x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    {
        let ret_val: float16x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_f16_u64)
pub fn vreinterpret_f16_u64(a: uint64x1_t) -> float16x4_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_f16_u64)
pub fn vreinterpret_f16_u64(a: uint64x1_t) -> float16x4_t {
    {
        let ret_val: float16x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_f16_u64)
pub fn vreinterpretq_f16_u64(a: uint64x2_t) -> float16x8_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_f16_u64)
pub fn vreinterpretq_f16_u64(a: uint64x2_t) -> float16x8_t {
    let a: uint64x2_t = unsafe { simd_shuffle(a, a, [1, 0]) };
    {
        let ret_val: float16x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_f16_p8)
pub fn vreinterpret_f16_p8(a: poly8x8_t) -> float16x4_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_f16_p8)
pub fn vreinterpret_f16_p8(a: poly8x8_t) -> float16x4_t {
    let a: poly8x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: float16x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_f16_p8)
pub fn vreinterpretq_f16_p8(a: poly8x16_t) -> float16x8_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_f16_p8)
pub fn vreinterpretq_f16_p8(a: poly8x16_t) -> float16x8_t {
    let a: poly8x16_t = unsafe {
        simd_shuffle(a, a, [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0])
    };
    {
        let ret_val: float16x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_f16_p16)
pub fn vreinterpret_f16_p16(a: poly16x4_t) -> float16x4_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_f16_p16)
pub fn vreinterpret_f16_p16(a: poly16x4_t) -> float16x4_t {
    let a: poly16x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    {
        let ret_val: float16x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_f16_p16)
pub fn vreinterpretq_f16_p16(a: poly16x8_t) -> float16x8_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_f16_p16)
pub fn vreinterpretq_f16_p16(a: poly16x8_t) -> float16x8_t {
    let a: poly16x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: float16x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_f16_p128)
pub fn vreinterpretq_f16_p128(a: p128) -> float16x8_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_f16_p128)
pub fn vreinterpretq_f16_p128(a: p128) -> float16x8_t {
    {
        let ret_val: float16x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_p64_f16)
pub fn vreinterpret_p64_f16(a: float16x4_t) -> poly64x1_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_p64_f16)
pub fn vreinterpret_p64_f16(a: float16x4_t) -> poly64x1_t {
    let a: float16x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p128_f16)
pub fn vreinterpretq_p128_f16(a: float16x8_t) -> p128 {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p128_f16)
pub fn vreinterpretq_p128_f16(a: float16x8_t) -> p128 {
    let a: float16x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p64_f16)
pub fn vreinterpretq_p64_f16(a: float16x8_t) -> poly64x2_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p64_f16)
pub fn vreinterpretq_p64_f16(a: float16x8_t) -> poly64x2_t {
    let a: float16x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: poly64x2_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_f16_p64)
pub fn vreinterpret_f16_p64(a: poly64x1_t) -> float16x4_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_f16_p64)
pub fn vreinterpret_f16_p64(a: poly64x1_t) -> float16x4_t {
    {
        let ret_val: float16x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_f16_p64)
pub fn vreinterpretq_f16_p64(a: poly64x2_t) -> float16x8_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_f16_p64)
pub fn vreinterpretq_f16_p64(a: poly64x2_t) -> float16x8_t {
    let a: poly64x2_t = unsafe { simd_shuffle(a, a, [1, 0]) };
    {
        let ret_val: float16x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_f32_p128)
pub fn vreinterpretq_f32_p128(a: p128) -> float32x4_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_f32_p128)
pub fn vreinterpretq_f32_p128(a: p128) -> float32x4_t {
    {
        let ret_val: float32x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s8_f32)
pub fn vreinterpret_s8_f32(a: float32x2_t) -> int8x8_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s8_f32)
pub fn vreinterpret_s8_f32(a: float32x2_t) -> int8x8_t {
    let a: float32x2_t = unsafe { simd_shuffle(a, a, [1, 0]) };
    {
        let ret_val: int8x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s16_f32)
pub fn vreinterpret_s16_f32(a: float32x2_t) -> int16x4_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s16_f32)
pub fn vreinterpret_s16_f32(a: float32x2_t) -> int16x4_t {
    let a: float32x2_t = unsafe { simd_shuffle(a, a, [1, 0]) };
    {
        let ret_val: int16x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s32_f32)
pub fn vreinterpret_s32_f32(a: float32x2_t) -> int32x2_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s32_f32)
pub fn vreinterpret_s32_f32(a: float32x2_t) -> int32x2_t {
    let a: float32x2_t = unsafe { simd_shuffle(a, a, [1, 0]) };
    {
        let ret_val: int32x2_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s64_f32)
pub fn vreinterpret_s64_f32(a: float32x2_t) -> int64x1_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s64_f32)
pub fn vreinterpret_s64_f32(a: float32x2_t) -> int64x1_t {
    let a: float32x2_t = unsafe { simd_shuffle(a, a, [1, 0]) };
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u8_f32)
pub fn vreinterpret_u8_f32(a: float32x2_t) -> uint8x8_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u8_f32)
pub fn vreinterpret_u8_f32(a: float32x2_t) -> uint8x8_t {
    let a: float32x2_t = unsafe { simd_shuffle(a, a, [1, 0]) };
    {
        let ret_val: uint8x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u16_f32)
pub fn vreinterpret_u16_f32(a: float32x2_t) -> uint16x4_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u16_f32)
pub fn vreinterpret_u16_f32(a: float32x2_t) -> uint16x4_t {
    let a: float32x2_t = unsafe { simd_shuffle(a, a, [1, 0]) };
    {
        let ret_val: uint16x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u32_f32)
pub fn vreinterpret_u32_f32(a: float32x2_t) -> uint32x2_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u32_f32)
pub fn vreinterpret_u32_f32(a: float32x2_t) -> uint32x2_t {
    let a: float32x2_t = unsafe { simd_shuffle(a, a, [1, 0]) };
    {
        let ret_val: uint32x2_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u64_f32)
pub fn vreinterpret_u64_f32(a: float32x2_t) -> uint64x1_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u64_f32)
pub fn vreinterpret_u64_f32(a: float32x2_t) -> uint64x1_t {
    let a: float32x2_t = unsafe { simd_shuffle(a, a, [1, 0]) };
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_p8_f32)
pub fn vreinterpret_p8_f32(a: float32x2_t) -> poly8x8_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_p8_f32)
pub fn vreinterpret_p8_f32(a: float32x2_t) -> poly8x8_t {
    let a: float32x2_t = unsafe { simd_shuffle(a, a, [1, 0]) };
    {
        let ret_val: poly8x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_p16_f32)
pub fn vreinterpret_p16_f32(a: float32x2_t) -> poly16x4_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_p16_f32)
pub fn vreinterpret_p16_f32(a: float32x2_t) -> poly16x4_t {
    let a: float32x2_t = unsafe { simd_shuffle(a, a, [1, 0]) };
    {
        let ret_val: poly16x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p128_f32)
pub fn vreinterpretq_p128_f32(a: float32x4_t) -> p128 {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p128_f32)
pub fn vreinterpretq_p128_f32(a: float32x4_t) -> p128 {
    let a: float32x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s8_f32)
pub fn vreinterpretq_s8_f32(a: float32x4_t) -> int8x16_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s8_f32)
pub fn vreinterpretq_s8_f32(a: float32x4_t) -> int8x16_t {
    let a: float32x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    {
        let ret_val: int8x16_t = transmute(a);
        simd_shuffle(
            ret_val, ret_val, [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
        )
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s16_f32)
pub fn vreinterpretq_s16_f32(a: float32x4_t) -> int16x8_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s16_f32)
pub fn vreinterpretq_s16_f32(a: float32x4_t) -> int16x8_t {
    let a: float32x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    {
        let ret_val: int16x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s32_f32)
pub fn vreinterpretq_s32_f32(a: float32x4_t) -> int32x4_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s32_f32)
pub fn vreinterpretq_s32_f32(a: float32x4_t) -> int32x4_t {
    let a: float32x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    {
        let ret_val: int32x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s64_f32)
pub fn vreinterpretq_s64_f32(a: float32x4_t) -> int64x2_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s64_f32)
pub fn vreinterpretq_s64_f32(a: float32x4_t) -> int64x2_t {
    let a: float32x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    {
        let ret_val: int64x2_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u8_f32)
pub fn vreinterpretq_u8_f32(a: float32x4_t) -> uint8x16_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u8_f32)
pub fn vreinterpretq_u8_f32(a: float32x4_t) -> uint8x16_t {
    let a: float32x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    {
        let ret_val: uint8x16_t = transmute(a);
        simd_shuffle(
            ret_val, ret_val, [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
        )
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u16_f32)
pub fn vreinterpretq_u16_f32(a: float32x4_t) -> uint16x8_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u16_f32)
pub fn vreinterpretq_u16_f32(a: float32x4_t) -> uint16x8_t {
    let a: float32x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    {
        let ret_val: uint16x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u32_f32)
pub fn vreinterpretq_u32_f32(a: float32x4_t) -> uint32x4_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u32_f32)
pub fn vreinterpretq_u32_f32(a: float32x4_t) -> uint32x4_t {
    let a: float32x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    {
        let ret_val: uint32x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u64_f32)
pub fn vreinterpretq_u64_f32(a: float32x4_t) -> uint64x2_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u64_f32)
pub fn vreinterpretq_u64_f32(a: float32x4_t) -> uint64x2_t {
    let a: float32x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    {
        let ret_val: uint64x2_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p8_f32)
pub fn vreinterpretq_p8_f32(a: float32x4_t) -> poly8x16_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p8_f32)
pub fn vreinterpretq_p8_f32(a: float32x4_t) -> poly8x16_t {
    let a: float32x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    {
        let ret_val: poly8x16_t = transmute(a);
        simd_shuffle(
            ret_val, ret_val, [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
        )
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p16_f32)
pub fn vreinterpretq_p16_f32(a: float32x4_t) -> poly16x8_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p16_f32)
pub fn vreinterpretq_p16_f32(a: float32x4_t) -> poly16x8_t {
    let a: float32x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    {
        let ret_val: poly16x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_f32_s8)
pub fn vreinterpret_f32_s8(a: int8x8_t) -> float32x2_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_f32_s8)
pub fn vreinterpret_f32_s8(a: int8x8_t) -> float32x2_t {
    let a: int8x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: float32x2_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s16_s8)
pub fn vreinterpret_s16_s8(a: int8x8_t) -> int16x4_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s16_s8)
pub fn vreinterpret_s16_s8(a: int8x8_t) -> int16x4_t {
    let a: int8x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: int16x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s32_s8)
pub fn vreinterpret_s32_s8(a: int8x8_t) -> int32x2_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s32_s8)
pub fn vreinterpret_s32_s8(a: int8x8_t) -> int32x2_t {
    let a: int8x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: int32x2_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s64_s8)
pub fn vreinterpret_s64_s8(a: int8x8_t) -> int64x1_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s64_s8)
pub fn vreinterpret_s64_s8(a: int8x8_t) -> int64x1_t {
    let a: int8x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u8_s8)
pub fn vreinterpret_u8_s8(a: int8x8_t) -> uint8x8_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u8_s8)
pub fn vreinterpret_u8_s8(a: int8x8_t) -> uint8x8_t {
    let a: int8x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: uint8x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u16_s8)
pub fn vreinterpret_u16_s8(a: int8x8_t) -> uint16x4_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u16_s8)
pub fn vreinterpret_u16_s8(a: int8x8_t) -> uint16x4_t {
    let a: int8x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: uint16x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u32_s8)
pub fn vreinterpret_u32_s8(a: int8x8_t) -> uint32x2_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u32_s8)
pub fn vreinterpret_u32_s8(a: int8x8_t) -> uint32x2_t {
    let a: int8x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: uint32x2_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u64_s8)
pub fn vreinterpret_u64_s8(a: int8x8_t) -> uint64x1_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u64_s8)
pub fn vreinterpret_u64_s8(a: int8x8_t) -> uint64x1_t {
    let a: int8x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_p8_s8)
pub fn vreinterpret_p8_s8(a: int8x8_t) -> poly8x8_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_p8_s8)
pub fn vreinterpret_p8_s8(a: int8x8_t) -> poly8x8_t {
    let a: int8x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: poly8x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_p16_s8)
pub fn vreinterpret_p16_s8(a: int8x8_t) -> poly16x4_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_p16_s8)
pub fn vreinterpret_p16_s8(a: int8x8_t) -> poly16x4_t {
    let a: int8x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: poly16x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_f32_s8)
pub fn vreinterpretq_f32_s8(a: int8x16_t) -> float32x4_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_f32_s8)
pub fn vreinterpretq_f32_s8(a: int8x16_t) -> float32x4_t {
    let a: int8x16_t = unsafe {
        simd_shuffle(a, a, [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0])
    };
    {
        let ret_val: float32x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s16_s8)
pub fn vreinterpretq_s16_s8(a: int8x16_t) -> int16x8_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s16_s8)
pub fn vreinterpretq_s16_s8(a: int8x16_t) -> int16x8_t {
    let a: int8x16_t = unsafe {
        simd_shuffle(a, a, [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0])
    };
    {
        let ret_val: int16x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s32_s8)
pub fn vreinterpretq_s32_s8(a: int8x16_t) -> int32x4_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s32_s8)
pub fn vreinterpretq_s32_s8(a: int8x16_t) -> int32x4_t {
    let a: int8x16_t = unsafe {
        simd_shuffle(a, a, [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0])
    };
    {
        let ret_val: int32x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s64_s8)
pub fn vreinterpretq_s64_s8(a: int8x16_t) -> int64x2_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s64_s8)
pub fn vreinterpretq_s64_s8(a: int8x16_t) -> int64x2_t {
    let a: int8x16_t = unsafe {
        simd_shuffle(a, a, [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0])
    };
    {
        let ret_val: int64x2_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u8_s8)
pub fn vreinterpretq_u8_s8(a: int8x16_t) -> uint8x16_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u8_s8)
pub fn vreinterpretq_u8_s8(a: int8x16_t) -> uint8x16_t {
    let a: int8x16_t = unsafe {
        simd_shuffle(a, a, [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0])
    };
    {
        let ret_val: uint8x16_t = transmute(a);
        simd_shuffle(
            ret_val, ret_val, [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
        )
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u16_s8)
pub fn vreinterpretq_u16_s8(a: int8x16_t) -> uint16x8_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u16_s8)
pub fn vreinterpretq_u16_s8(a: int8x16_t) -> uint16x8_t {
    let a: int8x16_t = unsafe {
        simd_shuffle(a, a, [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0])
    };
    {
        let ret_val: uint16x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u32_s8)
pub fn vreinterpretq_u32_s8(a: int8x16_t) -> uint32x4_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u32_s8)
pub fn vreinterpretq_u32_s8(a: int8x16_t) -> uint32x4_t {
    let a: int8x16_t = unsafe {
        simd_shuffle(a, a, [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0])
    };
    {
        let ret_val: uint32x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u64_s8)
pub fn vreinterpretq_u64_s8(a: int8x16_t) -> uint64x2_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u64_s8)
pub fn vreinterpretq_u64_s8(a: int8x16_t) -> uint64x2_t {
    let a: int8x16_t = unsafe {
        simd_shuffle(a, a, [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0])
    };
    {
        let ret_val: uint64x2_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p8_s8)
pub fn vreinterpretq_p8_s8(a: int8x16_t) -> poly8x16_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p8_s8)
pub fn vreinterpretq_p8_s8(a: int8x16_t) -> poly8x16_t {
    let a: int8x16_t = unsafe {
        simd_shuffle(a, a, [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0])
    };
    {
        let ret_val: poly8x16_t = transmute(a);
        simd_shuffle(
            ret_val, ret_val, [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
        )
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p16_s8)
pub fn vreinterpretq_p16_s8(a: int8x16_t) -> poly16x8_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p16_s8)
pub fn vreinterpretq_p16_s8(a: int8x16_t) -> poly16x8_t {
    let a: int8x16_t = unsafe {
        simd_shuffle(a, a, [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0])
    };
    {
        let ret_val: poly16x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_f32_s16)
pub fn vreinterpret_f32_s16(a: int16x4_t) -> float32x2_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_f32_s16)
pub fn vreinterpret_f32_s16(a: int16x4_t) -> float32x2_t {
    let a: int16x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    {
        let ret_val: float32x2_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s8_s16)
pub fn vreinterpret_s8_s16(a: int16x4_t) -> int8x8_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s8_s16)
pub fn vreinterpret_s8_s16(a: int16x4_t) -> int8x8_t {
    let a: int16x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    {
        let ret_val: int8x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s32_s16)
pub fn vreinterpret_s32_s16(a: int16x4_t) -> int32x2_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s32_s16)
pub fn vreinterpret_s32_s16(a: int16x4_t) -> int32x2_t {
    let a: int16x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    {
        let ret_val: int32x2_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s64_s16)
pub fn vreinterpret_s64_s16(a: int16x4_t) -> int64x1_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s64_s16)
pub fn vreinterpret_s64_s16(a: int16x4_t) -> int64x1_t {
    let a: int16x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u8_s16)
pub fn vreinterpret_u8_s16(a: int16x4_t) -> uint8x8_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u8_s16)
pub fn vreinterpret_u8_s16(a: int16x4_t) -> uint8x8_t {
    let a: int16x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    {
        let ret_val: uint8x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u16_s16)
pub fn vreinterpret_u16_s16(a: int16x4_t) -> uint16x4_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u16_s16)
pub fn vreinterpret_u16_s16(a: int16x4_t) -> uint16x4_t {
    let a: int16x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    {
        let ret_val: uint16x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u32_s16)
pub fn vreinterpret_u32_s16(a: int16x4_t) -> uint32x2_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u32_s16)
pub fn vreinterpret_u32_s16(a: int16x4_t) -> uint32x2_t {
    let a: int16x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    {
        let ret_val: uint32x2_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u64_s16)
pub fn vreinterpret_u64_s16(a: int16x4_t) -> uint64x1_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u64_s16)
pub fn vreinterpret_u64_s16(a: int16x4_t) -> uint64x1_t {
    let a: int16x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_p8_s16)
pub fn vreinterpret_p8_s16(a: int16x4_t) -> poly8x8_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_p8_s16)
pub fn vreinterpret_p8_s16(a: int16x4_t) -> poly8x8_t {
    let a: int16x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    {
        let ret_val: poly8x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_p16_s16)
pub fn vreinterpret_p16_s16(a: int16x4_t) -> poly16x4_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_p16_s16)
pub fn vreinterpret_p16_s16(a: int16x4_t) -> poly16x4_t {
    let a: int16x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    {
        let ret_val: poly16x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_f32_s16)
pub fn vreinterpretq_f32_s16(a: int16x8_t) -> float32x4_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_f32_s16)
pub fn vreinterpretq_f32_s16(a: int16x8_t) -> float32x4_t {
    let a: int16x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: float32x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s8_s16)
pub fn vreinterpretq_s8_s16(a: int16x8_t) -> int8x16_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s8_s16)
pub fn vreinterpretq_s8_s16(a: int16x8_t) -> int8x16_t {
    let a: int16x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: int8x16_t = transmute(a);
        simd_shuffle(
            ret_val, ret_val, [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
        )
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s32_s16)
pub fn vreinterpretq_s32_s16(a: int16x8_t) -> int32x4_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s32_s16)
pub fn vreinterpretq_s32_s16(a: int16x8_t) -> int32x4_t {
    let a: int16x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: int32x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s64_s16)
pub fn vreinterpretq_s64_s16(a: int16x8_t) -> int64x2_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s64_s16)
pub fn vreinterpretq_s64_s16(a: int16x8_t) -> int64x2_t {
    let a: int16x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: int64x2_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u8_s16)
pub fn vreinterpretq_u8_s16(a: int16x8_t) -> uint8x16_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u8_s16)
pub fn vreinterpretq_u8_s16(a: int16x8_t) -> uint8x16_t {
    let a: int16x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: uint8x16_t = transmute(a);
        simd_shuffle(
            ret_val, ret_val, [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
        )
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u16_s16)
pub fn vreinterpretq_u16_s16(a: int16x8_t) -> uint16x8_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u16_s16)
pub fn vreinterpretq_u16_s16(a: int16x8_t) -> uint16x8_t {
    let a: int16x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: uint16x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u32_s16)
pub fn vreinterpretq_u32_s16(a: int16x8_t) -> uint32x4_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u32_s16)
pub fn vreinterpretq_u32_s16(a: int16x8_t) -> uint32x4_t {
    let a: int16x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: uint32x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u64_s16)
pub fn vreinterpretq_u64_s16(a: int16x8_t) -> uint64x2_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u64_s16)
pub fn vreinterpretq_u64_s16(a: int16x8_t) -> uint64x2_t {
    let a: int16x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: uint64x2_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p8_s16)
pub fn vreinterpretq_p8_s16(a: int16x8_t) -> poly8x16_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p8_s16)
pub fn vreinterpretq_p8_s16(a: int16x8_t) -> poly8x16_t {
    let a: int16x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: poly8x16_t = transmute(a);
        simd_shuffle(
            ret_val, ret_val, [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
        )
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p16_s16)
pub fn vreinterpretq_p16_s16(a: int16x8_t) -> poly16x8_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p16_s16)
pub fn vreinterpretq_p16_s16(a: int16x8_t) -> poly16x8_t {
    let a: int16x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: poly16x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_f32_s32)
pub fn vreinterpret_f32_s32(a: int32x2_t) -> float32x2_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_f32_s32)
pub fn vreinterpret_f32_s32(a: int32x2_t) -> float32x2_t {
    let a: int32x2_t = unsafe { simd_shuffle(a, a, [1, 0]) };
    {
        let ret_val: float32x2_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s8_s32)
pub fn vreinterpret_s8_s32(a: int32x2_t) -> int8x8_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s8_s32)
pub fn vreinterpret_s8_s32(a: int32x2_t) -> int8x8_t {
    let a: int32x2_t = unsafe { simd_shuffle(a, a, [1, 0]) };
    {
        let ret_val: int8x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s16_s32)
pub fn vreinterpret_s16_s32(a: int32x2_t) -> int16x4_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s16_s32)
pub fn vreinterpret_s16_s32(a: int32x2_t) -> int16x4_t {
    let a: int32x2_t = unsafe { simd_shuffle(a, a, [1, 0]) };
    {
        let ret_val: int16x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s64_s32)
pub fn vreinterpret_s64_s32(a: int32x2_t) -> int64x1_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s64_s32)
pub fn vreinterpret_s64_s32(a: int32x2_t) -> int64x1_t {
    let a: int32x2_t = unsafe { simd_shuffle(a, a, [1, 0]) };
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u8_s32)
pub fn vreinterpret_u8_s32(a: int32x2_t) -> uint8x8_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u8_s32)
pub fn vreinterpret_u8_s32(a: int32x2_t) -> uint8x8_t {
    let a: int32x2_t = unsafe { simd_shuffle(a, a, [1, 0]) };
    {
        let ret_val: uint8x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u16_s32)
pub fn vreinterpret_u16_s32(a: int32x2_t) -> uint16x4_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u16_s32)
pub fn vreinterpret_u16_s32(a: int32x2_t) -> uint16x4_t {
    let a: int32x2_t = unsafe { simd_shuffle(a, a, [1, 0]) };
    {
        let ret_val: uint16x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u32_s32)
pub fn vreinterpret_u32_s32(a: int32x2_t) -> uint32x2_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u32_s32)
pub fn vreinterpret_u32_s32(a: int32x2_t) -> uint32x2_t {
    let a: int32x2_t = unsafe { simd_shuffle(a, a, [1, 0]) };
    {
        let ret_val: uint32x2_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u64_s32)
pub fn vreinterpret_u64_s32(a: int32x2_t) -> uint64x1_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u64_s32)
pub fn vreinterpret_u64_s32(a: int32x2_t) -> uint64x1_t {
    let a: int32x2_t = unsafe { simd_shuffle(a, a, [1, 0]) };
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_p8_s32)
pub fn vreinterpret_p8_s32(a: int32x2_t) -> poly8x8_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_p8_s32)
pub fn vreinterpret_p8_s32(a: int32x2_t) -> poly8x8_t {
    let a: int32x2_t = unsafe { simd_shuffle(a, a, [1, 0]) };
    {
        let ret_val: poly8x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_p16_s32)
pub fn vreinterpret_p16_s32(a: int32x2_t) -> poly16x4_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_p16_s32)
pub fn vreinterpret_p16_s32(a: int32x2_t) -> poly16x4_t {
    let a: int32x2_t = unsafe { simd_shuffle(a, a, [1, 0]) };
    {
        let ret_val: poly16x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_f32_s32)
pub fn vreinterpretq_f32_s32(a: int32x4_t) -> float32x4_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_f32_s32)
pub fn vreinterpretq_f32_s32(a: int32x4_t) -> float32x4_t {
    let a: int32x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    {
        let ret_val: float32x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s8_s32)
pub fn vreinterpretq_s8_s32(a: int32x4_t) -> int8x16_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s8_s32)
pub fn vreinterpretq_s8_s32(a: int32x4_t) -> int8x16_t {
    let a: int32x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    {
        let ret_val: int8x16_t = transmute(a);
        simd_shuffle(
            ret_val, ret_val, [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
        )
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s16_s32)
pub fn vreinterpretq_s16_s32(a: int32x4_t) -> int16x8_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s16_s32)
pub fn vreinterpretq_s16_s32(a: int32x4_t) -> int16x8_t {
    let a: int32x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    {
        let ret_val: int16x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s64_s32)
pub fn vreinterpretq_s64_s32(a: int32x4_t) -> int64x2_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s64_s32)
pub fn vreinterpretq_s64_s32(a: int32x4_t) -> int64x2_t {
    let a: int32x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    {
        let ret_val: int64x2_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u8_s32)
pub fn vreinterpretq_u8_s32(a: int32x4_t) -> uint8x16_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u8_s32)
pub fn vreinterpretq_u8_s32(a: int32x4_t) -> uint8x16_t {
    let a: int32x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    {
        let ret_val: uint8x16_t = transmute(a);
        simd_shuffle(
            ret_val, ret_val, [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
        )
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u16_s32)
pub fn vreinterpretq_u16_s32(a: int32x4_t) -> uint16x8_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u16_s32)
pub fn vreinterpretq_u16_s32(a: int32x4_t) -> uint16x8_t {
    let a: int32x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    {
        let ret_val: uint16x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u32_s32)
pub fn vreinterpretq_u32_s32(a: int32x4_t) -> uint32x4_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u32_s32)
pub fn vreinterpretq_u32_s32(a: int32x4_t) -> uint32x4_t {
    let a: int32x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    {
        let ret_val: uint32x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u64_s32)
pub fn vreinterpretq_u64_s32(a: int32x4_t) -> uint64x2_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u64_s32)
pub fn vreinterpretq_u64_s32(a: int32x4_t) -> uint64x2_t {
    let a: int32x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    {
        let ret_val: uint64x2_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p8_s32)
pub fn vreinterpretq_p8_s32(a: int32x4_t) -> poly8x16_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p8_s32)
pub fn vreinterpretq_p8_s32(a: int32x4_t) -> poly8x16_t {
    let a: int32x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    {
        let ret_val: poly8x16_t = transmute(a);
        simd_shuffle(
            ret_val, ret_val, [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
        )
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p16_s32)
pub fn vreinterpretq_p16_s32(a: int32x4_t) -> poly16x8_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p16_s32)
pub fn vreinterpretq_p16_s32(a: int32x4_t) -> poly16x8_t {
    let a: int32x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    {
        let ret_val: poly16x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_f32_s64)
pub fn vreinterpret_f32_s64(a: int64x1_t) -> float32x2_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_f32_s64)
pub fn vreinterpret_f32_s64(a: int64x1_t) -> float32x2_t {
    {
        let ret_val: float32x2_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s8_s64)
pub fn vreinterpret_s8_s64(a: int64x1_t) -> int8x8_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s8_s64)
pub fn vreinterpret_s8_s64(a: int64x1_t) -> int8x8_t {
    {
        let ret_val: int8x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s16_s64)
pub fn vreinterpret_s16_s64(a: int64x1_t) -> int16x4_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s16_s64)
pub fn vreinterpret_s16_s64(a: int64x1_t) -> int16x4_t {
    {
        let ret_val: int16x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s32_s64)
pub fn vreinterpret_s32_s64(a: int64x1_t) -> int32x2_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s32_s64)
pub fn vreinterpret_s32_s64(a: int64x1_t) -> int32x2_t {
    {
        let ret_val: int32x2_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u8_s64)
pub fn vreinterpret_u8_s64(a: int64x1_t) -> uint8x8_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u8_s64)
pub fn vreinterpret_u8_s64(a: int64x1_t) -> uint8x8_t {
    {
        let ret_val: uint8x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u16_s64)
pub fn vreinterpret_u16_s64(a: int64x1_t) -> uint16x4_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u16_s64)
pub fn vreinterpret_u16_s64(a: int64x1_t) -> uint16x4_t {
    {
        let ret_val: uint16x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u32_s64)
pub fn vreinterpret_u32_s64(a: int64x1_t) -> uint32x2_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u32_s64)
pub fn vreinterpret_u32_s64(a: int64x1_t) -> uint32x2_t {
    {
        let ret_val: uint32x2_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u64_s64)
pub fn vreinterpret_u64_s64(a: int64x1_t) -> uint64x1_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_p8_s64)
pub fn vreinterpret_p8_s64(a: int64x1_t) -> poly8x8_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_p8_s64)
pub fn vreinterpret_p8_s64(a: int64x1_t) -> poly8x8_t {
    {
        let ret_val: poly8x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_p16_s64)
pub fn vreinterpret_p16_s64(a: int64x1_t) -> poly16x4_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_p16_s64)
pub fn vreinterpret_p16_s64(a: int64x1_t) -> poly16x4_t {
    {
        let ret_val: poly16x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_f32_s64)
pub fn vreinterpretq_f32_s64(a: int64x2_t) -> float32x4_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_f32_s64)
pub fn vreinterpretq_f32_s64(a: int64x2_t) -> float32x4_t {
    let a: int64x2_t = unsafe { simd_shuffle(a, a, [1, 0]) };
    {
        let ret_val: float32x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s8_s64)
pub fn vreinterpretq_s8_s64(a: int64x2_t) -> int8x16_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s8_s64)
pub fn vreinterpretq_s8_s64(a: int64x2_t) -> int8x16_t {
    let a: int64x2_t = unsafe { simd_shuffle(a, a, [1, 0]) };
    {
        let ret_val: int8x16_t = transmute(a);
        simd_shuffle(
            ret_val, ret_val, [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
        )
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s16_s64)
pub fn vreinterpretq_s16_s64(a: int64x2_t) -> int16x8_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s16_s64)
pub fn vreinterpretq_s16_s64(a: int64x2_t) -> int16x8_t {
    let a: int64x2_t = unsafe { simd_shuffle(a, a, [1, 0]) };
    {
        let ret_val: int16x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s32_s64)
pub fn vreinterpretq_s32_s64(a: int64x2_t) -> int32x4_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s32_s64)
pub fn vreinterpretq_s32_s64(a: int64x2_t) -> int32x4_t {
    let a: int64x2_t = unsafe { simd_shuffle(a, a, [1, 0]) };
    {
        let ret_val: int32x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u8_s64)
pub fn vreinterpretq_u8_s64(a: int64x2_t) -> uint8x16_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u8_s64)
pub fn vreinterpretq_u8_s64(a: int64x2_t) -> uint8x16_t {
    let a: int64x2_t = unsafe { simd_shuffle(a, a, [1, 0]) };
    {
        let ret_val: uint8x16_t = transmute(a);
        simd_shuffle(
            ret_val, ret_val, [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
        )
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u16_s64)
pub fn vreinterpretq_u16_s64(a: int64x2_t) -> uint16x8_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u16_s64)
pub fn vreinterpretq_u16_s64(a: int64x2_t) -> uint16x8_t {
    let a: int64x2_t = unsafe { simd_shuffle(a, a, [1, 0]) };
    {
        let ret_val: uint16x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u32_s64)
pub fn vreinterpretq_u32_s64(a: int64x2_t) -> uint32x4_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u32_s64)
pub fn vreinterpretq_u32_s64(a: int64x2_t) -> uint32x4_t {
    let a: int64x2_t = unsafe { simd_shuffle(a, a, [1, 0]) };
    {
        let ret_val: uint32x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u64_s64)
pub fn vreinterpretq_u64_s64(a: int64x2_t) -> uint64x2_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u64_s64)
pub fn vreinterpretq_u64_s64(a: int64x2_t) -> uint64x2_t {
    let a: int64x2_t = unsafe { simd_shuffle(a, a, [1, 0]) };
    {
        let ret_val: uint64x2_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p8_s64)
pub fn vreinterpretq_p8_s64(a: int64x2_t) -> poly8x16_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p8_s64)
pub fn vreinterpretq_p8_s64(a: int64x2_t) -> poly8x16_t {
    let a: int64x2_t = unsafe { simd_shuffle(a, a, [1, 0]) };
    {
        let ret_val: poly8x16_t = transmute(a);
        simd_shuffle(
            ret_val, ret_val, [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
        )
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p16_s64)
pub fn vreinterpretq_p16_s64(a: int64x2_t) -> poly16x8_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p16_s64)
pub fn vreinterpretq_p16_s64(a: int64x2_t) -> poly16x8_t {
    let a: int64x2_t = unsafe { simd_shuffle(a, a, [1, 0]) };
    {
        let ret_val: poly16x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_f32_u8)
pub fn vreinterpret_f32_u8(a: uint8x8_t) -> float32x2_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_f32_u8)
pub fn vreinterpret_f32_u8(a: uint8x8_t) -> float32x2_t {
    let a: uint8x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: float32x2_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s8_u8)
pub fn vreinterpret_s8_u8(a: uint8x8_t) -> int8x8_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s8_u8)
pub fn vreinterpret_s8_u8(a: uint8x8_t) -> int8x8_t {
    let a: uint8x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: int8x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s16_u8)
pub fn vreinterpret_s16_u8(a: uint8x8_t) -> int16x4_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s16_u8)
pub fn vreinterpret_s16_u8(a: uint8x8_t) -> int16x4_t {
    let a: uint8x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: int16x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s32_u8)
pub fn vreinterpret_s32_u8(a: uint8x8_t) -> int32x2_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s32_u8)
pub fn vreinterpret_s32_u8(a: uint8x8_t) -> int32x2_t {
    let a: uint8x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: int32x2_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s64_u8)
pub fn vreinterpret_s64_u8(a: uint8x8_t) -> int64x1_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s64_u8)
pub fn vreinterpret_s64_u8(a: uint8x8_t) -> int64x1_t {
    let a: uint8x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u16_u8)
pub fn vreinterpret_u16_u8(a: uint8x8_t) -> uint16x4_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u16_u8)
pub fn vreinterpret_u16_u8(a: uint8x8_t) -> uint16x4_t {
    let a: uint8x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: uint16x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u32_u8)
pub fn vreinterpret_u32_u8(a: uint8x8_t) -> uint32x2_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u32_u8)
pub fn vreinterpret_u32_u8(a: uint8x8_t) -> uint32x2_t {
    let a: uint8x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: uint32x2_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u64_u8)
pub fn vreinterpret_u64_u8(a: uint8x8_t) -> uint64x1_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u64_u8)
pub fn vreinterpret_u64_u8(a: uint8x8_t) -> uint64x1_t {
    let a: uint8x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_p8_u8)
pub fn vreinterpret_p8_u8(a: uint8x8_t) -> poly8x8_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_p8_u8)
pub fn vreinterpret_p8_u8(a: uint8x8_t) -> poly8x8_t {
    let a: uint8x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: poly8x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_p16_u8)
pub fn vreinterpret_p16_u8(a: uint8x8_t) -> poly16x4_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_p16_u8)
pub fn vreinterpret_p16_u8(a: uint8x8_t) -> poly16x4_t {
    let a: uint8x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: poly16x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_f32_u8)
pub fn vreinterpretq_f32_u8(a: uint8x16_t) -> float32x4_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_f32_u8)
pub fn vreinterpretq_f32_u8(a: uint8x16_t) -> float32x4_t {
    let a: uint8x16_t = unsafe {
        simd_shuffle(a, a, [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0])
    };
    {
        let ret_val: float32x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s8_u8)
pub fn vreinterpretq_s8_u8(a: uint8x16_t) -> int8x16_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s8_u8)
pub fn vreinterpretq_s8_u8(a: uint8x16_t) -> int8x16_t {
    let a: uint8x16_t = unsafe {
        simd_shuffle(a, a, [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0])
    };
    {
        let ret_val: int8x16_t = transmute(a);
        simd_shuffle(
            ret_val, ret_val, [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
        )
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s16_u8)
pub fn vreinterpretq_s16_u8(a: uint8x16_t) -> int16x8_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s16_u8)
pub fn vreinterpretq_s16_u8(a: uint8x16_t) -> int16x8_t {
    let a: uint8x16_t = unsafe {
        simd_shuffle(a, a, [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0])
    };
    {
        let ret_val: int16x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s32_u8)
pub fn vreinterpretq_s32_u8(a: uint8x16_t) -> int32x4_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s32_u8)
pub fn vreinterpretq_s32_u8(a: uint8x16_t) -> int32x4_t {
    let a: uint8x16_t = unsafe {
        simd_shuffle(a, a, [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0])
    };
    {
        let ret_val: int32x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s64_u8)
pub fn vreinterpretq_s64_u8(a: uint8x16_t) -> int64x2_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s64_u8)
pub fn vreinterpretq_s64_u8(a: uint8x16_t) -> int64x2_t {
    let a: uint8x16_t = unsafe {
        simd_shuffle(a, a, [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0])
    };
    {
        let ret_val: int64x2_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u16_u8)
pub fn vreinterpretq_u16_u8(a: uint8x16_t) -> uint16x8_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u16_u8)
pub fn vreinterpretq_u16_u8(a: uint8x16_t) -> uint16x8_t {
    let a: uint8x16_t = unsafe {
        simd_shuffle(a, a, [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0])
    };
    {
        let ret_val: uint16x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u32_u8)
pub fn vreinterpretq_u32_u8(a: uint8x16_t) -> uint32x4_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u32_u8)
pub fn vreinterpretq_u32_u8(a: uint8x16_t) -> uint32x4_t {
    let a: uint8x16_t = unsafe {
        simd_shuffle(a, a, [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0])
    };
    {
        let ret_val: uint32x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u64_u8)
pub fn vreinterpretq_u64_u8(a: uint8x16_t) -> uint64x2_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u64_u8)
pub fn vreinterpretq_u64_u8(a: uint8x16_t) -> uint64x2_t {
    let a: uint8x16_t = unsafe {
        simd_shuffle(a, a, [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0])
    };
    {
        let ret_val: uint64x2_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p8_u8)
pub fn vreinterpretq_p8_u8(a: uint8x16_t) -> poly8x16_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p8_u8)
pub fn vreinterpretq_p8_u8(a: uint8x16_t) -> poly8x16_t {
    let a: uint8x16_t = unsafe {
        simd_shuffle(a, a, [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0])
    };
    {
        let ret_val: poly8x16_t = transmute(a);
        simd_shuffle(
            ret_val, ret_val, [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
        )
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p16_u8)
pub fn vreinterpretq_p16_u8(a: uint8x16_t) -> poly16x8_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p16_u8)
pub fn vreinterpretq_p16_u8(a: uint8x16_t) -> poly16x8_t {
    let a: uint8x16_t = unsafe {
        simd_shuffle(a, a, [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0])
    };
    {
        let ret_val: poly16x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_f32_u16)
pub fn vreinterpret_f32_u16(a: uint16x4_t) -> float32x2_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_f32_u16)
pub fn vreinterpret_f32_u16(a: uint16x4_t) -> float32x2_t {
    let a: uint16x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    {
        let ret_val: float32x2_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s8_u16)
pub fn vreinterpret_s8_u16(a: uint16x4_t) -> int8x8_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s8_u16)
pub fn vreinterpret_s8_u16(a: uint16x4_t) -> int8x8_t {
    let a: uint16x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    {
        let ret_val: int8x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s16_u16)
pub fn vreinterpret_s16_u16(a: uint16x4_t) -> int16x4_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s16_u16)
pub fn vreinterpret_s16_u16(a: uint16x4_t) -> int16x4_t {
    let a: uint16x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    {
        let ret_val: int16x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s32_u16)
pub fn vreinterpret_s32_u16(a: uint16x4_t) -> int32x2_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s32_u16)
pub fn vreinterpret_s32_u16(a: uint16x4_t) -> int32x2_t {
    let a: uint16x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    {
        let ret_val: int32x2_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s64_u16)
pub fn vreinterpret_s64_u16(a: uint16x4_t) -> int64x1_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s64_u16)
pub fn vreinterpret_s64_u16(a: uint16x4_t) -> int64x1_t {
    let a: uint16x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u8_u16)
pub fn vreinterpret_u8_u16(a: uint16x4_t) -> uint8x8_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u8_u16)
pub fn vreinterpret_u8_u16(a: uint16x4_t) -> uint8x8_t {
    let a: uint16x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    {
        let ret_val: uint8x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u32_u16)
pub fn vreinterpret_u32_u16(a: uint16x4_t) -> uint32x2_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u32_u16)
pub fn vreinterpret_u32_u16(a: uint16x4_t) -> uint32x2_t {
    let a: uint16x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    {
        let ret_val: uint32x2_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u64_u16)
pub fn vreinterpret_u64_u16(a: uint16x4_t) -> uint64x1_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u64_u16)
pub fn vreinterpret_u64_u16(a: uint16x4_t) -> uint64x1_t {
    let a: uint16x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_p8_u16)
pub fn vreinterpret_p8_u16(a: uint16x4_t) -> poly8x8_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_p8_u16)
pub fn vreinterpret_p8_u16(a: uint16x4_t) -> poly8x8_t {
    let a: uint16x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    {
        let ret_val: poly8x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_p16_u16)
pub fn vreinterpret_p16_u16(a: uint16x4_t) -> poly16x4_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_p16_u16)
pub fn vreinterpret_p16_u16(a: uint16x4_t) -> poly16x4_t {
    let a: uint16x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    {
        let ret_val: poly16x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_f32_u16)
pub fn vreinterpretq_f32_u16(a: uint16x8_t) -> float32x4_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_f32_u16)
pub fn vreinterpretq_f32_u16(a: uint16x8_t) -> float32x4_t {
    let a: uint16x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: float32x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s8_u16)
pub fn vreinterpretq_s8_u16(a: uint16x8_t) -> int8x16_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s8_u16)
pub fn vreinterpretq_s8_u16(a: uint16x8_t) -> int8x16_t {
    let a: uint16x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: int8x16_t = transmute(a);
        simd_shuffle(
            ret_val, ret_val, [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
        )
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s16_u16)
pub fn vreinterpretq_s16_u16(a: uint16x8_t) -> int16x8_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s16_u16)
pub fn vreinterpretq_s16_u16(a: uint16x8_t) -> int16x8_t {
    let a: uint16x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: int16x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s32_u16)
pub fn vreinterpretq_s32_u16(a: uint16x8_t) -> int32x4_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s32_u16)
pub fn vreinterpretq_s32_u16(a: uint16x8_t) -> int32x4_t {
    let a: uint16x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: int32x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s64_u16)
pub fn vreinterpretq_s64_u16(a: uint16x8_t) -> int64x2_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s64_u16)
pub fn vreinterpretq_s64_u16(a: uint16x8_t) -> int64x2_t {
    let a: uint16x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: int64x2_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u8_u16)
pub fn vreinterpretq_u8_u16(a: uint16x8_t) -> uint8x16_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u8_u16)
pub fn vreinterpretq_u8_u16(a: uint16x8_t) -> uint8x16_t {
    let a: uint16x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: uint8x16_t = transmute(a);
        simd_shuffle(
            ret_val, ret_val, [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
        )
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u32_u16)
pub fn vreinterpretq_u32_u16(a: uint16x8_t) -> uint32x4_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u32_u16)
pub fn vreinterpretq_u32_u16(a: uint16x8_t) -> uint32x4_t {
    let a: uint16x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: uint32x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u64_u16)
pub fn vreinterpretq_u64_u16(a: uint16x8_t) -> uint64x2_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u64_u16)
pub fn vreinterpretq_u64_u16(a: uint16x8_t) -> uint64x2_t {
    let a: uint16x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: uint64x2_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p8_u16)
pub fn vreinterpretq_p8_u16(a: uint16x8_t) -> poly8x16_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p8_u16)
pub fn vreinterpretq_p8_u16(a: uint16x8_t) -> poly8x16_t {
    let a: uint16x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: poly8x16_t = transmute(a);
        simd_shuffle(
            ret_val, ret_val, [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
        )
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p16_u16)
pub fn vreinterpretq_p16_u16(a: uint16x8_t) -> poly16x8_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p16_u16)
pub fn vreinterpretq_p16_u16(a: uint16x8_t) -> poly16x8_t {
    let a: uint16x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: poly16x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_f32_u32)
pub fn vreinterpret_f32_u32(a: uint32x2_t) -> float32x2_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_f32_u32)
pub fn vreinterpret_f32_u32(a: uint32x2_t) -> float32x2_t {
    let a: uint32x2_t = unsafe { simd_shuffle(a, a, [1, 0]) };
    {
        let ret_val: float32x2_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s8_u32)
pub fn vreinterpret_s8_u32(a: uint32x2_t) -> int8x8_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s8_u32)
pub fn vreinterpret_s8_u32(a: uint32x2_t) -> int8x8_t {
    let a: uint32x2_t = unsafe { simd_shuffle(a, a, [1, 0]) };
    {
        let ret_val: int8x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s16_u32)
pub fn vreinterpret_s16_u32(a: uint32x2_t) -> int16x4_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s16_u32)
pub fn vreinterpret_s16_u32(a: uint32x2_t) -> int16x4_t {
    let a: uint32x2_t = unsafe { simd_shuffle(a, a, [1, 0]) };
    {
        let ret_val: int16x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s32_u32)
pub fn vreinterpret_s32_u32(a: uint32x2_t) -> int32x2_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s32_u32)
pub fn vreinterpret_s32_u32(a: uint32x2_t) -> int32x2_t {
    let a: uint32x2_t = unsafe { simd_shuffle(a, a, [1, 0]) };
    {
        let ret_val: int32x2_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s64_u32)
pub fn vreinterpret_s64_u32(a: uint32x2_t) -> int64x1_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s64_u32)
pub fn vreinterpret_s64_u32(a: uint32x2_t) -> int64x1_t {
    let a: uint32x2_t = unsafe { simd_shuffle(a, a, [1, 0]) };
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u8_u32)
pub fn vreinterpret_u8_u32(a: uint32x2_t) -> uint8x8_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u8_u32)
pub fn vreinterpret_u8_u32(a: uint32x2_t) -> uint8x8_t {
    let a: uint32x2_t = unsafe { simd_shuffle(a, a, [1, 0]) };
    {
        let ret_val: uint8x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u16_u32)
pub fn vreinterpret_u16_u32(a: uint32x2_t) -> uint16x4_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u16_u32)
pub fn vreinterpret_u16_u32(a: uint32x2_t) -> uint16x4_t {
    let a: uint32x2_t = unsafe { simd_shuffle(a, a, [1, 0]) };
    {
        let ret_val: uint16x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u64_u32)
pub fn vreinterpret_u64_u32(a: uint32x2_t) -> uint64x1_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u64_u32)
pub fn vreinterpret_u64_u32(a: uint32x2_t) -> uint64x1_t {
    let a: uint32x2_t = unsafe { simd_shuffle(a, a, [1, 0]) };
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_p8_u32)
pub fn vreinterpret_p8_u32(a: uint32x2_t) -> poly8x8_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_p8_u32)
pub fn vreinterpret_p8_u32(a: uint32x2_t) -> poly8x8_t {
    let a: uint32x2_t = unsafe { simd_shuffle(a, a, [1, 0]) };
    {
        let ret_val: poly8x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_p16_u32)
pub fn vreinterpret_p16_u32(a: uint32x2_t) -> poly16x4_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_p16_u32)
pub fn vreinterpret_p16_u32(a: uint32x2_t) -> poly16x4_t {
    let a: uint32x2_t = unsafe { simd_shuffle(a, a, [1, 0]) };
    {
        let ret_val: poly16x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_f32_u32)
pub fn vreinterpretq_f32_u32(a: uint32x4_t) -> float32x4_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_f32_u32)
pub fn vreinterpretq_f32_u32(a: uint32x4_t) -> float32x4_t {
    let a: uint32x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    {
        let ret_val: float32x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s8_u32)
pub fn vreinterpretq_s8_u32(a: uint32x4_t) -> int8x16_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s8_u32)
pub fn vreinterpretq_s8_u32(a: uint32x4_t) -> int8x16_t {
    let a: uint32x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    {
        let ret_val: int8x16_t = transmute(a);
        simd_shuffle(
            ret_val, ret_val, [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
        )
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s16_u32)
pub fn vreinterpretq_s16_u32(a: uint32x4_t) -> int16x8_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s16_u32)
pub fn vreinterpretq_s16_u32(a: uint32x4_t) -> int16x8_t {
    let a: uint32x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    {
        let ret_val: int16x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s32_u32)
pub fn vreinterpretq_s32_u32(a: uint32x4_t) -> int32x4_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s32_u32)
pub fn vreinterpretq_s32_u32(a: uint32x4_t) -> int32x4_t {
    let a: uint32x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    {
        let ret_val: int32x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s64_u32)
pub fn vreinterpretq_s64_u32(a: uint32x4_t) -> int64x2_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s64_u32)
pub fn vreinterpretq_s64_u32(a: uint32x4_t) -> int64x2_t {
    let a: uint32x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    {
        let ret_val: int64x2_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u8_u32)
pub fn vreinterpretq_u8_u32(a: uint32x4_t) -> uint8x16_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u8_u32)
pub fn vreinterpretq_u8_u32(a: uint32x4_t) -> uint8x16_t {
    let a: uint32x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    {
        let ret_val: uint8x16_t = transmute(a);
        simd_shuffle(
            ret_val, ret_val, [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
        )
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u16_u32)
pub fn vreinterpretq_u16_u32(a: uint32x4_t) -> uint16x8_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u16_u32)
pub fn vreinterpretq_u16_u32(a: uint32x4_t) -> uint16x8_t {
    let a: uint32x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    {
        let ret_val: uint16x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u64_u32)
pub fn vreinterpretq_u64_u32(a: uint32x4_t) -> uint64x2_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u64_u32)
pub fn vreinterpretq_u64_u32(a: uint32x4_t) -> uint64x2_t {
    let a: uint32x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    {
        let ret_val: uint64x2_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p8_u32)
pub fn vreinterpretq_p8_u32(a: uint32x4_t) -> poly8x16_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p8_u32)
pub fn vreinterpretq_p8_u32(a: uint32x4_t) -> poly8x16_t {
    let a: uint32x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    {
        let ret_val: poly8x16_t = transmute(a);
        simd_shuffle(
            ret_val, ret_val, [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
        )
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p16_u32)
pub fn vreinterpretq_p16_u32(a: uint32x4_t) -> poly16x8_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p16_u32)
pub fn vreinterpretq_p16_u32(a: uint32x4_t) -> poly16x8_t {
    let a: uint32x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    {
        let ret_val: poly16x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_f32_u64)
pub fn vreinterpret_f32_u64(a: uint64x1_t) -> float32x2_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_f32_u64)
pub fn vreinterpret_f32_u64(a: uint64x1_t) -> float32x2_t {
    {
        let ret_val: float32x2_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s8_u64)
pub fn vreinterpret_s8_u64(a: uint64x1_t) -> int8x8_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s8_u64)
pub fn vreinterpret_s8_u64(a: uint64x1_t) -> int8x8_t {
    {
        let ret_val: int8x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s16_u64)
pub fn vreinterpret_s16_u64(a: uint64x1_t) -> int16x4_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s16_u64)
pub fn vreinterpret_s16_u64(a: uint64x1_t) -> int16x4_t {
    {
        let ret_val: int16x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s32_u64)
pub fn vreinterpret_s32_u64(a: uint64x1_t) -> int32x2_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s32_u64)
pub fn vreinterpret_s32_u64(a: uint64x1_t) -> int32x2_t {
    {
        let ret_val: int32x2_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s64_u64)
pub fn vreinterpret_s64_u64(a: uint64x1_t) -> int64x1_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u8_u64)
pub fn vreinterpret_u8_u64(a: uint64x1_t) -> uint8x8_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u8_u64)
pub fn vreinterpret_u8_u64(a: uint64x1_t) -> uint8x8_t {
    {
        let ret_val: uint8x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u16_u64)
pub fn vreinterpret_u16_u64(a: uint64x1_t) -> uint16x4_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u16_u64)
pub fn vreinterpret_u16_u64(a: uint64x1_t) -> uint16x4_t {
    {
        let ret_val: uint16x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u32_u64)
pub fn vreinterpret_u32_u64(a: uint64x1_t) -> uint32x2_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u32_u64)
pub fn vreinterpret_u32_u64(a: uint64x1_t) -> uint32x2_t {
    {
        let ret_val: uint32x2_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_p8_u64)
pub fn vreinterpret_p8_u64(a: uint64x1_t) -> poly8x8_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_p8_u64)
pub fn vreinterpret_p8_u64(a: uint64x1_t) -> poly8x8_t {
    {
        let ret_val: poly8x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_p16_u64)
pub fn vreinterpret_p16_u64(a: uint64x1_t) -> poly16x4_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_p16_u64)
pub fn vreinterpret_p16_u64(a: uint64x1_t) -> poly16x4_t {
    {
        let ret_val: poly16x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_f32_u64)
pub fn vreinterpretq_f32_u64(a: uint64x2_t) -> float32x4_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_f32_u64)
pub fn vreinterpretq_f32_u64(a: uint64x2_t) -> float32x4_t {
    let a: uint64x2_t = unsafe { simd_shuffle(a, a, [1, 0]) };
    {
        let ret_val: float32x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s8_u64)
pub fn vreinterpretq_s8_u64(a: uint64x2_t) -> int8x16_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s8_u64)
pub fn vreinterpretq_s8_u64(a: uint64x2_t) -> int8x16_t {
    let a: uint64x2_t = unsafe { simd_shuffle(a, a, [1, 0]) };
    {
        let ret_val: int8x16_t = transmute(a);
        simd_shuffle(
            ret_val, ret_val, [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
        )
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s16_u64)
pub fn vreinterpretq_s16_u64(a: uint64x2_t) -> int16x8_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s16_u64)
pub fn vreinterpretq_s16_u64(a: uint64x2_t) -> int16x8_t {
    let a: uint64x2_t = unsafe { simd_shuffle(a, a, [1, 0]) };
    {
        let ret_val: int16x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s32_u64)
pub fn vreinterpretq_s32_u64(a: uint64x2_t) -> int32x4_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s32_u64)
pub fn vreinterpretq_s32_u64(a: uint64x2_t) -> int32x4_t {
    let a: uint64x2_t = unsafe { simd_shuffle(a, a, [1, 0]) };
    {
        let ret_val: int32x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s64_u64)
pub fn vreinterpretq_s64_u64(a: uint64x2_t) -> int64x2_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s64_u64)
pub fn vreinterpretq_s64_u64(a: uint64x2_t) -> int64x2_t {
    let a: uint64x2_t = unsafe { simd_shuffle(a, a, [1, 0]) };
    {
        let ret_val: int64x2_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u8_u64)
pub fn vreinterpretq_u8_u64(a: uint64x2_t) -> uint8x16_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u8_u64)
pub fn vreinterpretq_u8_u64(a: uint64x2_t) -> uint8x16_t {
    let a: uint64x2_t = unsafe { simd_shuffle(a, a, [1, 0]) };
    {
        let ret_val: uint8x16_t = transmute(a);
        simd_shuffle(
            ret_val, ret_val, [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
        )
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u16_u64)
pub fn vreinterpretq_u16_u64(a: uint64x2_t) -> uint16x8_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u16_u64)
pub fn vreinterpretq_u16_u64(a: uint64x2_t) -> uint16x8_t {
    let a: uint64x2_t = unsafe { simd_shuffle(a, a, [1, 0]) };
    {
        let ret_val: uint16x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u32_u64)
pub fn vreinterpretq_u32_u64(a: uint64x2_t) -> uint32x4_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u32_u64)
pub fn vreinterpretq_u32_u64(a: uint64x2_t) -> uint32x4_t {
    let a: uint64x2_t = unsafe { simd_shuffle(a, a, [1, 0]) };
    {
        let ret_val: uint32x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p8_u64)
pub fn vreinterpretq_p8_u64(a: uint64x2_t) -> poly8x16_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p8_u64)
pub fn vreinterpretq_p8_u64(a: uint64x2_t) -> poly8x16_t {
    let a: uint64x2_t = unsafe { simd_shuffle(a, a, [1, 0]) };
    {
        let ret_val: poly8x16_t = transmute(a);
        simd_shuffle(
            ret_val, ret_val, [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
        )
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p16_u64)
pub fn vreinterpretq_p16_u64(a: uint64x2_t) -> poly16x8_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p16_u64)
pub fn vreinterpretq_p16_u64(a: uint64x2_t) -> poly16x8_t {
    let a: uint64x2_t = unsafe { simd_shuffle(a, a, [1, 0]) };
    {
        let ret_val: poly16x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_f32_p8)
pub fn vreinterpret_f32_p8(a: poly8x8_t) -> float32x2_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_f32_p8)
pub fn vreinterpret_f32_p8(a: poly8x8_t) -> float32x2_t {
    let a: poly8x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: float32x2_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s8_p8)
pub fn vreinterpret_s8_p8(a: poly8x8_t) -> int8x8_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s8_p8)
pub fn vreinterpret_s8_p8(a: poly8x8_t) -> int8x8_t {
    let a: poly8x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: int8x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s16_p8)
pub fn vreinterpret_s16_p8(a: poly8x8_t) -> int16x4_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s16_p8)
pub fn vreinterpret_s16_p8(a: poly8x8_t) -> int16x4_t {
    let a: poly8x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: int16x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s32_p8)
pub fn vreinterpret_s32_p8(a: poly8x8_t) -> int32x2_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s32_p8)
pub fn vreinterpret_s32_p8(a: poly8x8_t) -> int32x2_t {
    let a: poly8x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: int32x2_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s64_p8)
pub fn vreinterpret_s64_p8(a: poly8x8_t) -> int64x1_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s64_p8)
pub fn vreinterpret_s64_p8(a: poly8x8_t) -> int64x1_t {
    let a: poly8x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u8_p8)
pub fn vreinterpret_u8_p8(a: poly8x8_t) -> uint8x8_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u8_p8)
pub fn vreinterpret_u8_p8(a: poly8x8_t) -> uint8x8_t {
    let a: poly8x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: uint8x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u16_p8)
pub fn vreinterpret_u16_p8(a: poly8x8_t) -> uint16x4_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u16_p8)
pub fn vreinterpret_u16_p8(a: poly8x8_t) -> uint16x4_t {
    let a: poly8x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: uint16x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u32_p8)
pub fn vreinterpret_u32_p8(a: poly8x8_t) -> uint32x2_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u32_p8)
pub fn vreinterpret_u32_p8(a: poly8x8_t) -> uint32x2_t {
    let a: poly8x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: uint32x2_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u64_p8)
pub fn vreinterpret_u64_p8(a: poly8x8_t) -> uint64x1_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u64_p8)
pub fn vreinterpret_u64_p8(a: poly8x8_t) -> uint64x1_t {
    let a: poly8x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_p16_p8)
pub fn vreinterpret_p16_p8(a: poly8x8_t) -> poly16x4_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_p16_p8)
pub fn vreinterpret_p16_p8(a: poly8x8_t) -> poly16x4_t {
    let a: poly8x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: poly16x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_f32_p8)
pub fn vreinterpretq_f32_p8(a: poly8x16_t) -> float32x4_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_f32_p8)
pub fn vreinterpretq_f32_p8(a: poly8x16_t) -> float32x4_t {
    let a: poly8x16_t = unsafe {
        simd_shuffle(a, a, [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0])
    };
    {
        let ret_val: float32x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s8_p8)
pub fn vreinterpretq_s8_p8(a: poly8x16_t) -> int8x16_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s8_p8)
pub fn vreinterpretq_s8_p8(a: poly8x16_t) -> int8x16_t {
    let a: poly8x16_t = unsafe {
        simd_shuffle(a, a, [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0])
    };
    {
        let ret_val: int8x16_t = transmute(a);
        simd_shuffle(
            ret_val, ret_val, [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
        )
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s16_p8)
pub fn vreinterpretq_s16_p8(a: poly8x16_t) -> int16x8_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s16_p8)
pub fn vreinterpretq_s16_p8(a: poly8x16_t) -> int16x8_t {
    let a: poly8x16_t = unsafe {
        simd_shuffle(a, a, [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0])
    };
    {
        let ret_val: int16x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s32_p8)
pub fn vreinterpretq_s32_p8(a: poly8x16_t) -> int32x4_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s32_p8)
pub fn vreinterpretq_s32_p8(a: poly8x16_t) -> int32x4_t {
    let a: poly8x16_t = unsafe {
        simd_shuffle(a, a, [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0])
    };
    {
        let ret_val: int32x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s64_p8)
pub fn vreinterpretq_s64_p8(a: poly8x16_t) -> int64x2_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s64_p8)
pub fn vreinterpretq_s64_p8(a: poly8x16_t) -> int64x2_t {
    let a: poly8x16_t = unsafe {
        simd_shuffle(a, a, [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0])
    };
    {
        let ret_val: int64x2_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u8_p8)
pub fn vreinterpretq_u8_p8(a: poly8x16_t) -> uint8x16_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u8_p8)
pub fn vreinterpretq_u8_p8(a: poly8x16_t) -> uint8x16_t {
    let a: poly8x16_t = unsafe {
        simd_shuffle(a, a, [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0])
    };
    {
        let ret_val: uint8x16_t = transmute(a);
        simd_shuffle(
            ret_val, ret_val, [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
        )
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u16_p8)
pub fn vreinterpretq_u16_p8(a: poly8x16_t) -> uint16x8_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u16_p8)
pub fn vreinterpretq_u16_p8(a: poly8x16_t) -> uint16x8_t {
    let a: poly8x16_t = unsafe {
        simd_shuffle(a, a, [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0])
    };
    {
        let ret_val: uint16x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u32_p8)
pub fn vreinterpretq_u32_p8(a: poly8x16_t) -> uint32x4_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u32_p8)
pub fn vreinterpretq_u32_p8(a: poly8x16_t) -> uint32x4_t {
    let a: poly8x16_t = unsafe {
        simd_shuffle(a, a, [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0])
    };
    {
        let ret_val: uint32x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u64_p8)
pub fn vreinterpretq_u64_p8(a: poly8x16_t) -> uint64x2_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u64_p8)
pub fn vreinterpretq_u64_p8(a: poly8x16_t) -> uint64x2_t {
    let a: poly8x16_t = unsafe {
        simd_shuffle(a, a, [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0])
    };
    {
        let ret_val: uint64x2_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p16_p8)
pub fn vreinterpretq_p16_p8(a: poly8x16_t) -> poly16x8_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p16_p8)
pub fn vreinterpretq_p16_p8(a: poly8x16_t) -> poly16x8_t {
    let a: poly8x16_t = unsafe {
        simd_shuffle(a, a, [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0])
    };
    {
        let ret_val: poly16x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_f32_p16)
pub fn vreinterpret_f32_p16(a: poly16x4_t) -> float32x2_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_f32_p16)
pub fn vreinterpret_f32_p16(a: poly16x4_t) -> float32x2_t {
    let a: poly16x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    {
        let ret_val: float32x2_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s8_p16)
pub fn vreinterpret_s8_p16(a: poly16x4_t) -> int8x8_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s8_p16)
pub fn vreinterpret_s8_p16(a: poly16x4_t) -> int8x8_t {
    let a: poly16x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    {
        let ret_val: int8x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s16_p16)
pub fn vreinterpret_s16_p16(a: poly16x4_t) -> int16x4_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s16_p16)
pub fn vreinterpret_s16_p16(a: poly16x4_t) -> int16x4_t {
    let a: poly16x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    {
        let ret_val: int16x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s32_p16)
pub fn vreinterpret_s32_p16(a: poly16x4_t) -> int32x2_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s32_p16)
pub fn vreinterpret_s32_p16(a: poly16x4_t) -> int32x2_t {
    let a: poly16x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    {
        let ret_val: int32x2_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s64_p16)
pub fn vreinterpret_s64_p16(a: poly16x4_t) -> int64x1_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s64_p16)
pub fn vreinterpret_s64_p16(a: poly16x4_t) -> int64x1_t {
    let a: poly16x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u8_p16)
pub fn vreinterpret_u8_p16(a: poly16x4_t) -> uint8x8_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u8_p16)
pub fn vreinterpret_u8_p16(a: poly16x4_t) -> uint8x8_t {
    let a: poly16x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    {
        let ret_val: uint8x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u16_p16)
pub fn vreinterpret_u16_p16(a: poly16x4_t) -> uint16x4_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u16_p16)
pub fn vreinterpret_u16_p16(a: poly16x4_t) -> uint16x4_t {
    let a: poly16x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    {
        let ret_val: uint16x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u32_p16)
pub fn vreinterpret_u32_p16(a: poly16x4_t) -> uint32x2_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u32_p16)
pub fn vreinterpret_u32_p16(a: poly16x4_t) -> uint32x2_t {
    let a: poly16x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    {
        let ret_val: uint32x2_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u64_p16)
pub fn vreinterpret_u64_p16(a: poly16x4_t) -> uint64x1_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u64_p16)
pub fn vreinterpret_u64_p16(a: poly16x4_t) -> uint64x1_t {
    let a: poly16x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_p8_p16)
pub fn vreinterpret_p8_p16(a: poly16x4_t) -> poly8x8_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_p8_p16)
pub fn vreinterpret_p8_p16(a: poly16x4_t) -> poly8x8_t {
    let a: poly16x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    {
        let ret_val: poly8x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_f32_p16)
pub fn vreinterpretq_f32_p16(a: poly16x8_t) -> float32x4_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_f32_p16)
pub fn vreinterpretq_f32_p16(a: poly16x8_t) -> float32x4_t {
    let a: poly16x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: float32x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s8_p16)
pub fn vreinterpretq_s8_p16(a: poly16x8_t) -> int8x16_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s8_p16)
pub fn vreinterpretq_s8_p16(a: poly16x8_t) -> int8x16_t {
    let a: poly16x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: int8x16_t = transmute(a);
        simd_shuffle(
            ret_val, ret_val, [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
        )
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s16_p16)
pub fn vreinterpretq_s16_p16(a: poly16x8_t) -> int16x8_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s16_p16)
pub fn vreinterpretq_s16_p16(a: poly16x8_t) -> int16x8_t {
    let a: poly16x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: int16x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s32_p16)
pub fn vreinterpretq_s32_p16(a: poly16x8_t) -> int32x4_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s32_p16)
pub fn vreinterpretq_s32_p16(a: poly16x8_t) -> int32x4_t {
    let a: poly16x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: int32x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s64_p16)
pub fn vreinterpretq_s64_p16(a: poly16x8_t) -> int64x2_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s64_p16)
pub fn vreinterpretq_s64_p16(a: poly16x8_t) -> int64x2_t {
    let a: poly16x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: int64x2_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u8_p16)
pub fn vreinterpretq_u8_p16(a: poly16x8_t) -> uint8x16_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u8_p16)
pub fn vreinterpretq_u8_p16(a: poly16x8_t) -> uint8x16_t {
    let a: poly16x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: uint8x16_t = transmute(a);
        simd_shuffle(
            ret_val, ret_val, [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
        )
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u16_p16)
pub fn vreinterpretq_u16_p16(a: poly16x8_t) -> uint16x8_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u16_p16)
pub fn vreinterpretq_u16_p16(a: poly16x8_t) -> uint16x8_t {
    let a: poly16x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: uint16x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u32_p16)
pub fn vreinterpretq_u32_p16(a: poly16x8_t) -> uint32x4_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u32_p16)
pub fn vreinterpretq_u32_p16(a: poly16x8_t) -> uint32x4_t {
    let a: poly16x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: uint32x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u64_p16)
pub fn vreinterpretq_u64_p16(a: poly16x8_t) -> uint64x2_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u64_p16)
pub fn vreinterpretq_u64_p16(a: poly16x8_t) -> uint64x2_t {
    let a: poly16x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: uint64x2_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p8_p16)
pub fn vreinterpretq_p8_p16(a: poly16x8_t) -> poly8x16_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p8_p16)
pub fn vreinterpretq_p8_p16(a: poly16x8_t) -> poly8x16_t {
    let a: poly16x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: poly8x16_t = transmute(a);
        simd_shuffle(
            ret_val, ret_val, [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
        )
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s8_p128)
pub fn vreinterpretq_s8_p128(a: p128) -> int8x16_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s8_p128)
pub fn vreinterpretq_s8_p128(a: p128) -> int8x16_t {
    {
        let ret_val: int8x16_t = transmute(a);
        simd_shuffle(
            ret_val, ret_val, [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
        )
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s16_p128)
pub fn vreinterpretq_s16_p128(a: p128) -> int16x8_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s16_p128)
pub fn vreinterpretq_s16_p128(a: p128) -> int16x8_t {
    {
        let ret_val: int16x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s32_p128)
pub fn vreinterpretq_s32_p128(a: p128) -> int32x4_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s32_p128)
pub fn vreinterpretq_s32_p128(a: p128) -> int32x4_t {
    {
        let ret_val: int32x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s64_p128)
pub fn vreinterpretq_s64_p128(a: p128) -> int64x2_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s64_p128)
pub fn vreinterpretq_s64_p128(a: p128) -> int64x2_t {
    {
        let ret_val: int64x2_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u8_p128)
pub fn vreinterpretq_u8_p128(a: p128) -> uint8x16_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u8_p128)
pub fn vreinterpretq_u8_p128(a: p128) -> uint8x16_t {
    {
        let ret_val: uint8x16_t = transmute(a);
        simd_shuffle(
            ret_val, ret_val, [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
        )
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u16_p128)
pub fn vreinterpretq_u16_p128(a: p128) -> uint16x8_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u16_p128)
pub fn vreinterpretq_u16_p128(a: p128) -> uint16x8_t {
    {
        let ret_val: uint16x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u32_p128)
pub fn vreinterpretq_u32_p128(a: p128) -> uint32x4_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u32_p128)
pub fn vreinterpretq_u32_p128(a: p128) -> uint32x4_t {
    {
        let ret_val: uint32x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u64_p128)
pub fn vreinterpretq_u64_p128(a: p128) -> uint64x2_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u64_p128)
pub fn vreinterpretq_u64_p128(a: p128) -> uint64x2_t {
    {
        let ret_val: uint64x2_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p8_p128)
pub fn vreinterpretq_p8_p128(a: p128) -> poly8x16_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p8_p128)
pub fn vreinterpretq_p8_p128(a: p128) -> poly8x16_t {
    {
        let ret_val: poly8x16_t = transmute(a);
        simd_shuffle(
            ret_val, ret_val, [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
        )
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p16_p128)
pub fn vreinterpretq_p16_p128(a: p128) -> poly16x8_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p16_p128)
pub fn vreinterpretq_p16_p128(a: p128) -> poly16x8_t {
    {
        let ret_val: poly16x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p64_p128)
pub fn vreinterpretq_p64_p128(a: p128) -> poly64x2_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p64_p128)
pub fn vreinterpretq_p64_p128(a: p128) -> poly64x2_t {
    {
        let ret_val: poly64x2_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_p64_s8)
pub fn vreinterpret_p64_s8(a: int8x8_t) -> poly64x1_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_p64_s8)
pub fn vreinterpret_p64_s8(a: int8x8_t) -> poly64x1_t {
    let a: int8x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p128_s8)
pub fn vreinterpretq_p128_s8(a: int8x16_t) -> p128 {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p128_s8)
pub fn vreinterpretq_p128_s8(a: int8x16_t) -> p128 {
    let a: int8x16_t = unsafe {
        simd_shuffle(a, a, [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0])
    };
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p64_s8)
pub fn vreinterpretq_p64_s8(a: int8x16_t) -> poly64x2_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p64_s8)
pub fn vreinterpretq_p64_s8(a: int8x16_t) -> poly64x2_t {
    let a: int8x16_t = unsafe {
        simd_shuffle(a, a, [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0])
    };
    {
        let ret_val: poly64x2_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_p64_s16)
pub fn vreinterpret_p64_s16(a: int16x4_t) -> poly64x1_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_p64_s16)
pub fn vreinterpret_p64_s16(a: int16x4_t) -> poly64x1_t {
    let a: int16x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p128_s16)
pub fn vreinterpretq_p128_s16(a: int16x8_t) -> p128 {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p128_s16)
pub fn vreinterpretq_p128_s16(a: int16x8_t) -> p128 {
    let a: int16x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p64_s16)
pub fn vreinterpretq_p64_s16(a: int16x8_t) -> poly64x2_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p64_s16)
pub fn vreinterpretq_p64_s16(a: int16x8_t) -> poly64x2_t {
    let a: int16x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: poly64x2_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_p64_s32)
pub fn vreinterpret_p64_s32(a: int32x2_t) -> poly64x1_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_p64_s32)
pub fn vreinterpret_p64_s32(a: int32x2_t) -> poly64x1_t {
    let a: int32x2_t = unsafe { simd_shuffle(a, a, [1, 0]) };
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p128_s32)
pub fn vreinterpretq_p128_s32(a: int32x4_t) -> p128 {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p128_s32)
pub fn vreinterpretq_p128_s32(a: int32x4_t) -> p128 {
    let a: int32x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p64_s32)
pub fn vreinterpretq_p64_s32(a: int32x4_t) -> poly64x2_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p64_s32)
pub fn vreinterpretq_p64_s32(a: int32x4_t) -> poly64x2_t {
    let a: int32x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    {
        let ret_val: poly64x2_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p128_s64)
pub fn vreinterpretq_p128_s64(a: int64x2_t) -> p128 {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p128_s64)
pub fn vreinterpretq_p128_s64(a: int64x2_t) -> p128 {
    let a: int64x2_t = unsafe { simd_shuffle(a, a, [1, 0]) };
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_p64_u8)
pub fn vreinterpret_p64_u8(a: uint8x8_t) -> poly64x1_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_p64_u8)
pub fn vreinterpret_p64_u8(a: uint8x8_t) -> poly64x1_t {
    let a: uint8x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p128_u8)
pub fn vreinterpretq_p128_u8(a: uint8x16_t) -> p128 {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p128_u8)
pub fn vreinterpretq_p128_u8(a: uint8x16_t) -> p128 {
    let a: uint8x16_t = unsafe {
        simd_shuffle(a, a, [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0])
    };
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p64_u8)
pub fn vreinterpretq_p64_u8(a: uint8x16_t) -> poly64x2_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p64_u8)
pub fn vreinterpretq_p64_u8(a: uint8x16_t) -> poly64x2_t {
    let a: uint8x16_t = unsafe {
        simd_shuffle(a, a, [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0])
    };
    {
        let ret_val: poly64x2_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_p64_u16)
pub fn vreinterpret_p64_u16(a: uint16x4_t) -> poly64x1_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_p64_u16)
pub fn vreinterpret_p64_u16(a: uint16x4_t) -> poly64x1_t {
    let a: uint16x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p128_u16)
pub fn vreinterpretq_p128_u16(a: uint16x8_t) -> p128 {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p128_u16)
pub fn vreinterpretq_p128_u16(a: uint16x8_t) -> p128 {
    let a: uint16x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p64_u16)
pub fn vreinterpretq_p64_u16(a: uint16x8_t) -> poly64x2_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p64_u16)
pub fn vreinterpretq_p64_u16(a: uint16x8_t) -> poly64x2_t {
    let a: uint16x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: poly64x2_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_p64_u32)
pub fn vreinterpret_p64_u32(a: uint32x2_t) -> poly64x1_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_p64_u32)
pub fn vreinterpret_p64_u32(a: uint32x2_t) -> poly64x1_t {
    let a: uint32x2_t = unsafe { simd_shuffle(a, a, [1, 0]) };
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p128_u32)
pub fn vreinterpretq_p128_u32(a: uint32x4_t) -> p128 {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p128_u32)
pub fn vreinterpretq_p128_u32(a: uint32x4_t) -> p128 {
    let a: uint32x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p64_u32)
pub fn vreinterpretq_p64_u32(a: uint32x4_t) -> poly64x2_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p64_u32)
pub fn vreinterpretq_p64_u32(a: uint32x4_t) -> poly64x2_t {
    let a: uint32x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    {
        let ret_val: poly64x2_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p128_u64)
pub fn vreinterpretq_p128_u64(a: uint64x2_t) -> p128 {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p128_u64)
pub fn vreinterpretq_p128_u64(a: uint64x2_t) -> p128 {
    let a: uint64x2_t = unsafe { simd_shuffle(a, a, [1, 0]) };
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_p64_p8)
pub fn vreinterpret_p64_p8(a: poly8x8_t) -> poly64x1_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_p64_p8)
pub fn vreinterpret_p64_p8(a: poly8x8_t) -> poly64x1_t {
    let a: poly8x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p128_p8)
pub fn vreinterpretq_p128_p8(a: poly8x16_t) -> p128 {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p128_p8)
pub fn vreinterpretq_p128_p8(a: poly8x16_t) -> p128 {
    let a: poly8x16_t = unsafe {
        simd_shuffle(a, a, [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0])
    };
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p64_p8)
pub fn vreinterpretq_p64_p8(a: poly8x16_t) -> poly64x2_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p64_p8)
pub fn vreinterpretq_p64_p8(a: poly8x16_t) -> poly64x2_t {
    let a: poly8x16_t = unsafe {
        simd_shuffle(a, a, [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0])
    };
    {
        let ret_val: poly64x2_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_p64_p16)
pub fn vreinterpret_p64_p16(a: poly16x4_t) -> poly64x1_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_p64_p16)
pub fn vreinterpret_p64_p16(a: poly16x4_t) -> poly64x1_t {
    let a: poly16x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p128_p16)
pub fn vreinterpretq_p128_p16(a: poly16x8_t) -> p128 {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p128_p16)
pub fn vreinterpretq_p128_p16(a: poly16x8_t) -> p128 {
    let a: poly16x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p64_p16)
pub fn vreinterpretq_p64_p16(a: poly16x8_t) -> poly64x2_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p64_p16)
pub fn vreinterpretq_p64_p16(a: poly16x8_t) -> poly64x2_t {
    let a: poly16x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: poly64x2_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s8_p64)
pub fn vreinterpret_s8_p64(a: poly64x1_t) -> int8x8_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s8_p64)
pub fn vreinterpret_s8_p64(a: poly64x1_t) -> int8x8_t {
    {
        let ret_val: int8x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s16_p64)
pub fn vreinterpret_s16_p64(a: poly64x1_t) -> int16x4_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s16_p64)
pub fn vreinterpret_s16_p64(a: poly64x1_t) -> int16x4_t {
    {
        let ret_val: int16x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s32_p64)
pub fn vreinterpret_s32_p64(a: poly64x1_t) -> int32x2_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_s32_p64)
pub fn vreinterpret_s32_p64(a: poly64x1_t) -> int32x2_t {
    {
        let ret_val: int32x2_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u8_p64)
pub fn vreinterpret_u8_p64(a: poly64x1_t) -> uint8x8_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u8_p64)
pub fn vreinterpret_u8_p64(a: poly64x1_t) -> uint8x8_t {
    {
        let ret_val: uint8x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u16_p64)
pub fn vreinterpret_u16_p64(a: poly64x1_t) -> uint16x4_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u16_p64)
pub fn vreinterpret_u16_p64(a: poly64x1_t) -> uint16x4_t {
    {
        let ret_val: uint16x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u32_p64)
pub fn vreinterpret_u32_p64(a: poly64x1_t) -> uint32x2_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_u32_p64)
pub fn vreinterpret_u32_p64(a: poly64x1_t) -> uint32x2_t {
    {
        let ret_val: uint32x2_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_p8_p64)
pub fn vreinterpret_p8_p64(a: poly64x1_t) -> poly8x8_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_p8_p64)
pub fn vreinterpret_p8_p64(a: poly64x1_t) -> poly8x8_t {
    {
        let ret_val: poly8x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_p16_p64)
pub fn vreinterpret_p16_p64(a: poly64x1_t) -> poly16x4_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpret_p16_p64)
pub fn vreinterpret_p16_p64(a: poly64x1_t) -> poly16x4_t {
    {
        let ret_val: poly16x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p128_p64)
pub fn vreinterpretq_p128_p64(a: poly64x2_t) -> p128 {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p128_p64)
pub fn vreinterpretq_p128_p64(a: poly64x2_t) -> p128 {
    let a: poly64x2_t = unsafe { simd_shuffle(a, a, [1, 0]) };
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s8_p64)
pub fn vreinterpretq_s8_p64(a: poly64x2_t) -> int8x16_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s8_p64)
pub fn vreinterpretq_s8_p64(a: poly64x2_t) -> int8x16_t {
    let a: poly64x2_t = unsafe { simd_shuffle(a, a, [1, 0]) };
    {
        let ret_val: int8x16_t = transmute(a);
        simd_shuffle(
            ret_val, ret_val, [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
        )
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s16_p64)
pub fn vreinterpretq_s16_p64(a: poly64x2_t) -> int16x8_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s16_p64)
pub fn vreinterpretq_s16_p64(a: poly64x2_t) -> int16x8_t {
    let a: poly64x2_t = unsafe { simd_shuffle(a, a, [1, 0]) };
    {
        let ret_val: int16x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s32_p64)
pub fn vreinterpretq_s32_p64(a: poly64x2_t) -> int32x4_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_s32_p64)
pub fn vreinterpretq_s32_p64(a: poly64x2_t) -> int32x4_t {
    let a: poly64x2_t = unsafe { simd_shuffle(a, a, [1, 0]) };
    {
        let ret_val: int32x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u8_p64)
pub fn vreinterpretq_u8_p64(a: poly64x2_t) -> uint8x16_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u8_p64)
pub fn vreinterpretq_u8_p64(a: poly64x2_t) -> uint8x16_t {
    let a: poly64x2_t = unsafe { simd_shuffle(a, a, [1, 0]) };
    {
        let ret_val: uint8x16_t = transmute(a);
        simd_shuffle(
            ret_val, ret_val, [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
        )
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u16_p64)
pub fn vreinterpretq_u16_p64(a: poly64x2_t) -> uint16x8_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u16_p64)
pub fn vreinterpretq_u16_p64(a: poly64x2_t) -> uint16x8_t {
    let a: poly64x2_t = unsafe { simd_shuffle(a, a, [1, 0]) };
    {
        let ret_val: uint16x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u32_p64)
pub fn vreinterpretq_u32_p64(a: poly64x2_t) -> uint32x4_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_u32_p64)
pub fn vreinterpretq_u32_p64(a: poly64x2_t) -> uint32x4_t {
    let a: poly64x2_t = unsafe { simd_shuffle(a, a, [1, 0]) };
    {
        let ret_val: uint32x4_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p8_p64)
pub fn vreinterpretq_p8_p64(a: poly64x2_t) -> poly8x16_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p8_p64)
pub fn vreinterpretq_p8_p64(a: poly64x2_t) -> poly8x16_t {
    let a: poly64x2_t = unsafe { simd_shuffle(a, a, [1, 0]) };
    {
        let ret_val: poly8x16_t = transmute(a);
        simd_shuffle(
            ret_val, ret_val, [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
        )
    }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p16_p64)
pub fn vreinterpretq_p16_p64(a: poly64x2_t) -> poly16x8_t {
    { transmute(a) }
}
///Vector reinterpret cast operation
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vreinterpretq_p16_p64)
pub fn vreinterpretq_p16_p64(a: poly64x2_t) -> poly16x8_t {
    let a: poly64x2_t = unsafe { simd_shuffle(a, a, [1, 0]) };
    {
        let ret_val: poly16x8_t = transmute(a);
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Reversing vector elements (swap endianness)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrev16_p8)
pub fn vrev16_p8(a: poly8x8_t) -> poly8x8_t {
    { simd_shuffle(a, a, [1, 0, 3, 2, 5, 4, 7, 6]) }
}
///Reversing vector elements (swap endianness)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrev16_s8)
pub fn vrev16_s8(a: int8x8_t) -> int8x8_t {
    { simd_shuffle(a, a, [1, 0, 3, 2, 5, 4, 7, 6]) }
}
///Reversing vector elements (swap endianness)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrev16_u8)
pub fn vrev16_u8(a: uint8x8_t) -> uint8x8_t {
    { simd_shuffle(a, a, [1, 0, 3, 2, 5, 4, 7, 6]) }
}
///Reversing vector elements (swap endianness)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrev16q_p8)
pub fn vrev16q_p8(a: poly8x16_t) -> poly8x16_t {
    { simd_shuffle(a, a, [1, 0, 3, 2, 5, 4, 7, 6, 9, 8, 11, 10, 13, 12, 15, 14]) }
}
///Reversing vector elements (swap endianness)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrev16q_s8)
pub fn vrev16q_s8(a: int8x16_t) -> int8x16_t {
    { simd_shuffle(a, a, [1, 0, 3, 2, 5, 4, 7, 6, 9, 8, 11, 10, 13, 12, 15, 14]) }
}
///Reversing vector elements (swap endianness)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrev16q_u8)
pub fn vrev16q_u8(a: uint8x16_t) -> uint8x16_t {
    { simd_shuffle(a, a, [1, 0, 3, 2, 5, 4, 7, 6, 9, 8, 11, 10, 13, 12, 15, 14]) }
}
///Reversing vector elements (swap endianness)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrev32_p16)
pub fn vrev32_p16(a: poly16x4_t) -> poly16x4_t {
    { simd_shuffle(a, a, [1, 0, 3, 2]) }
}
///Reversing vector elements (swap endianness)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrev32_p8)
pub fn vrev32_p8(a: poly8x8_t) -> poly8x8_t {
    { simd_shuffle(a, a, [3, 2, 1, 0, 7, 6, 5, 4]) }
}
///Reversing vector elements (swap endianness)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrev32_s16)
pub fn vrev32_s16(a: int16x4_t) -> int16x4_t {
    { simd_shuffle(a, a, [1, 0, 3, 2]) }
}
///Reversing vector elements (swap endianness)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrev32_s8)
pub fn vrev32_s8(a: int8x8_t) -> int8x8_t {
    { simd_shuffle(a, a, [3, 2, 1, 0, 7, 6, 5, 4]) }
}
///Reversing vector elements (swap endianness)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrev32_u16)
pub fn vrev32_u16(a: uint16x4_t) -> uint16x4_t {
    { simd_shuffle(a, a, [1, 0, 3, 2]) }
}
///Reversing vector elements (swap endianness)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrev32_u8)
pub fn vrev32_u8(a: uint8x8_t) -> uint8x8_t {
    { simd_shuffle(a, a, [3, 2, 1, 0, 7, 6, 5, 4]) }
}
///Reversing vector elements (swap endianness)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrev32q_p16)
pub fn vrev32q_p16(a: poly16x8_t) -> poly16x8_t {
    { simd_shuffle(a, a, [1, 0, 3, 2, 5, 4, 7, 6]) }
}
///Reversing vector elements (swap endianness)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrev32q_p8)
pub fn vrev32q_p8(a: poly8x16_t) -> poly8x16_t {
    { simd_shuffle(a, a, [3, 2, 1, 0, 7, 6, 5, 4, 11, 10, 9, 8, 15, 14, 13, 12]) }
}
///Reversing vector elements (swap endianness)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrev32q_s16)
pub fn vrev32q_s16(a: int16x8_t) -> int16x8_t {
    { simd_shuffle(a, a, [1, 0, 3, 2, 5, 4, 7, 6]) }
}
///Reversing vector elements (swap endianness)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrev32q_s8)
pub fn vrev32q_s8(a: int8x16_t) -> int8x16_t {
    { simd_shuffle(a, a, [3, 2, 1, 0, 7, 6, 5, 4, 11, 10, 9, 8, 15, 14, 13, 12]) }
}
///Reversing vector elements (swap endianness)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrev32q_u16)
pub fn vrev32q_u16(a: uint16x8_t) -> uint16x8_t {
    { simd_shuffle(a, a, [1, 0, 3, 2, 5, 4, 7, 6]) }
}
///Reversing vector elements (swap endianness)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrev32q_u8)
pub fn vrev32q_u8(a: uint8x16_t) -> uint8x16_t {
    { simd_shuffle(a, a, [3, 2, 1, 0, 7, 6, 5, 4, 11, 10, 9, 8, 15, 14, 13, 12]) }
}
///Reversing vector elements (swap endianness)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrev64_f32)
pub fn vrev64_f32(a: float32x2_t) -> float32x2_t {
    { simd_shuffle(a, a, [1, 0]) }
}
///Reversing vector elements (swap endianness)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrev64_p16)
pub fn vrev64_p16(a: poly16x4_t) -> poly16x4_t {
    { simd_shuffle(a, a, [3, 2, 1, 0]) }
}
///Reversing vector elements (swap endianness)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrev64_p8)
pub fn vrev64_p8(a: poly8x8_t) -> poly8x8_t {
    { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) }
}
///Reversing vector elements (swap endianness)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrev64_s16)
pub fn vrev64_s16(a: int16x4_t) -> int16x4_t {
    { simd_shuffle(a, a, [3, 2, 1, 0]) }
}
///Reversing vector elements (swap endianness)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrev64_s32)
pub fn vrev64_s32(a: int32x2_t) -> int32x2_t {
    { simd_shuffle(a, a, [1, 0]) }
}
///Reversing vector elements (swap endianness)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrev64_s8)
pub fn vrev64_s8(a: int8x8_t) -> int8x8_t {
    { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) }
}
///Reversing vector elements (swap endianness)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrev64_u16)
pub fn vrev64_u16(a: uint16x4_t) -> uint16x4_t {
    { simd_shuffle(a, a, [3, 2, 1, 0]) }
}
///Reversing vector elements (swap endianness)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrev64_u32)
pub fn vrev64_u32(a: uint32x2_t) -> uint32x2_t {
    { simd_shuffle(a, a, [1, 0]) }
}
///Reversing vector elements (swap endianness)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrev64_u8)
pub fn vrev64_u8(a: uint8x8_t) -> uint8x8_t {
    { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) }
}
///Reversing vector elements (swap endianness)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrev64q_f32)
pub fn vrev64q_f32(a: float32x4_t) -> float32x4_t {
    { simd_shuffle(a, a, [1, 0, 3, 2]) }
}
///Reversing vector elements (swap endianness)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrev64q_p16)
pub fn vrev64q_p16(a: poly16x8_t) -> poly16x8_t {
    { simd_shuffle(a, a, [3, 2, 1, 0, 7, 6, 5, 4]) }
}
///Reversing vector elements (swap endianness)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrev64q_p8)
pub fn vrev64q_p8(a: poly8x16_t) -> poly8x16_t {
    { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0, 15, 14, 13, 12, 11, 10, 9, 8]) }
}
///Reversing vector elements (swap endianness)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrev64q_s16)
pub fn vrev64q_s16(a: int16x8_t) -> int16x8_t {
    { simd_shuffle(a, a, [3, 2, 1, 0, 7, 6, 5, 4]) }
}
///Reversing vector elements (swap endianness)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrev64q_s32)
pub fn vrev64q_s32(a: int32x4_t) -> int32x4_t {
    { simd_shuffle(a, a, [1, 0, 3, 2]) }
}
///Reversing vector elements (swap endianness)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrev64q_s8)
pub fn vrev64q_s8(a: int8x16_t) -> int8x16_t {
    { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0, 15, 14, 13, 12, 11, 10, 9, 8]) }
}
///Reversing vector elements (swap endianness)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrev64q_u16)
pub fn vrev64q_u16(a: uint16x8_t) -> uint16x8_t {
    { simd_shuffle(a, a, [3, 2, 1, 0, 7, 6, 5, 4]) }
}
///Reversing vector elements (swap endianness)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrev64q_u32)
pub fn vrev64q_u32(a: uint32x4_t) -> uint32x4_t {
    { simd_shuffle(a, a, [1, 0, 3, 2]) }
}
///Reversing vector elements (swap endianness)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrev64q_u8)
pub fn vrev64q_u8(a: uint8x16_t) -> uint8x16_t {
    { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0, 15, 14, 13, 12, 11, 10, 9, 8]) }
}
///Reverse elements in 64-bit doublewords
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrev64_f16)
pub fn vrev64_f16(a: float16x4_t) -> float16x4_t {
    { simd_shuffle(a, a, [3, 2, 1, 0]) }
}
///Reverse elements in 64-bit doublewords
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrev64q_f16)
pub fn vrev64q_f16(a: float16x8_t) -> float16x8_t {
    { simd_shuffle(a, a, [3, 2, 1, 0, 7, 6, 5, 4]) }
}
///Rounding halving add
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrhadd_s8)
pub fn vrhadd_s8(a: int8x8_t, b: int8x8_t) -> int8x8_t {
    { _vrhadd_s8(a, b) }
}
///Rounding halving add
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrhaddq_s8)
pub fn vrhaddq_s8(a: int8x16_t, b: int8x16_t) -> int8x16_t {
    { _vrhaddq_s8(a, b) }
}
///Rounding halving add
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrhadd_s16)
pub fn vrhadd_s16(a: int16x4_t, b: int16x4_t) -> int16x4_t {
    { _vrhadd_s16(a, b) }
}
///Rounding halving add
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrhaddq_s16)
pub fn vrhaddq_s16(a: int16x8_t, b: int16x8_t) -> int16x8_t {
    { _vrhaddq_s16(a, b) }
}
///Rounding halving add
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrhadd_s32)
pub fn vrhadd_s32(a: int32x2_t, b: int32x2_t) -> int32x2_t {
    { _vrhadd_s32(a, b) }
}
///Rounding halving add
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrhaddq_s32)
pub fn vrhaddq_s32(a: int32x4_t, b: int32x4_t) -> int32x4_t {
    { _vrhaddq_s32(a, b) }
}
///Rounding halving add
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrhadd_u8)
pub fn vrhadd_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
    { _vrhadd_u8(a, b) }
}
///Rounding halving add
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrhaddq_u8)
pub fn vrhaddq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
    { _vrhaddq_u8(a, b) }
}
///Rounding halving add
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrhadd_u16)
pub fn vrhadd_u16(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
    { _vrhadd_u16(a, b) }
}
///Rounding halving add
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrhaddq_u16)
pub fn vrhaddq_u16(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
    { _vrhaddq_u16(a, b) }
}
///Rounding halving add
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrhadd_u32)
pub fn vrhadd_u32(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
    { _vrhadd_u32(a, b) }
}
///Rounding halving add
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrhaddq_u32)
pub fn vrhaddq_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
    { _vrhaddq_u32(a, b) }
}
///Floating-point round to integral, to nearest with ties to even
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrndn_f16)
pub fn vrndn_f16(a: float16x4_t) -> float16x4_t {
    { _vrndn_f16(a) }
}
///Floating-point round to integral, to nearest with ties to even
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrndnq_f16)
pub fn vrndnq_f16(a: float16x8_t) -> float16x8_t {
    { _vrndnq_f16(a) }
}
///Floating-point round to integral, to nearest with ties to even
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrndn_f32)
pub fn vrndn_f32(a: float32x2_t) -> float32x2_t {
    { _vrndn_f32(a) }
}
///Floating-point round to integral, to nearest with ties to even
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrndnq_f32)
pub fn vrndnq_f32(a: float32x4_t) -> float32x4_t {
    { _vrndnq_f32(a) }
}
///Signed rounding shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrshl_s8)
pub fn vrshl_s8(a: int8x8_t, b: int8x8_t) -> int8x8_t {
    { _vrshl_s8(a, b) }
}
///Signed rounding shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrshlq_s8)
pub fn vrshlq_s8(a: int8x16_t, b: int8x16_t) -> int8x16_t {
    { _vrshlq_s8(a, b) }
}
///Signed rounding shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrshl_s16)
pub fn vrshl_s16(a: int16x4_t, b: int16x4_t) -> int16x4_t {
    { _vrshl_s16(a, b) }
}
///Signed rounding shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrshlq_s16)
pub fn vrshlq_s16(a: int16x8_t, b: int16x8_t) -> int16x8_t {
    { _vrshlq_s16(a, b) }
}
///Signed rounding shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrshl_s32)
pub fn vrshl_s32(a: int32x2_t, b: int32x2_t) -> int32x2_t {
    { _vrshl_s32(a, b) }
}
///Signed rounding shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrshlq_s32)
pub fn vrshlq_s32(a: int32x4_t, b: int32x4_t) -> int32x4_t {
    { _vrshlq_s32(a, b) }
}
///Signed rounding shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrshl_s64)
pub fn vrshl_s64(a: int64x1_t, b: int64x1_t) -> int64x1_t {
    { _vrshl_s64(a, b) }
}
///Signed rounding shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrshlq_s64)
pub fn vrshlq_s64(a: int64x2_t, b: int64x2_t) -> int64x2_t {
    { _vrshlq_s64(a, b) }
}
///Unsigned rounding shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrshl_u8)
pub fn vrshl_u8(a: uint8x8_t, b: int8x8_t) -> uint8x8_t {
    { _vrshl_u8(a, b) }
}
///Unsigned rounding shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrshlq_u8)
pub fn vrshlq_u8(a: uint8x16_t, b: int8x16_t) -> uint8x16_t {
    { _vrshlq_u8(a, b) }
}
///Unsigned rounding shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrshl_u16)
pub fn vrshl_u16(a: uint16x4_t, b: int16x4_t) -> uint16x4_t {
    { _vrshl_u16(a, b) }
}
///Unsigned rounding shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrshlq_u16)
pub fn vrshlq_u16(a: uint16x8_t, b: int16x8_t) -> uint16x8_t {
    { _vrshlq_u16(a, b) }
}
///Unsigned rounding shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrshl_u32)
pub fn vrshl_u32(a: uint32x2_t, b: int32x2_t) -> uint32x2_t {
    { _vrshl_u32(a, b) }
}
///Unsigned rounding shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrshlq_u32)
pub fn vrshlq_u32(a: uint32x4_t, b: int32x4_t) -> uint32x4_t {
    { _vrshlq_u32(a, b) }
}
///Unsigned rounding shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrshl_u64)
pub fn vrshl_u64(a: uint64x1_t, b: int64x1_t) -> uint64x1_t {
    { _vrshl_u64(a, b) }
}
///Unsigned rounding shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrshlq_u64)
pub fn vrshlq_u64(a: uint64x2_t, b: int64x2_t) -> uint64x2_t {
    { _vrshlq_u64(a, b) }
}
///Signed rounding shift right
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrshr_n_s8)
pub fn vrshr_n_s8<const N: i32>(a: int8x8_t) -> int8x8_t {
    static_assert!(N >= 1 && N <= 8);
    vrshl_s8(a, vdup_n_s8(-N as _))
}
///Signed rounding shift right
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrshrq_n_s8)
pub fn vrshrq_n_s8<const N: i32>(a: int8x16_t) -> int8x16_t {
    static_assert!(N >= 1 && N <= 8);
    vrshlq_s8(a, vdupq_n_s8(-N as _))
}
///Signed rounding shift right
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrshr_n_s16)
pub fn vrshr_n_s16<const N: i32>(a: int16x4_t) -> int16x4_t {
    static_assert!(N >= 1 && N <= 16);
    vrshl_s16(a, vdup_n_s16(-N as _))
}
///Signed rounding shift right
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrshrq_n_s16)
pub fn vrshrq_n_s16<const N: i32>(a: int16x8_t) -> int16x8_t {
    static_assert!(N >= 1 && N <= 16);
    vrshlq_s16(a, vdupq_n_s16(-N as _))
}
///Signed rounding shift right
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrshr_n_s32)
pub fn vrshr_n_s32<const N: i32>(a: int32x2_t) -> int32x2_t {
    static_assert!(N >= 1 && N <= 32);
    vrshl_s32(a, vdup_n_s32(-N as _))
}
///Signed rounding shift right
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrshrq_n_s32)
pub fn vrshrq_n_s32<const N: i32>(a: int32x4_t) -> int32x4_t {
    static_assert!(N >= 1 && N <= 32);
    vrshlq_s32(a, vdupq_n_s32(-N as _))
}
///Signed rounding shift right
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrshr_n_s64)
pub fn vrshr_n_s64<const N: i32>(a: int64x1_t) -> int64x1_t {
    static_assert!(N >= 1 && N <= 64);
    vrshl_s64(a, vdup_n_s64(-N as _))
}
///Signed rounding shift right
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrshrq_n_s64)
pub fn vrshrq_n_s64<const N: i32>(a: int64x2_t) -> int64x2_t {
    static_assert!(N >= 1 && N <= 64);
    vrshlq_s64(a, vdupq_n_s64(-N as _))
}
///Unsigned rounding shift right
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrshr_n_u8)
pub fn vrshr_n_u8<const N: i32>(a: uint8x8_t) -> uint8x8_t {
    static_assert!(N >= 1 && N <= 8);
    vrshl_u8(a, vdup_n_s8(-N as _))
}
///Unsigned rounding shift right
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrshrq_n_u8)
pub fn vrshrq_n_u8<const N: i32>(a: uint8x16_t) -> uint8x16_t {
    static_assert!(N >= 1 && N <= 8);
    vrshlq_u8(a, vdupq_n_s8(-N as _))
}
///Unsigned rounding shift right
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrshr_n_u16)
pub fn vrshr_n_u16<const N: i32>(a: uint16x4_t) -> uint16x4_t {
    static_assert!(N >= 1 && N <= 16);
    vrshl_u16(a, vdup_n_s16(-N as _))
}
///Unsigned rounding shift right
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrshrq_n_u16)
pub fn vrshrq_n_u16<const N: i32>(a: uint16x8_t) -> uint16x8_t {
    static_assert!(N >= 1 && N <= 16);
    vrshlq_u16(a, vdupq_n_s16(-N as _))
}
///Unsigned rounding shift right
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrshr_n_u32)
pub fn vrshr_n_u32<const N: i32>(a: uint32x2_t) -> uint32x2_t {
    static_assert!(N >= 1 && N <= 32);
    vrshl_u32(a, vdup_n_s32(-N as _))
}
///Unsigned rounding shift right
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrshrq_n_u32)
pub fn vrshrq_n_u32<const N: i32>(a: uint32x4_t) -> uint32x4_t {
    static_assert!(N >= 1 && N <= 32);
    vrshlq_u32(a, vdupq_n_s32(-N as _))
}
///Unsigned rounding shift right
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrshr_n_u64)
pub fn vrshr_n_u64<const N: i32>(a: uint64x1_t) -> uint64x1_t {
    static_assert!(N >= 1 && N <= 64);
    vrshl_u64(a, vdup_n_s64(-N as _))
}
///Unsigned rounding shift right
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrshrq_n_u64)
pub fn vrshrq_n_u64<const N: i32>(a: uint64x2_t) -> uint64x2_t {
    static_assert!(N >= 1 && N <= 64);
    vrshlq_u64(a, vdupq_n_s64(-N as _))
}
///Rounding shift right narrow
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrshrn_n_s16)
pub fn vrshrn_n_s16<const N: i32>(a: int16x8_t) -> int8x8_t {
    static_assert!(N >= 1 && N <= 8);
    {
        _vrshrn_n_s16(
            a,
            const {
                int16x8_t([
                    -N as i16,
                    -N as i16,
                    -N as i16,
                    -N as i16,
                    -N as i16,
                    -N as i16,
                    -N as i16,
                    -N as i16,
                ])
            },
        )
    }
}
///Rounding shift right narrow
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrshrn_n_s32)
pub fn vrshrn_n_s32<const N: i32>(a: int32x4_t) -> int16x4_t {
    static_assert!(N >= 1 && N <= 16);
    {
        _vrshrn_n_s32(
            a,
            const { int32x4_t([-N as i32, -N as i32, -N as i32, -N as i32]) },
        )
    }
}
///Rounding shift right narrow
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrshrn_n_s64)
pub fn vrshrn_n_s64<const N: i32>(a: int64x2_t) -> int32x2_t {
    static_assert!(N >= 1 && N <= 32);
    { _vrshrn_n_s64(a, const { int64x2_t([-N as i64, -N as i64]) }) }
}
///Rounding shift right narrow
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrshrn_n_s16)
pub fn vrshrn_n_s16<const N: i32>(a: int16x8_t) -> int8x8_t {
    static_assert!(N >= 1 && N <= 8);
    { _vrshrn_n_s16(a, N) }
}
///Rounding shift right narrow
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrshrn_n_s32)
pub fn vrshrn_n_s32<const N: i32>(a: int32x4_t) -> int16x4_t {
    static_assert!(N >= 1 && N <= 16);
    { _vrshrn_n_s32(a, N) }
}
///Rounding shift right narrow
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrshrn_n_s64)
pub fn vrshrn_n_s64<const N: i32>(a: int64x2_t) -> int32x2_t {
    static_assert!(N >= 1 && N <= 32);
    { _vrshrn_n_s64(a, N) }
}
///Rounding shift right narrow
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrshrn_n_u16)
pub fn vrshrn_n_u16<const N: i32>(a: uint16x8_t) -> uint8x8_t {
    static_assert!(N >= 1 && N <= 8);
    { transmute(vrshrn_n_s16::<N>(transmute(a))) }
}
///Rounding shift right narrow
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrshrn_n_u32)
pub fn vrshrn_n_u32<const N: i32>(a: uint32x4_t) -> uint16x4_t {
    static_assert!(N >= 1 && N <= 16);
    { transmute(vrshrn_n_s32::<N>(transmute(a))) }
}
///Rounding shift right narrow
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrshrn_n_u64)
pub fn vrshrn_n_u64<const N: i32>(a: uint64x2_t) -> uint32x2_t {
    static_assert!(N >= 1 && N <= 32);
    { transmute(vrshrn_n_s64::<N>(transmute(a))) }
}
///Reciprocal square-root estimate.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrsqrte_f16)
pub fn vrsqrte_f16(a: float16x4_t) -> float16x4_t {
    { _vrsqrte_f16(a) }
}
///Reciprocal square-root estimate.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrsqrteq_f16)
pub fn vrsqrteq_f16(a: float16x8_t) -> float16x8_t {
    { _vrsqrteq_f16(a) }
}
///Reciprocal square-root estimate.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrsqrte_f32)
pub fn vrsqrte_f32(a: float32x2_t) -> float32x2_t {
    { _vrsqrte_f32(a) }
}
///Reciprocal square-root estimate.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrsqrteq_f32)
pub fn vrsqrteq_f32(a: float32x4_t) -> float32x4_t {
    { _vrsqrteq_f32(a) }
}
///Unsigned reciprocal square root estimate
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrsqrte_u32)
pub fn vrsqrte_u32(a: uint32x2_t) -> uint32x2_t {
    { _vrsqrte_u32(a) }
}
///Unsigned reciprocal square root estimate
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrsqrteq_u32)
pub fn vrsqrteq_u32(a: uint32x4_t) -> uint32x4_t {
    { _vrsqrteq_u32(a) }
}
///Floating-point reciprocal square root step
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrsqrts_f16)
pub fn vrsqrts_f16(a: float16x4_t, b: float16x4_t) -> float16x4_t {
    { _vrsqrts_f16(a, b) }
}
///Floating-point reciprocal square root step
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrsqrtsq_f16)
pub fn vrsqrtsq_f16(a: float16x8_t, b: float16x8_t) -> float16x8_t {
    { _vrsqrtsq_f16(a, b) }
}
///Floating-point reciprocal square root step
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrsqrts_f32)
pub fn vrsqrts_f32(a: float32x2_t, b: float32x2_t) -> float32x2_t {
    { _vrsqrts_f32(a, b) }
}
///Floating-point reciprocal square root step
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrsqrtsq_f32)
pub fn vrsqrtsq_f32(a: float32x4_t, b: float32x4_t) -> float32x4_t {
    { _vrsqrtsq_f32(a, b) }
}
///Signed rounding shift right and accumulate
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrsra_n_s8)
pub fn vrsra_n_s8<const N: i32>(a: int8x8_t, b: int8x8_t) -> int8x8_t {
    static_assert!(N >= 1 && N <= 8);
    { simd_add(a, vrshr_n_s8::<N>(b)) }
}
///Signed rounding shift right and accumulate
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrsraq_n_s8)
pub fn vrsraq_n_s8<const N: i32>(a: int8x16_t, b: int8x16_t) -> int8x16_t {
    static_assert!(N >= 1 && N <= 8);
    { simd_add(a, vrshrq_n_s8::<N>(b)) }
}
///Signed rounding shift right and accumulate
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrsra_n_s16)
pub fn vrsra_n_s16<const N: i32>(a: int16x4_t, b: int16x4_t) -> int16x4_t {
    static_assert!(N >= 1 && N <= 16);
    { simd_add(a, vrshr_n_s16::<N>(b)) }
}
///Signed rounding shift right and accumulate
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrsraq_n_s16)
pub fn vrsraq_n_s16<const N: i32>(a: int16x8_t, b: int16x8_t) -> int16x8_t {
    static_assert!(N >= 1 && N <= 16);
    { simd_add(a, vrshrq_n_s16::<N>(b)) }
}
///Signed rounding shift right and accumulate
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrsra_n_s32)
pub fn vrsra_n_s32<const N: i32>(a: int32x2_t, b: int32x2_t) -> int32x2_t {
    static_assert!(N >= 1 && N <= 32);
    { simd_add(a, vrshr_n_s32::<N>(b)) }
}
///Signed rounding shift right and accumulate
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrsraq_n_s32)
pub fn vrsraq_n_s32<const N: i32>(a: int32x4_t, b: int32x4_t) -> int32x4_t {
    static_assert!(N >= 1 && N <= 32);
    { simd_add(a, vrshrq_n_s32::<N>(b)) }
}
///Signed rounding shift right and accumulate
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrsra_n_s64)
pub fn vrsra_n_s64<const N: i32>(a: int64x1_t, b: int64x1_t) -> int64x1_t {
    static_assert!(N >= 1 && N <= 64);
    { simd_add(a, vrshr_n_s64::<N>(b)) }
}
///Signed rounding shift right and accumulate
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrsraq_n_s64)
pub fn vrsraq_n_s64<const N: i32>(a: int64x2_t, b: int64x2_t) -> int64x2_t {
    static_assert!(N >= 1 && N <= 64);
    { simd_add(a, vrshrq_n_s64::<N>(b)) }
}
///Unsigned rounding shift right and accumulate
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrsra_n_u8)
pub fn vrsra_n_u8<const N: i32>(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
    static_assert!(N >= 1 && N <= 8);
    { simd_add(a, vrshr_n_u8::<N>(b)) }
}
///Unsigned rounding shift right and accumulate
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrsraq_n_u8)
pub fn vrsraq_n_u8<const N: i32>(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
    static_assert!(N >= 1 && N <= 8);
    { simd_add(a, vrshrq_n_u8::<N>(b)) }
}
///Unsigned rounding shift right and accumulate
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrsra_n_u16)
pub fn vrsra_n_u16<const N: i32>(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
    static_assert!(N >= 1 && N <= 16);
    { simd_add(a, vrshr_n_u16::<N>(b)) }
}
///Unsigned rounding shift right and accumulate
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrsraq_n_u16)
pub fn vrsraq_n_u16<const N: i32>(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
    static_assert!(N >= 1 && N <= 16);
    { simd_add(a, vrshrq_n_u16::<N>(b)) }
}
///Unsigned rounding shift right and accumulate
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrsra_n_u32)
pub fn vrsra_n_u32<const N: i32>(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
    static_assert!(N >= 1 && N <= 32);
    { simd_add(a, vrshr_n_u32::<N>(b)) }
}
///Unsigned rounding shift right and accumulate
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrsraq_n_u32)
pub fn vrsraq_n_u32<const N: i32>(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
    static_assert!(N >= 1 && N <= 32);
    { simd_add(a, vrshrq_n_u32::<N>(b)) }
}
///Unsigned rounding shift right and accumulate
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrsra_n_u64)
pub fn vrsra_n_u64<const N: i32>(a: uint64x1_t, b: uint64x1_t) -> uint64x1_t {
    static_assert!(N >= 1 && N <= 64);
    { simd_add(a, vrshr_n_u64::<N>(b)) }
}
///Unsigned rounding shift right and accumulate
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrsraq_n_u64)
pub fn vrsraq_n_u64<const N: i32>(a: uint64x2_t, b: uint64x2_t) -> uint64x2_t {
    static_assert!(N >= 1 && N <= 64);
    { simd_add(a, vrshrq_n_u64::<N>(b)) }
}
///Rounding subtract returning high narrow
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrsubhn_s16)
pub fn vrsubhn_s16(a: int16x8_t, b: int16x8_t) -> int8x8_t {
    { _vrsubhn_s16(a, b) }
}
///Rounding subtract returning high narrow
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrsubhn_s32)
pub fn vrsubhn_s32(a: int32x4_t, b: int32x4_t) -> int16x4_t {
    { _vrsubhn_s32(a, b) }
}
///Rounding subtract returning high narrow
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrsubhn_s64)
pub fn vrsubhn_s64(a: int64x2_t, b: int64x2_t) -> int32x2_t {
    { _vrsubhn_s64(a, b) }
}
///Rounding subtract returning high narrow
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrsubhn_u16)
pub fn vrsubhn_u16(a: uint16x8_t, b: uint16x8_t) -> uint8x8_t {
    { transmute(vrsubhn_s16(transmute(a), transmute(b))) }
}
///Rounding subtract returning high narrow
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrsubhn_u16)
pub fn vrsubhn_u16(a: uint16x8_t, b: uint16x8_t) -> uint8x8_t {
    let a: uint16x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    let b: uint16x8_t = unsafe { simd_shuffle(b, b, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: uint8x8_t = transmute(vrsubhn_s16(transmute(a), transmute(b)));
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Rounding subtract returning high narrow
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrsubhn_u32)
pub fn vrsubhn_u32(a: uint32x4_t, b: uint32x4_t) -> uint16x4_t {
    { transmute(vrsubhn_s32(transmute(a), transmute(b))) }
}
///Rounding subtract returning high narrow
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrsubhn_u32)
pub fn vrsubhn_u32(a: uint32x4_t, b: uint32x4_t) -> uint16x4_t {
    let a: uint32x4_t = unsafe { simd_shuffle(a, a, [3, 2, 1, 0]) };
    let b: uint32x4_t = unsafe { simd_shuffle(b, b, [3, 2, 1, 0]) };
    {
        let ret_val: uint16x4_t = transmute(vrsubhn_s32(transmute(a), transmute(b)));
        simd_shuffle(ret_val, ret_val, [3, 2, 1, 0])
    }
}
///Rounding subtract returning high narrow
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrsubhn_u64)
pub fn vrsubhn_u64(a: uint64x2_t, b: uint64x2_t) -> uint32x2_t {
    { transmute(vrsubhn_s64(transmute(a), transmute(b))) }
}
///Rounding subtract returning high narrow
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrsubhn_u64)
pub fn vrsubhn_u64(a: uint64x2_t, b: uint64x2_t) -> uint32x2_t {
    let a: uint64x2_t = unsafe { simd_shuffle(a, a, [1, 0]) };
    let b: uint64x2_t = unsafe { simd_shuffle(b, b, [1, 0]) };
    {
        let ret_val: uint32x2_t = transmute(vrsubhn_s64(transmute(a), transmute(b)));
        simd_shuffle(ret_val, ret_val, [1, 0])
    }
}
///Insert vector element from another vector element
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vset_lane_f16)
pub fn vset_lane_f16<const LANE: i32>(a: f16, b: float16x4_t) -> float16x4_t {
    static_assert_uimm_bits!(LANE, 2);
    { simd_insert!(b, LANE as u32, a) }
}
///Insert vector element from another vector element
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsetq_lane_f16)
pub fn vsetq_lane_f16<const LANE: i32>(a: f16, b: float16x8_t) -> float16x8_t {
    static_assert_uimm_bits!(LANE, 3);
    { simd_insert!(b, LANE as u32, a) }
}
///Insert vector element from another vector element
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vset_lane_f32)
pub fn vset_lane_f32<const LANE: i32>(a: f32, b: float32x2_t) -> float32x2_t {
    static_assert_uimm_bits!(LANE, 1);
    { simd_insert!(b, LANE as u32, a) }
}
///Insert vector element from another vector element
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsetq_lane_f32)
pub fn vsetq_lane_f32<const LANE: i32>(a: f32, b: float32x4_t) -> float32x4_t {
    static_assert_uimm_bits!(LANE, 2);
    { simd_insert!(b, LANE as u32, a) }
}
///Insert vector element from another vector element
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vset_lane_s8)
pub fn vset_lane_s8<const LANE: i32>(a: i8, b: int8x8_t) -> int8x8_t {
    static_assert_uimm_bits!(LANE, 3);
    { simd_insert!(b, LANE as u32, a) }
}
///Insert vector element from another vector element
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsetq_lane_s8)
pub fn vsetq_lane_s8<const LANE: i32>(a: i8, b: int8x16_t) -> int8x16_t {
    static_assert_uimm_bits!(LANE, 4);
    { simd_insert!(b, LANE as u32, a) }
}
///Insert vector element from another vector element
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vset_lane_s16)
pub fn vset_lane_s16<const LANE: i32>(a: i16, b: int16x4_t) -> int16x4_t {
    static_assert_uimm_bits!(LANE, 2);
    { simd_insert!(b, LANE as u32, a) }
}
///Insert vector element from another vector element
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsetq_lane_s16)
pub fn vsetq_lane_s16<const LANE: i32>(a: i16, b: int16x8_t) -> int16x8_t {
    static_assert_uimm_bits!(LANE, 3);
    { simd_insert!(b, LANE as u32, a) }
}
///Insert vector element from another vector element
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vset_lane_s32)
pub fn vset_lane_s32<const LANE: i32>(a: i32, b: int32x2_t) -> int32x2_t {
    static_assert_uimm_bits!(LANE, 1);
    { simd_insert!(b, LANE as u32, a) }
}
///Insert vector element from another vector element
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsetq_lane_s32)
pub fn vsetq_lane_s32<const LANE: i32>(a: i32, b: int32x4_t) -> int32x4_t {
    static_assert_uimm_bits!(LANE, 2);
    { simd_insert!(b, LANE as u32, a) }
}
///Insert vector element from another vector element
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsetq_lane_s64)
pub fn vsetq_lane_s64<const LANE: i32>(a: i64, b: int64x2_t) -> int64x2_t {
    static_assert_uimm_bits!(LANE, 1);
    { simd_insert!(b, LANE as u32, a) }
}
///Insert vector element from another vector element
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vset_lane_u8)
pub fn vset_lane_u8<const LANE: i32>(a: u8, b: uint8x8_t) -> uint8x8_t {
    static_assert_uimm_bits!(LANE, 3);
    { simd_insert!(b, LANE as u32, a) }
}
///Insert vector element from another vector element
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsetq_lane_u8)
pub fn vsetq_lane_u8<const LANE: i32>(a: u8, b: uint8x16_t) -> uint8x16_t {
    static_assert_uimm_bits!(LANE, 4);
    { simd_insert!(b, LANE as u32, a) }
}
///Insert vector element from another vector element
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vset_lane_u16)
pub fn vset_lane_u16<const LANE: i32>(a: u16, b: uint16x4_t) -> uint16x4_t {
    static_assert_uimm_bits!(LANE, 2);
    { simd_insert!(b, LANE as u32, a) }
}
///Insert vector element from another vector element
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsetq_lane_u16)
pub fn vsetq_lane_u16<const LANE: i32>(a: u16, b: uint16x8_t) -> uint16x8_t {
    static_assert_uimm_bits!(LANE, 3);
    { simd_insert!(b, LANE as u32, a) }
}
///Insert vector element from another vector element
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vset_lane_u32)
pub fn vset_lane_u32<const LANE: i32>(a: u32, b: uint32x2_t) -> uint32x2_t {
    static_assert_uimm_bits!(LANE, 1);
    { simd_insert!(b, LANE as u32, a) }
}
///Insert vector element from another vector element
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsetq_lane_u32)
pub fn vsetq_lane_u32<const LANE: i32>(a: u32, b: uint32x4_t) -> uint32x4_t {
    static_assert_uimm_bits!(LANE, 2);
    { simd_insert!(b, LANE as u32, a) }
}
///Insert vector element from another vector element
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsetq_lane_u64)
pub fn vsetq_lane_u64<const LANE: i32>(a: u64, b: uint64x2_t) -> uint64x2_t {
    static_assert_uimm_bits!(LANE, 1);
    { simd_insert!(b, LANE as u32, a) }
}
///Insert vector element from another vector element
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vset_lane_p8)
pub fn vset_lane_p8<const LANE: i32>(a: p8, b: poly8x8_t) -> poly8x8_t {
    static_assert_uimm_bits!(LANE, 3);
    { simd_insert!(b, LANE as u32, a) }
}
///Insert vector element from another vector element
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsetq_lane_p8)
pub fn vsetq_lane_p8<const LANE: i32>(a: p8, b: poly8x16_t) -> poly8x16_t {
    static_assert_uimm_bits!(LANE, 4);
    { simd_insert!(b, LANE as u32, a) }
}
///Insert vector element from another vector element
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vset_lane_p16)
pub fn vset_lane_p16<const LANE: i32>(a: p16, b: poly16x4_t) -> poly16x4_t {
    static_assert_uimm_bits!(LANE, 2);
    { simd_insert!(b, LANE as u32, a) }
}
///Insert vector element from another vector element
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsetq_lane_p16)
pub fn vsetq_lane_p16<const LANE: i32>(a: p16, b: poly16x8_t) -> poly16x8_t {
    static_assert_uimm_bits!(LANE, 3);
    { simd_insert!(b, LANE as u32, a) }
}
///Insert vector element from another vector element
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vset_lane_p64)
pub fn vset_lane_p64<const LANE: i32>(a: p64, b: poly64x1_t) -> poly64x1_t {
    static_assert!(LANE == 0);
    { simd_insert!(b, LANE as u32, a) }
}
///Insert vector element from another vector element
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vset_lane_s64)
pub fn vset_lane_s64<const LANE: i32>(a: i64, b: int64x1_t) -> int64x1_t {
    static_assert!(LANE == 0);
    { simd_insert!(b, LANE as u32, a) }
}
///Insert vector element from another vector element
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vset_lane_u64)
pub fn vset_lane_u64<const LANE: i32>(a: u64, b: uint64x1_t) -> uint64x1_t {
    static_assert!(LANE == 0);
    { simd_insert!(b, LANE as u32, a) }
}
///Insert vector element from another vector element
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsetq_lane_p64)
pub fn vsetq_lane_p64<const LANE: i32>(a: p64, b: poly64x2_t) -> poly64x2_t {
    static_assert_uimm_bits!(LANE, 1);
    { simd_insert!(b, LANE as u32, a) }
}
///SHA1 hash update accelerator, choose.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsha1cq_u32)
pub fn vsha1cq_u32(hash_abcd: uint32x4_t, hash_e: u32, wk: uint32x4_t) -> uint32x4_t {
    { _vsha1cq_u32(hash_abcd, hash_e, wk) }
}
///SHA1 fixed rotate.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsha1h_u32)
pub fn vsha1h_u32(hash_e: u32) -> u32 {
    { _vsha1h_u32(hash_e) }
}
///SHA1 hash update accelerator, majority
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsha1mq_u32)
pub fn vsha1mq_u32(hash_abcd: uint32x4_t, hash_e: u32, wk: uint32x4_t) -> uint32x4_t {
    { _vsha1mq_u32(hash_abcd, hash_e, wk) }
}
///SHA1 hash update accelerator, parity
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsha1pq_u32)
pub fn vsha1pq_u32(hash_abcd: uint32x4_t, hash_e: u32, wk: uint32x4_t) -> uint32x4_t {
    { _vsha1pq_u32(hash_abcd, hash_e, wk) }
}
///SHA1 schedule update accelerator, first part.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsha1su0q_u32)
pub fn vsha1su0q_u32(
    w0_3: uint32x4_t,
    w4_7: uint32x4_t,
    w8_11: uint32x4_t,
) -> uint32x4_t {
    { _vsha1su0q_u32(w0_3, w4_7, w8_11) }
}
///SHA1 schedule update accelerator, second part.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsha1su1q_u32)
pub fn vsha1su1q_u32(tw0_3: uint32x4_t, w12_15: uint32x4_t) -> uint32x4_t {
    { _vsha1su1q_u32(tw0_3, w12_15) }
}
///SHA1 schedule update accelerator, upper part.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsha256h2q_u32)
pub fn vsha256h2q_u32(
    hash_abcd: uint32x4_t,
    hash_efgh: uint32x4_t,
    wk: uint32x4_t,
) -> uint32x4_t {
    { _vsha256h2q_u32(hash_abcd, hash_efgh, wk) }
}
///SHA1 schedule update accelerator, first part.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsha256hq_u32)
pub fn vsha256hq_u32(
    hash_abcd: uint32x4_t,
    hash_efgh: uint32x4_t,
    wk: uint32x4_t,
) -> uint32x4_t {
    { _vsha256hq_u32(hash_abcd, hash_efgh, wk) }
}
///SHA256 schedule update accelerator, first part.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsha256su0q_u32)
pub fn vsha256su0q_u32(w0_3: uint32x4_t, w4_7: uint32x4_t) -> uint32x4_t {
    { _vsha256su0q_u32(w0_3, w4_7) }
}
///SHA256 schedule update accelerator, second part.
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsha256su1q_u32)
pub fn vsha256su1q_u32(
    tw0_3: uint32x4_t,
    w8_11: uint32x4_t,
    w12_15: uint32x4_t,
) -> uint32x4_t {
    { _vsha256su1q_u32(tw0_3, w8_11, w12_15) }
}
///Shift Right and Insert (immediate)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vshiftins_v16i8)
fn vshiftins_v16i8(a: int8x16_t, b: int8x16_t, c: int8x16_t) -> int8x16_t {
    { _vshiftins_v16i8(a, b, c) }
}
///Shift Right and Insert (immediate)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vshiftins_v1i64)
fn vshiftins_v1i64(a: int64x1_t, b: int64x1_t, c: int64x1_t) -> int64x1_t {
    { _vshiftins_v1i64(a, b, c) }
}
///Shift Right and Insert (immediate)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vshiftins_v2i32)
fn vshiftins_v2i32(a: int32x2_t, b: int32x2_t, c: int32x2_t) -> int32x2_t {
    { _vshiftins_v2i32(a, b, c) }
}
///Shift Right and Insert (immediate)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vshiftins_v2i64)
fn vshiftins_v2i64(a: int64x2_t, b: int64x2_t, c: int64x2_t) -> int64x2_t {
    { _vshiftins_v2i64(a, b, c) }
}
///Shift Right and Insert (immediate)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vshiftins_v4i16)
fn vshiftins_v4i16(a: int16x4_t, b: int16x4_t, c: int16x4_t) -> int16x4_t {
    { _vshiftins_v4i16(a, b, c) }
}
///Shift Right and Insert (immediate)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vshiftins_v4i32)
fn vshiftins_v4i32(a: int32x4_t, b: int32x4_t, c: int32x4_t) -> int32x4_t {
    { _vshiftins_v4i32(a, b, c) }
}
///Shift Right and Insert (immediate)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vshiftins_v8i16)
fn vshiftins_v8i16(a: int16x8_t, b: int16x8_t, c: int16x8_t) -> int16x8_t {
    { _vshiftins_v8i16(a, b, c) }
}
///Shift Right and Insert (immediate)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vshiftins_v8i8)
fn vshiftins_v8i8(a: int8x8_t, b: int8x8_t, c: int8x8_t) -> int8x8_t {
    { _vshiftins_v8i8(a, b, c) }
}
///Shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vshl_n_s8)
pub fn vshl_n_s8<const N: i32>(a: int8x8_t) -> int8x8_t {
    static_assert_uimm_bits!(N, 3);
    { simd_shl(a, vdup_n_s8(N as _)) }
}
///Shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vshlq_n_s8)
pub fn vshlq_n_s8<const N: i32>(a: int8x16_t) -> int8x16_t {
    static_assert_uimm_bits!(N, 3);
    { simd_shl(a, vdupq_n_s8(N as _)) }
}
///Shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vshl_n_s16)
pub fn vshl_n_s16<const N: i32>(a: int16x4_t) -> int16x4_t {
    static_assert_uimm_bits!(N, 4);
    { simd_shl(a, vdup_n_s16(N as _)) }
}
///Shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vshlq_n_s16)
pub fn vshlq_n_s16<const N: i32>(a: int16x8_t) -> int16x8_t {
    static_assert_uimm_bits!(N, 4);
    { simd_shl(a, vdupq_n_s16(N as _)) }
}
///Shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vshl_n_s32)
pub fn vshl_n_s32<const N: i32>(a: int32x2_t) -> int32x2_t {
    static_assert_uimm_bits!(N, 5);
    { simd_shl(a, vdup_n_s32(N as _)) }
}
///Shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vshlq_n_s32)
pub fn vshlq_n_s32<const N: i32>(a: int32x4_t) -> int32x4_t {
    static_assert_uimm_bits!(N, 5);
    { simd_shl(a, vdupq_n_s32(N as _)) }
}
///Shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vshl_n_s64)
pub fn vshl_n_s64<const N: i32>(a: int64x1_t) -> int64x1_t {
    static_assert_uimm_bits!(N, 6);
    { simd_shl(a, vdup_n_s64(N as _)) }
}
///Shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vshlq_n_s64)
pub fn vshlq_n_s64<const N: i32>(a: int64x2_t) -> int64x2_t {
    static_assert_uimm_bits!(N, 6);
    { simd_shl(a, vdupq_n_s64(N as _)) }
}
///Shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vshl_n_u8)
pub fn vshl_n_u8<const N: i32>(a: uint8x8_t) -> uint8x8_t {
    static_assert_uimm_bits!(N, 3);
    { simd_shl(a, vdup_n_u8(N as _)) }
}
///Shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vshlq_n_u8)
pub fn vshlq_n_u8<const N: i32>(a: uint8x16_t) -> uint8x16_t {
    static_assert_uimm_bits!(N, 3);
    { simd_shl(a, vdupq_n_u8(N as _)) }
}
///Shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vshl_n_u16)
pub fn vshl_n_u16<const N: i32>(a: uint16x4_t) -> uint16x4_t {
    static_assert_uimm_bits!(N, 4);
    { simd_shl(a, vdup_n_u16(N as _)) }
}
///Shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vshlq_n_u16)
pub fn vshlq_n_u16<const N: i32>(a: uint16x8_t) -> uint16x8_t {
    static_assert_uimm_bits!(N, 4);
    { simd_shl(a, vdupq_n_u16(N as _)) }
}
///Shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vshl_n_u32)
pub fn vshl_n_u32<const N: i32>(a: uint32x2_t) -> uint32x2_t {
    static_assert_uimm_bits!(N, 5);
    { simd_shl(a, vdup_n_u32(N as _)) }
}
///Shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vshlq_n_u32)
pub fn vshlq_n_u32<const N: i32>(a: uint32x4_t) -> uint32x4_t {
    static_assert_uimm_bits!(N, 5);
    { simd_shl(a, vdupq_n_u32(N as _)) }
}
///Shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vshl_n_u64)
pub fn vshl_n_u64<const N: i32>(a: uint64x1_t) -> uint64x1_t {
    static_assert_uimm_bits!(N, 6);
    { simd_shl(a, vdup_n_u64(N as _)) }
}
///Shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vshlq_n_u64)
pub fn vshlq_n_u64<const N: i32>(a: uint64x2_t) -> uint64x2_t {
    static_assert_uimm_bits!(N, 6);
    { simd_shl(a, vdupq_n_u64(N as _)) }
}
///Signed Shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vshl_s8)
pub fn vshl_s8(a: int8x8_t, b: int8x8_t) -> int8x8_t {
    { _vshl_s8(a, b) }
}
///Signed Shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vshlq_s8)
pub fn vshlq_s8(a: int8x16_t, b: int8x16_t) -> int8x16_t {
    { _vshlq_s8(a, b) }
}
///Signed Shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vshl_s16)
pub fn vshl_s16(a: int16x4_t, b: int16x4_t) -> int16x4_t {
    { _vshl_s16(a, b) }
}
///Signed Shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vshlq_s16)
pub fn vshlq_s16(a: int16x8_t, b: int16x8_t) -> int16x8_t {
    { _vshlq_s16(a, b) }
}
///Signed Shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vshl_s32)
pub fn vshl_s32(a: int32x2_t, b: int32x2_t) -> int32x2_t {
    { _vshl_s32(a, b) }
}
///Signed Shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vshlq_s32)
pub fn vshlq_s32(a: int32x4_t, b: int32x4_t) -> int32x4_t {
    { _vshlq_s32(a, b) }
}
///Signed Shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vshl_s64)
pub fn vshl_s64(a: int64x1_t, b: int64x1_t) -> int64x1_t {
    { _vshl_s64(a, b) }
}
///Signed Shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vshlq_s64)
pub fn vshlq_s64(a: int64x2_t, b: int64x2_t) -> int64x2_t {
    { _vshlq_s64(a, b) }
}
///Unsigned Shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vshl_u8)
pub fn vshl_u8(a: uint8x8_t, b: int8x8_t) -> uint8x8_t {
    { _vshl_u8(a, b) }
}
///Unsigned Shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vshlq_u8)
pub fn vshlq_u8(a: uint8x16_t, b: int8x16_t) -> uint8x16_t {
    { _vshlq_u8(a, b) }
}
///Unsigned Shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vshl_u16)
pub fn vshl_u16(a: uint16x4_t, b: int16x4_t) -> uint16x4_t {
    { _vshl_u16(a, b) }
}
///Unsigned Shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vshlq_u16)
pub fn vshlq_u16(a: uint16x8_t, b: int16x8_t) -> uint16x8_t {
    { _vshlq_u16(a, b) }
}
///Unsigned Shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vshl_u32)
pub fn vshl_u32(a: uint32x2_t, b: int32x2_t) -> uint32x2_t {
    { _vshl_u32(a, b) }
}
///Unsigned Shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vshlq_u32)
pub fn vshlq_u32(a: uint32x4_t, b: int32x4_t) -> uint32x4_t {
    { _vshlq_u32(a, b) }
}
///Unsigned Shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vshl_u64)
pub fn vshl_u64(a: uint64x1_t, b: int64x1_t) -> uint64x1_t {
    { _vshl_u64(a, b) }
}
///Unsigned Shift left
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vshlq_u64)
pub fn vshlq_u64(a: uint64x2_t, b: int64x2_t) -> uint64x2_t {
    { _vshlq_u64(a, b) }
}
///Signed shift left long
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vshll_n_s16)
pub fn vshll_n_s16<const N: i32>(a: int16x4_t) -> int32x4_t {
    static_assert!(N >= 0 && N <= 16);
    { simd_shl(simd_cast(a), vdupq_n_s32(N as _)) }
}
///Signed shift left long
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vshll_n_s32)
pub fn vshll_n_s32<const N: i32>(a: int32x2_t) -> int64x2_t {
    static_assert!(N >= 0 && N <= 32);
    { simd_shl(simd_cast(a), vdupq_n_s64(N as _)) }
}
///Signed shift left long
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vshll_n_s8)
pub fn vshll_n_s8<const N: i32>(a: int8x8_t) -> int16x8_t {
    static_assert!(N >= 0 && N <= 8);
    { simd_shl(simd_cast(a), vdupq_n_s16(N as _)) }
}
///Signed shift left long
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vshll_n_u16)
pub fn vshll_n_u16<const N: i32>(a: uint16x4_t) -> uint32x4_t {
    static_assert!(N >= 0 && N <= 16);
    { simd_shl(simd_cast(a), vdupq_n_u32(N as _)) }
}
///Signed shift left long
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vshll_n_u32)
pub fn vshll_n_u32<const N: i32>(a: uint32x2_t) -> uint64x2_t {
    static_assert!(N >= 0 && N <= 32);
    { simd_shl(simd_cast(a), vdupq_n_u64(N as _)) }
}
///Signed shift left long
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vshll_n_u8)
pub fn vshll_n_u8<const N: i32>(a: uint8x8_t) -> uint16x8_t {
    static_assert!(N >= 0 && N <= 8);
    { simd_shl(simd_cast(a), vdupq_n_u16(N as _)) }
}
///Shift right
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vshr_n_s8)
pub fn vshr_n_s8<const N: i32>(a: int8x8_t) -> int8x8_t {
    static_assert!(N >= 1 && N <= 8);
    let n: i32 = if N == 8 { 7 } else { N };
    { simd_shr(a, vdup_n_s8(n as _)) }
}
///Shift right
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vshrq_n_s8)
pub fn vshrq_n_s8<const N: i32>(a: int8x16_t) -> int8x16_t {
    static_assert!(N >= 1 && N <= 8);
    let n: i32 = if N == 8 { 7 } else { N };
    { simd_shr(a, vdupq_n_s8(n as _)) }
}
///Shift right
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vshr_n_s16)
pub fn vshr_n_s16<const N: i32>(a: int16x4_t) -> int16x4_t {
    static_assert!(N >= 1 && N <= 16);
    let n: i32 = if N == 16 { 15 } else { N };
    { simd_shr(a, vdup_n_s16(n as _)) }
}
///Shift right
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vshrq_n_s16)
pub fn vshrq_n_s16<const N: i32>(a: int16x8_t) -> int16x8_t {
    static_assert!(N >= 1 && N <= 16);
    let n: i32 = if N == 16 { 15 } else { N };
    { simd_shr(a, vdupq_n_s16(n as _)) }
}
///Shift right
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vshr_n_s32)
pub fn vshr_n_s32<const N: i32>(a: int32x2_t) -> int32x2_t {
    static_assert!(N >= 1 && N <= 32);
    let n: i32 = if N == 32 { 31 } else { N };
    { simd_shr(a, vdup_n_s32(n as _)) }
}
///Shift right
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vshrq_n_s32)
pub fn vshrq_n_s32<const N: i32>(a: int32x4_t) -> int32x4_t {
    static_assert!(N >= 1 && N <= 32);
    let n: i32 = if N == 32 { 31 } else { N };
    { simd_shr(a, vdupq_n_s32(n as _)) }
}
///Shift right
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vshr_n_s64)
pub fn vshr_n_s64<const N: i32>(a: int64x1_t) -> int64x1_t {
    static_assert!(N >= 1 && N <= 64);
    let n: i32 = if N == 64 { 63 } else { N };
    { simd_shr(a, vdup_n_s64(n as _)) }
}
///Shift right
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vshrq_n_s64)
pub fn vshrq_n_s64<const N: i32>(a: int64x2_t) -> int64x2_t {
    static_assert!(N >= 1 && N <= 64);
    let n: i32 = if N == 64 { 63 } else { N };
    { simd_shr(a, vdupq_n_s64(n as _)) }
}
///Shift right
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vshr_n_u8)
pub fn vshr_n_u8<const N: i32>(a: uint8x8_t) -> uint8x8_t {
    static_assert!(N >= 1 && N <= 8);
    let n: i32 = if N == 8 {
        return vdup_n_u8(0);
    } else {
        N
    };
    { simd_shr(a, vdup_n_u8(n as _)) }
}
///Shift right
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vshrq_n_u8)
pub fn vshrq_n_u8<const N: i32>(a: uint8x16_t) -> uint8x16_t {
    static_assert!(N >= 1 && N <= 8);
    let n: i32 = if N == 8 {
        return vdupq_n_u8(0);
    } else {
        N
    };
    { simd_shr(a, vdupq_n_u8(n as _)) }
}
///Shift right
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vshr_n_u16)
pub fn vshr_n_u16<const N: i32>(a: uint16x4_t) -> uint16x4_t {
    static_assert!(N >= 1 && N <= 16);
    let n: i32 = if N == 16 {
        return vdup_n_u16(0);
    } else {
        N
    };
    { simd_shr(a, vdup_n_u16(n as _)) }
}
///Shift right
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vshrq_n_u16)
pub fn vshrq_n_u16<const N: i32>(a: uint16x8_t) -> uint16x8_t {
    static_assert!(N >= 1 && N <= 16);
    let n: i32 = if N == 16 {
        return vdupq_n_u16(0);
    } else {
        N
    };
    { simd_shr(a, vdupq_n_u16(n as _)) }
}
///Shift right
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vshr_n_u32)
pub fn vshr_n_u32<const N: i32>(a: uint32x2_t) -> uint32x2_t {
    static_assert!(N >= 1 && N <= 32);
    let n: i32 = if N == 32 {
        return vdup_n_u32(0);
    } else {
        N
    };
    { simd_shr(a, vdup_n_u32(n as _)) }
}
///Shift right
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vshrq_n_u32)
pub fn vshrq_n_u32<const N: i32>(a: uint32x4_t) -> uint32x4_t {
    static_assert!(N >= 1 && N <= 32);
    let n: i32 = if N == 32 {
        return vdupq_n_u32(0);
    } else {
        N
    };
    { simd_shr(a, vdupq_n_u32(n as _)) }
}
///Shift right
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vshr_n_u64)
pub fn vshr_n_u64<const N: i32>(a: uint64x1_t) -> uint64x1_t {
    static_assert!(N >= 1 && N <= 64);
    let n: i32 = if N == 64 {
        return vdup_n_u64(0);
    } else {
        N
    };
    { simd_shr(a, vdup_n_u64(n as _)) }
}
///Shift right
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vshrq_n_u64)
pub fn vshrq_n_u64<const N: i32>(a: uint64x2_t) -> uint64x2_t {
    static_assert!(N >= 1 && N <= 64);
    let n: i32 = if N == 64 {
        return vdupq_n_u64(0);
    } else {
        N
    };
    { simd_shr(a, vdupq_n_u64(n as _)) }
}
///Shift right narrow
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vshrn_n_s16)
pub fn vshrn_n_s16<const N: i32>(a: int16x8_t) -> int8x8_t {
    static_assert!(N >= 1 && N <= 8);
    { simd_cast(simd_shr(a, vdupq_n_s16(N as _))) }
}
///Shift right narrow
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vshrn_n_s32)
pub fn vshrn_n_s32<const N: i32>(a: int32x4_t) -> int16x4_t {
    static_assert!(N >= 1 && N <= 16);
    { simd_cast(simd_shr(a, vdupq_n_s32(N as _))) }
}
///Shift right narrow
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vshrn_n_s64)
pub fn vshrn_n_s64<const N: i32>(a: int64x2_t) -> int32x2_t {
    static_assert!(N >= 1 && N <= 32);
    { simd_cast(simd_shr(a, vdupq_n_s64(N as _))) }
}
///Shift right narrow
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vshrn_n_u16)
pub fn vshrn_n_u16<const N: i32>(a: uint16x8_t) -> uint8x8_t {
    static_assert!(N >= 1 && N <= 8);
    { simd_cast(simd_shr(a, vdupq_n_u16(N as _))) }
}
///Shift right narrow
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vshrn_n_u32)
pub fn vshrn_n_u32<const N: i32>(a: uint32x4_t) -> uint16x4_t {
    static_assert!(N >= 1 && N <= 16);
    { simd_cast(simd_shr(a, vdupq_n_u32(N as _))) }
}
///Shift right narrow
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vshrn_n_u64)
pub fn vshrn_n_u64<const N: i32>(a: uint64x2_t) -> uint32x2_t {
    static_assert!(N >= 1 && N <= 32);
    { simd_cast(simd_shr(a, vdupq_n_u64(N as _))) }
}
///Shift Left and Insert (immediate)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsli_n_s8)
pub fn vsli_n_s8<const N: i32>(a: int8x8_t, b: int8x8_t) -> int8x8_t {
    static_assert_uimm_bits!(N, 3);
    vshiftins_v8i8(a, b, int8x8_t::splat(N as i8))
}
///Shift Left and Insert (immediate)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsliq_n_s8)
pub fn vsliq_n_s8<const N: i32>(a: int8x16_t, b: int8x16_t) -> int8x16_t {
    static_assert_uimm_bits!(N, 3);
    vshiftins_v16i8(a, b, int8x16_t::splat(N as i8))
}
///Shift Left and Insert (immediate)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsli_n_s16)
pub fn vsli_n_s16<const N: i32>(a: int16x4_t, b: int16x4_t) -> int16x4_t {
    static_assert_uimm_bits!(N, 4);
    vshiftins_v4i16(a, b, int16x4_t::splat(N as i16))
}
///Shift Left and Insert (immediate)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsliq_n_s16)
pub fn vsliq_n_s16<const N: i32>(a: int16x8_t, b: int16x8_t) -> int16x8_t {
    static_assert_uimm_bits!(N, 4);
    vshiftins_v8i16(a, b, int16x8_t::splat(N as i16))
}
///Shift Left and Insert (immediate)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsli_n_s32)
pub fn vsli_n_s32<const N: i32>(a: int32x2_t, b: int32x2_t) -> int32x2_t {
    static_assert!(N >= 0 && N <= 31);
    vshiftins_v2i32(a, b, int32x2_t::splat(N))
}
///Shift Left and Insert (immediate)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsliq_n_s32)
pub fn vsliq_n_s32<const N: i32>(a: int32x4_t, b: int32x4_t) -> int32x4_t {
    static_assert!(N >= 0 && N <= 31);
    vshiftins_v4i32(a, b, int32x4_t::splat(N))
}
///Shift Left and Insert (immediate)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsli_n_s64)
pub fn vsli_n_s64<const N: i32>(a: int64x1_t, b: int64x1_t) -> int64x1_t {
    static_assert!(N >= 0 && N <= 63);
    vshiftins_v1i64(a, b, int64x1_t::splat(N as i64))
}
///Shift Left and Insert (immediate)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsliq_n_s64)
pub fn vsliq_n_s64<const N: i32>(a: int64x2_t, b: int64x2_t) -> int64x2_t {
    static_assert!(N >= 0 && N <= 63);
    vshiftins_v2i64(a, b, int64x2_t::splat(N as i64))
}
///Shift Left and Insert (immediate)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsli_n_u8)
pub fn vsli_n_u8<const N: i32>(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
    static_assert_uimm_bits!(N, 3);
    { transmute(vshiftins_v8i8(transmute(a), transmute(b), int8x8_t::splat(N as i8))) }
}
///Shift Left and Insert (immediate)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsliq_n_u8)
pub fn vsliq_n_u8<const N: i32>(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
    static_assert_uimm_bits!(N, 3);
    { transmute(vshiftins_v16i8(transmute(a), transmute(b), int8x16_t::splat(N as i8))) }
}
///Shift Left and Insert (immediate)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsli_n_u16)
pub fn vsli_n_u16<const N: i32>(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
    static_assert_uimm_bits!(N, 4);
    {
        transmute(
            vshiftins_v4i16(transmute(a), transmute(b), int16x4_t::splat(N as i16)),
        )
    }
}
///Shift Left and Insert (immediate)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsliq_n_u16)
pub fn vsliq_n_u16<const N: i32>(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
    static_assert_uimm_bits!(N, 4);
    {
        transmute(
            vshiftins_v8i16(transmute(a), transmute(b), int16x8_t::splat(N as i16)),
        )
    }
}
///Shift Left and Insert (immediate)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsli_n_u32)
pub fn vsli_n_u32<const N: i32>(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
    static_assert!(N >= 0 && N <= 31);
    {
        transmute(
            vshiftins_v2i32(transmute(a), transmute(b), int32x2_t::splat(N as i32)),
        )
    }
}
///Shift Left and Insert (immediate)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsliq_n_u32)
pub fn vsliq_n_u32<const N: i32>(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
    static_assert!(N >= 0 && N <= 31);
    {
        transmute(
            vshiftins_v4i32(transmute(a), transmute(b), int32x4_t::splat(N as i32)),
        )
    }
}
///Shift Left and Insert (immediate)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsli_n_u64)
pub fn vsli_n_u64<const N: i32>(a: uint64x1_t, b: uint64x1_t) -> uint64x1_t {
    static_assert!(N >= 0 && N <= 63);
    {
        transmute(
            vshiftins_v1i64(transmute(a), transmute(b), int64x1_t::splat(N as i64)),
        )
    }
}
///Shift Left and Insert (immediate)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsliq_n_u64)
pub fn vsliq_n_u64<const N: i32>(a: uint64x2_t, b: uint64x2_t) -> uint64x2_t {
    static_assert!(N >= 0 && N <= 63);
    {
        transmute(
            vshiftins_v2i64(transmute(a), transmute(b), int64x2_t::splat(N as i64)),
        )
    }
}
///Shift Left and Insert (immediate)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsli_n_p8)
pub fn vsli_n_p8<const N: i32>(a: poly8x8_t, b: poly8x8_t) -> poly8x8_t {
    static_assert_uimm_bits!(N, 3);
    { transmute(vshiftins_v8i8(transmute(a), transmute(b), int8x8_t::splat(N as i8))) }
}
///Shift Left and Insert (immediate)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsliq_n_p8)
pub fn vsliq_n_p8<const N: i32>(a: poly8x16_t, b: poly8x16_t) -> poly8x16_t {
    static_assert_uimm_bits!(N, 3);
    { transmute(vshiftins_v16i8(transmute(a), transmute(b), int8x16_t::splat(N as i8))) }
}
///Shift Left and Insert (immediate)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsli_n_p16)
pub fn vsli_n_p16<const N: i32>(a: poly16x4_t, b: poly16x4_t) -> poly16x4_t {
    static_assert_uimm_bits!(N, 4);
    {
        transmute(
            vshiftins_v4i16(transmute(a), transmute(b), int16x4_t::splat(N as i16)),
        )
    }
}
///Shift Left and Insert (immediate)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsliq_n_p16)
pub fn vsliq_n_p16<const N: i32>(a: poly16x8_t, b: poly16x8_t) -> poly16x8_t {
    static_assert_uimm_bits!(N, 4);
    {
        transmute(
            vshiftins_v8i16(transmute(a), transmute(b), int16x8_t::splat(N as i16)),
        )
    }
}
///Signed shift right and accumulate
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsra_n_s8)
pub fn vsra_n_s8<const N: i32>(a: int8x8_t, b: int8x8_t) -> int8x8_t {
    static_assert!(N >= 1 && N <= 8);
    { simd_add(a, vshr_n_s8::<N>(b)) }
}
///Signed shift right and accumulate
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsraq_n_s8)
pub fn vsraq_n_s8<const N: i32>(a: int8x16_t, b: int8x16_t) -> int8x16_t {
    static_assert!(N >= 1 && N <= 8);
    { simd_add(a, vshrq_n_s8::<N>(b)) }
}
///Signed shift right and accumulate
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsra_n_s16)
pub fn vsra_n_s16<const N: i32>(a: int16x4_t, b: int16x4_t) -> int16x4_t {
    static_assert!(N >= 1 && N <= 16);
    { simd_add(a, vshr_n_s16::<N>(b)) }
}
///Signed shift right and accumulate
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsraq_n_s16)
pub fn vsraq_n_s16<const N: i32>(a: int16x8_t, b: int16x8_t) -> int16x8_t {
    static_assert!(N >= 1 && N <= 16);
    { simd_add(a, vshrq_n_s16::<N>(b)) }
}
///Signed shift right and accumulate
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsra_n_s32)
pub fn vsra_n_s32<const N: i32>(a: int32x2_t, b: int32x2_t) -> int32x2_t {
    static_assert!(N >= 1 && N <= 32);
    { simd_add(a, vshr_n_s32::<N>(b)) }
}
///Signed shift right and accumulate
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsraq_n_s32)
pub fn vsraq_n_s32<const N: i32>(a: int32x4_t, b: int32x4_t) -> int32x4_t {
    static_assert!(N >= 1 && N <= 32);
    { simd_add(a, vshrq_n_s32::<N>(b)) }
}
///Signed shift right and accumulate
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsra_n_s64)
pub fn vsra_n_s64<const N: i32>(a: int64x1_t, b: int64x1_t) -> int64x1_t {
    static_assert!(N >= 1 && N <= 64);
    { simd_add(a, vshr_n_s64::<N>(b)) }
}
///Signed shift right and accumulate
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsraq_n_s64)
pub fn vsraq_n_s64<const N: i32>(a: int64x2_t, b: int64x2_t) -> int64x2_t {
    static_assert!(N >= 1 && N <= 64);
    { simd_add(a, vshrq_n_s64::<N>(b)) }
}
///Unsigned shift right and accumulate
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsra_n_u8)
pub fn vsra_n_u8<const N: i32>(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
    static_assert!(N >= 1 && N <= 8);
    { simd_add(a, vshr_n_u8::<N>(b)) }
}
///Unsigned shift right and accumulate
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsraq_n_u8)
pub fn vsraq_n_u8<const N: i32>(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
    static_assert!(N >= 1 && N <= 8);
    { simd_add(a, vshrq_n_u8::<N>(b)) }
}
///Unsigned shift right and accumulate
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsra_n_u16)
pub fn vsra_n_u16<const N: i32>(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
    static_assert!(N >= 1 && N <= 16);
    { simd_add(a, vshr_n_u16::<N>(b)) }
}
///Unsigned shift right and accumulate
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsraq_n_u16)
pub fn vsraq_n_u16<const N: i32>(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
    static_assert!(N >= 1 && N <= 16);
    { simd_add(a, vshrq_n_u16::<N>(b)) }
}
///Unsigned shift right and accumulate
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsra_n_u32)
pub fn vsra_n_u32<const N: i32>(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
    static_assert!(N >= 1 && N <= 32);
    { simd_add(a, vshr_n_u32::<N>(b)) }
}
///Unsigned shift right and accumulate
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsraq_n_u32)
pub fn vsraq_n_u32<const N: i32>(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
    static_assert!(N >= 1 && N <= 32);
    { simd_add(a, vshrq_n_u32::<N>(b)) }
}
///Unsigned shift right and accumulate
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsra_n_u64)
pub fn vsra_n_u64<const N: i32>(a: uint64x1_t, b: uint64x1_t) -> uint64x1_t {
    static_assert!(N >= 1 && N <= 64);
    { simd_add(a, vshr_n_u64::<N>(b)) }
}
///Unsigned shift right and accumulate
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsraq_n_u64)
pub fn vsraq_n_u64<const N: i32>(a: uint64x2_t, b: uint64x2_t) -> uint64x2_t {
    static_assert!(N >= 1 && N <= 64);
    { simd_add(a, vshrq_n_u64::<N>(b)) }
}
///Shift Right and Insert (immediate)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsri_n_s8)
pub fn vsri_n_s8<const N: i32>(a: int8x8_t, b: int8x8_t) -> int8x8_t {
    static_assert!(1 <= N && N <= 8);
    vshiftins_v8i8(a, b, int8x8_t::splat(-N as i8))
}
///Shift Right and Insert (immediate)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsriq_n_s8)
pub fn vsriq_n_s8<const N: i32>(a: int8x16_t, b: int8x16_t) -> int8x16_t {
    static_assert!(1 <= N && N <= 8);
    vshiftins_v16i8(a, b, int8x16_t::splat(-N as i8))
}
///Shift Right and Insert (immediate)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsri_n_s16)
pub fn vsri_n_s16<const N: i32>(a: int16x4_t, b: int16x4_t) -> int16x4_t {
    static_assert!(1 <= N && N <= 16);
    vshiftins_v4i16(a, b, int16x4_t::splat(-N as i16))
}
///Shift Right and Insert (immediate)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsriq_n_s16)
pub fn vsriq_n_s16<const N: i32>(a: int16x8_t, b: int16x8_t) -> int16x8_t {
    static_assert!(1 <= N && N <= 16);
    vshiftins_v8i16(a, b, int16x8_t::splat(-N as i16))
}
///Shift Right and Insert (immediate)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsri_n_s32)
pub fn vsri_n_s32<const N: i32>(a: int32x2_t, b: int32x2_t) -> int32x2_t {
    static_assert!(1 <= N && N <= 32);
    vshiftins_v2i32(a, b, int32x2_t::splat(-N as i32))
}
///Shift Right and Insert (immediate)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsriq_n_s32)
pub fn vsriq_n_s32<const N: i32>(a: int32x4_t, b: int32x4_t) -> int32x4_t {
    static_assert!(1 <= N && N <= 32);
    vshiftins_v4i32(a, b, int32x4_t::splat(-N as i32))
}
///Shift Right and Insert (immediate)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsri_n_s64)
pub fn vsri_n_s64<const N: i32>(a: int64x1_t, b: int64x1_t) -> int64x1_t {
    static_assert!(1 <= N && N <= 64);
    vshiftins_v1i64(a, b, int64x1_t::splat(-N as i64))
}
///Shift Right and Insert (immediate)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsriq_n_s64)
pub fn vsriq_n_s64<const N: i32>(a: int64x2_t, b: int64x2_t) -> int64x2_t {
    static_assert!(1 <= N && N <= 64);
    vshiftins_v2i64(a, b, int64x2_t::splat(-N as i64))
}
///Shift Right and Insert (immediate)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsri_n_u8)
pub fn vsri_n_u8<const N: i32>(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
    static_assert!(1 <= N && N <= 8);
    { transmute(vshiftins_v8i8(transmute(a), transmute(b), int8x8_t::splat(-N as i8))) }
}
///Shift Right and Insert (immediate)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsriq_n_u8)
pub fn vsriq_n_u8<const N: i32>(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
    static_assert!(1 <= N && N <= 8);
    {
        transmute(
            vshiftins_v16i8(transmute(a), transmute(b), int8x16_t::splat(-N as i8)),
        )
    }
}
///Shift Right and Insert (immediate)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsri_n_u16)
pub fn vsri_n_u16<const N: i32>(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
    static_assert!(1 <= N && N <= 16);
    {
        transmute(
            vshiftins_v4i16(transmute(a), transmute(b), int16x4_t::splat(-N as i16)),
        )
    }
}
///Shift Right and Insert (immediate)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsriq_n_u16)
pub fn vsriq_n_u16<const N: i32>(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
    static_assert!(1 <= N && N <= 16);
    {
        transmute(
            vshiftins_v8i16(transmute(a), transmute(b), int16x8_t::splat(-N as i16)),
        )
    }
}
///Shift Right and Insert (immediate)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsri_n_u32)
pub fn vsri_n_u32<const N: i32>(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
    static_assert!(1 <= N && N <= 32);
    { transmute(vshiftins_v2i32(transmute(a), transmute(b), int32x2_t::splat(-N))) }
}
///Shift Right and Insert (immediate)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsriq_n_u32)
pub fn vsriq_n_u32<const N: i32>(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
    static_assert!(1 <= N && N <= 32);
    { transmute(vshiftins_v4i32(transmute(a), transmute(b), int32x4_t::splat(-N))) }
}
///Shift Right and Insert (immediate)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsri_n_u64)
pub fn vsri_n_u64<const N: i32>(a: uint64x1_t, b: uint64x1_t) -> uint64x1_t {
    static_assert!(1 <= N && N <= 64);
    {
        transmute(
            vshiftins_v1i64(transmute(a), transmute(b), int64x1_t::splat(-N as i64)),
        )
    }
}
///Shift Right and Insert (immediate)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsriq_n_u64)
pub fn vsriq_n_u64<const N: i32>(a: uint64x2_t, b: uint64x2_t) -> uint64x2_t {
    static_assert!(1 <= N && N <= 64);
    {
        transmute(
            vshiftins_v2i64(transmute(a), transmute(b), int64x2_t::splat(-N as i64)),
        )
    }
}
///Shift Right and Insert (immediate)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsri_n_p8)
pub fn vsri_n_p8<const N: i32>(a: poly8x8_t, b: poly8x8_t) -> poly8x8_t {
    static_assert!(1 <= N && N <= 8);
    { transmute(vshiftins_v8i8(transmute(a), transmute(b), int8x8_t::splat(-N as i8))) }
}
///Shift Right and Insert (immediate)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsriq_n_p8)
pub fn vsriq_n_p8<const N: i32>(a: poly8x16_t, b: poly8x16_t) -> poly8x16_t {
    static_assert!(1 <= N && N <= 8);
    {
        transmute(
            vshiftins_v16i8(transmute(a), transmute(b), int8x16_t::splat(-N as i8)),
        )
    }
}
///Shift Right and Insert (immediate)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsri_n_p16)
pub fn vsri_n_p16<const N: i32>(a: poly16x4_t, b: poly16x4_t) -> poly16x4_t {
    static_assert!(1 <= N && N <= 16);
    {
        transmute(
            vshiftins_v4i16(transmute(a), transmute(b), int16x4_t::splat(-N as i16)),
        )
    }
}
///Shift Right and Insert (immediate)
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsriq_n_p16)
pub fn vsriq_n_p16<const N: i32>(a: poly16x8_t, b: poly16x8_t) -> poly16x8_t {
    static_assert!(1 <= N && N <= 16);
    {
        transmute(
            vshiftins_v8i16(transmute(a), transmute(b), int16x8_t::splat(-N as i16)),
        )
    }
}
///Subtract
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsub_f16)
pub fn vsub_f16(a: float16x4_t, b: float16x4_t) -> float16x4_t {
    { simd_sub(a, b) }
}
///Subtract
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsubq_f16)
pub fn vsubq_f16(a: float16x8_t, b: float16x8_t) -> float16x8_t {
    { simd_sub(a, b) }
}
///Subtract
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsub_f32)
pub fn vsub_f32(a: float32x2_t, b: float32x2_t) -> float32x2_t {
    { simd_sub(a, b) }
}
///Subtract
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsubq_f32)
pub fn vsubq_f32(a: float32x4_t, b: float32x4_t) -> float32x4_t {
    { simd_sub(a, b) }
}
///Subtract
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsub_s16)
pub fn vsub_s16(a: int16x4_t, b: int16x4_t) -> int16x4_t {
    { simd_sub(a, b) }
}
///Subtract
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsubq_s16)
pub fn vsubq_s16(a: int16x8_t, b: int16x8_t) -> int16x8_t {
    { simd_sub(a, b) }
}
///Subtract
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsub_u16)
pub fn vsub_u16(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
    { simd_sub(a, b) }
}
///Subtract
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsubq_u16)
pub fn vsubq_u16(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
    { simd_sub(a, b) }
}
///Subtract
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsub_s32)
pub fn vsub_s32(a: int32x2_t, b: int32x2_t) -> int32x2_t {
    { simd_sub(a, b) }
}
///Subtract
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsubq_s32)
pub fn vsubq_s32(a: int32x4_t, b: int32x4_t) -> int32x4_t {
    { simd_sub(a, b) }
}
///Subtract
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsub_u32)
pub fn vsub_u32(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
    { simd_sub(a, b) }
}
///Subtract
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsubq_u32)
pub fn vsubq_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
    { simd_sub(a, b) }
}
///Subtract
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsub_s64)
pub fn vsub_s64(a: int64x1_t, b: int64x1_t) -> int64x1_t {
    { simd_sub(a, b) }
}
///Subtract
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsubq_s64)
pub fn vsubq_s64(a: int64x2_t, b: int64x2_t) -> int64x2_t {
    { simd_sub(a, b) }
}
///Subtract
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsub_u64)
pub fn vsub_u64(a: uint64x1_t, b: uint64x1_t) -> uint64x1_t {
    { simd_sub(a, b) }
}
///Subtract
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsubq_u64)
pub fn vsubq_u64(a: uint64x2_t, b: uint64x2_t) -> uint64x2_t {
    { simd_sub(a, b) }
}
///Subtract
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsub_s8)
pub fn vsub_s8(a: int8x8_t, b: int8x8_t) -> int8x8_t {
    { simd_sub(a, b) }
}
///Subtract
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsubq_s8)
pub fn vsubq_s8(a: int8x16_t, b: int8x16_t) -> int8x16_t {
    { simd_sub(a, b) }
}
///Subtract
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsub_u8)
pub fn vsub_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
    { simd_sub(a, b) }
}
///Subtract
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsubq_u8)
pub fn vsubq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
    { simd_sub(a, b) }
}
///Subtract returning high narrow
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsubhn_high_s16)
pub fn vsubhn_high_s16(a: int8x8_t, b: int16x8_t, c: int16x8_t) -> int8x16_t {
    let d: int8x8_t = vsubhn_s16(b, c);
    { simd_shuffle(a, d, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]) }
}
///Subtract returning high narrow
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsubhn_high_s32)
pub fn vsubhn_high_s32(a: int16x4_t, b: int32x4_t, c: int32x4_t) -> int16x8_t {
    let d: int16x4_t = vsubhn_s32(b, c);
    { simd_shuffle(a, d, [0, 1, 2, 3, 4, 5, 6, 7]) }
}
///Subtract returning high narrow
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsubhn_high_s64)
pub fn vsubhn_high_s64(a: int32x2_t, b: int64x2_t, c: int64x2_t) -> int32x4_t {
    let d: int32x2_t = vsubhn_s64(b, c);
    { simd_shuffle(a, d, [0, 1, 2, 3]) }
}
///Subtract returning high narrow
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsubhn_high_u16)
pub fn vsubhn_high_u16(a: uint8x8_t, b: uint16x8_t, c: uint16x8_t) -> uint8x16_t {
    let d: uint8x8_t = vsubhn_u16(b, c);
    { simd_shuffle(a, d, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]) }
}
///Subtract returning high narrow
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsubhn_high_u32)
pub fn vsubhn_high_u32(a: uint16x4_t, b: uint32x4_t, c: uint32x4_t) -> uint16x8_t {
    let d: uint16x4_t = vsubhn_u32(b, c);
    { simd_shuffle(a, d, [0, 1, 2, 3, 4, 5, 6, 7]) }
}
///Subtract returning high narrow
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsubhn_high_u64)
pub fn vsubhn_high_u64(a: uint32x2_t, b: uint64x2_t, c: uint64x2_t) -> uint32x4_t {
    let d: uint32x2_t = vsubhn_u64(b, c);
    { simd_shuffle(a, d, [0, 1, 2, 3]) }
}
///Subtract returning high narrow
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsubhn_s16)
pub fn vsubhn_s16(a: int16x8_t, b: int16x8_t) -> int8x8_t {
    let c: i16x8 = i16x8::new(8, 8, 8, 8, 8, 8, 8, 8);
    { simd_cast(simd_shr(simd_sub(a, b), transmute(c))) }
}
///Subtract returning high narrow
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsubhn_s32)
pub fn vsubhn_s32(a: int32x4_t, b: int32x4_t) -> int16x4_t {
    let c: i32x4 = i32x4::new(16, 16, 16, 16);
    { simd_cast(simd_shr(simd_sub(a, b), transmute(c))) }
}
///Subtract returning high narrow
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsubhn_s64)
pub fn vsubhn_s64(a: int64x2_t, b: int64x2_t) -> int32x2_t {
    let c: i64x2 = i64x2::new(32, 32);
    { simd_cast(simd_shr(simd_sub(a, b), transmute(c))) }
}
///Subtract returning high narrow
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsubhn_u16)
pub fn vsubhn_u16(a: uint16x8_t, b: uint16x8_t) -> uint8x8_t {
    let c: u16x8 = u16x8::new(8, 8, 8, 8, 8, 8, 8, 8);
    { simd_cast(simd_shr(simd_sub(a, b), transmute(c))) }
}
///Subtract returning high narrow
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsubhn_u32)
pub fn vsubhn_u32(a: uint32x4_t, b: uint32x4_t) -> uint16x4_t {
    let c: u32x4 = u32x4::new(16, 16, 16, 16);
    { simd_cast(simd_shr(simd_sub(a, b), transmute(c))) }
}
///Subtract returning high narrow
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsubhn_u64)
pub fn vsubhn_u64(a: uint64x2_t, b: uint64x2_t) -> uint32x2_t {
    let c: u64x2 = u64x2::new(32, 32);
    { simd_cast(simd_shr(simd_sub(a, b), transmute(c))) }
}
///Signed Subtract Long
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsubl_s8)
pub fn vsubl_s8(a: int8x8_t, b: int8x8_t) -> int16x8_t {
    {
        let c: int16x8_t = simd_cast(a);
        let d: int16x8_t = simd_cast(b);
        simd_sub(c, d)
    }
}
///Signed Subtract Long
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsubl_s16)
pub fn vsubl_s16(a: int16x4_t, b: int16x4_t) -> int32x4_t {
    {
        let c: int32x4_t = simd_cast(a);
        let d: int32x4_t = simd_cast(b);
        simd_sub(c, d)
    }
}
///Signed Subtract Long
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsubl_s32)
pub fn vsubl_s32(a: int32x2_t, b: int32x2_t) -> int64x2_t {
    {
        let c: int64x2_t = simd_cast(a);
        let d: int64x2_t = simd_cast(b);
        simd_sub(c, d)
    }
}
///Unsigned Subtract Long
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsubl_u8)
pub fn vsubl_u8(a: uint8x8_t, b: uint8x8_t) -> uint16x8_t {
    {
        let c: uint16x8_t = simd_cast(a);
        let d: uint16x8_t = simd_cast(b);
        simd_sub(c, d)
    }
}
///Unsigned Subtract Long
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsubl_u16)
pub fn vsubl_u16(a: uint16x4_t, b: uint16x4_t) -> uint32x4_t {
    {
        let c: uint32x4_t = simd_cast(a);
        let d: uint32x4_t = simd_cast(b);
        simd_sub(c, d)
    }
}
///Unsigned Subtract Long
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsubl_u32)
pub fn vsubl_u32(a: uint32x2_t, b: uint32x2_t) -> uint64x2_t {
    {
        let c: uint64x2_t = simd_cast(a);
        let d: uint64x2_t = simd_cast(b);
        simd_sub(c, d)
    }
}
///Signed Subtract Wide
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsubw_s8)
pub fn vsubw_s8(a: int16x8_t, b: int8x8_t) -> int16x8_t {
    { simd_sub(a, simd_cast(b)) }
}
///Signed Subtract Wide
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsubw_s16)
pub fn vsubw_s16(a: int32x4_t, b: int16x4_t) -> int32x4_t {
    { simd_sub(a, simd_cast(b)) }
}
///Signed Subtract Wide
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsubw_s32)
pub fn vsubw_s32(a: int64x2_t, b: int32x2_t) -> int64x2_t {
    { simd_sub(a, simd_cast(b)) }
}
///Unsigned Subtract Wide
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsubw_u8)
pub fn vsubw_u8(a: uint16x8_t, b: uint8x8_t) -> uint16x8_t {
    { simd_sub(a, simd_cast(b)) }
}
///Unsigned Subtract Wide
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsubw_u16)
pub fn vsubw_u16(a: uint32x4_t, b: uint16x4_t) -> uint32x4_t {
    { simd_sub(a, simd_cast(b)) }
}
///Unsigned Subtract Wide
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsubw_u32)
pub fn vsubw_u32(a: uint64x2_t, b: uint32x2_t) -> uint64x2_t {
    { simd_sub(a, simd_cast(b)) }
}
///Dot product index form with signed and unsigned integers
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsudot_lane_s32)
pub fn vsudot_lane_s32<const LANE: i32>(
    a: int32x2_t,
    b: int8x8_t,
    c: uint8x8_t,
) -> int32x2_t {
    static_assert_uimm_bits!(LANE, 1);
    {
        let c: uint32x2_t = transmute(c);
        let c: uint32x2_t = simd_shuffle(c, c, [LANE as u32, LANE as u32]);
        vusdot_s32(a, transmute(c), b)
    }
}
///Dot product index form with signed and unsigned integers
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsudotq_lane_s32)
pub fn vsudotq_lane_s32<const LANE: i32>(
    a: int32x4_t,
    b: int8x16_t,
    c: uint8x8_t,
) -> int32x4_t {
    static_assert_uimm_bits!(LANE, 1);
    {
        let c: uint32x2_t = transmute(c);
        let c: uint32x4_t = simd_shuffle(
            c, c, [LANE as u32, LANE as u32, LANE as u32, LANE as u32]
        );
        vusdotq_s32(a, transmute(c), b)
    }
}
///Table look-up
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vtbl1)
fn vtbl1(a: int8x8_t, b: int8x8_t) -> int8x8_t {
    { _vtbl1(a, b) }
}
///Table look-up
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vtbl1_s8)
pub fn vtbl1_s8(a: int8x8_t, b: int8x8_t) -> int8x8_t {
    vtbl1(a, b)
}
///Table look-up
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vtbl1_u8)
pub fn vtbl1_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
    { transmute(vtbl1(transmute(a), transmute(b))) }
}
///Table look-up
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vtbl1_u8)
pub fn vtbl1_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
    let a: uint8x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    let b: uint8x8_t = unsafe { simd_shuffle(b, b, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: uint8x8_t = transmute(vtbl1(transmute(a), transmute(b)));
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Table look-up
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vtbl1_p8)
pub fn vtbl1_p8(a: poly8x8_t, b: uint8x8_t) -> poly8x8_t {
    { transmute(vtbl1(transmute(a), transmute(b))) }
}
///Table look-up
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vtbl1_p8)
pub fn vtbl1_p8(a: poly8x8_t, b: uint8x8_t) -> poly8x8_t {
    let a: poly8x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    let b: uint8x8_t = unsafe { simd_shuffle(b, b, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: poly8x8_t = transmute(vtbl1(transmute(a), transmute(b)));
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Table look-up
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vtbl2)
fn vtbl2(a: int8x8_t, b: int8x8_t, c: int8x8_t) -> int8x8_t {
    { _vtbl2(a, b, c) }
}
///Table look-up
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vtbl2_s8)
pub fn vtbl2_s8(a: int8x8x2_t, b: int8x8_t) -> int8x8_t {
    vtbl2(a.0, a.1, b)
}
///Table look-up
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vtbl2_u8)
pub fn vtbl2_u8(a: uint8x8x2_t, b: uint8x8_t) -> uint8x8_t {
    { transmute(vtbl2(transmute(a.0), transmute(a.1), transmute(b))) }
}
///Table look-up
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vtbl2_u8)
pub fn vtbl2_u8(a: uint8x8x2_t, b: uint8x8_t) -> uint8x8_t {
    let mut a: uint8x8x2_t = a;
    a.0 = unsafe { simd_shuffle(a.0, a.0, [7, 6, 5, 4, 3, 2, 1, 0]) };
    a.1 = unsafe { simd_shuffle(a.1, a.1, [7, 6, 5, 4, 3, 2, 1, 0]) };
    let b: uint8x8_t = unsafe { simd_shuffle(b, b, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: uint8x8_t = transmute(
            vtbl2(transmute(a.0), transmute(a.1), transmute(b)),
        );
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Table look-up
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vtbl2_p8)
pub fn vtbl2_p8(a: poly8x8x2_t, b: uint8x8_t) -> poly8x8_t {
    { transmute(vtbl2(transmute(a.0), transmute(a.1), transmute(b))) }
}
///Table look-up
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vtbl2_p8)
pub fn vtbl2_p8(a: poly8x8x2_t, b: uint8x8_t) -> poly8x8_t {
    let mut a: poly8x8x2_t = a;
    a.0 = unsafe { simd_shuffle(a.0, a.0, [7, 6, 5, 4, 3, 2, 1, 0]) };
    a.1 = unsafe { simd_shuffle(a.1, a.1, [7, 6, 5, 4, 3, 2, 1, 0]) };
    let b: uint8x8_t = unsafe { simd_shuffle(b, b, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: poly8x8_t = transmute(
            vtbl2(transmute(a.0), transmute(a.1), transmute(b)),
        );
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Table look-up
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vtbl3)
fn vtbl3(a: int8x8_t, b: int8x8_t, c: int8x8_t, d: int8x8_t) -> int8x8_t {
    { _vtbl3(a, b, c, d) }
}
///Table look-up
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vtbl3_s8)
pub fn vtbl3_s8(a: int8x8x3_t, b: int8x8_t) -> int8x8_t {
    vtbl3(a.0, a.1, a.2, b)
}
///Table look-up
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vtbl3_u8)
pub fn vtbl3_u8(a: uint8x8x3_t, b: uint8x8_t) -> uint8x8_t {
    { transmute(vtbl3(transmute(a.0), transmute(a.1), transmute(a.2), transmute(b))) }
}
///Table look-up
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vtbl3_u8)
pub fn vtbl3_u8(a: uint8x8x3_t, b: uint8x8_t) -> uint8x8_t {
    let mut a: uint8x8x3_t = a;
    a.0 = unsafe { simd_shuffle(a.0, a.0, [7, 6, 5, 4, 3, 2, 1, 0]) };
    a.1 = unsafe { simd_shuffle(a.1, a.1, [7, 6, 5, 4, 3, 2, 1, 0]) };
    a.2 = unsafe { simd_shuffle(a.2, a.2, [7, 6, 5, 4, 3, 2, 1, 0]) };
    let b: uint8x8_t = unsafe { simd_shuffle(b, b, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: uint8x8_t = transmute(
            vtbl3(transmute(a.0), transmute(a.1), transmute(a.2), transmute(b)),
        );
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Table look-up
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vtbl3_p8)
pub fn vtbl3_p8(a: poly8x8x3_t, b: uint8x8_t) -> poly8x8_t {
    { transmute(vtbl3(transmute(a.0), transmute(a.1), transmute(a.2), transmute(b))) }
}
///Table look-up
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vtbl3_p8)
pub fn vtbl3_p8(a: poly8x8x3_t, b: uint8x8_t) -> poly8x8_t {
    let mut a: poly8x8x3_t = a;
    a.0 = unsafe { simd_shuffle(a.0, a.0, [7, 6, 5, 4, 3, 2, 1, 0]) };
    a.1 = unsafe { simd_shuffle(a.1, a.1, [7, 6, 5, 4, 3, 2, 1, 0]) };
    a.2 = unsafe { simd_shuffle(a.2, a.2, [7, 6, 5, 4, 3, 2, 1, 0]) };
    let b: uint8x8_t = unsafe { simd_shuffle(b, b, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: poly8x8_t = transmute(
            vtbl3(transmute(a.0), transmute(a.1), transmute(a.2), transmute(b)),
        );
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Table look-up
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vtbl4)
fn vtbl4(a: int8x8_t, b: int8x8_t, c: int8x8_t, d: int8x8_t, e: int8x8_t) -> int8x8_t {
    { _vtbl4(a, b, c, d, e) }
}
///Table look-up
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vtbl4_s8)
pub fn vtbl4_s8(a: int8x8x4_t, b: int8x8_t) -> int8x8_t {
    vtbl4(a.0, a.1, a.2, a.3, b)
}
///Table look-up
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vtbl4_u8)
pub fn vtbl4_u8(a: uint8x8x4_t, b: uint8x8_t) -> uint8x8_t {
    {
        transmute(
            vtbl4(
                transmute(a.0),
                transmute(a.1),
                transmute(a.2),
                transmute(a.3),
                transmute(b),
            ),
        )
    }
}
///Table look-up
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vtbl4_u8)
pub fn vtbl4_u8(a: uint8x8x4_t, b: uint8x8_t) -> uint8x8_t {
    let mut a: uint8x8x4_t = a;
    a.0 = unsafe { simd_shuffle(a.0, a.0, [7, 6, 5, 4, 3, 2, 1, 0]) };
    a.1 = unsafe { simd_shuffle(a.1, a.1, [7, 6, 5, 4, 3, 2, 1, 0]) };
    a.2 = unsafe { simd_shuffle(a.2, a.2, [7, 6, 5, 4, 3, 2, 1, 0]) };
    a.3 = unsafe { simd_shuffle(a.3, a.3, [7, 6, 5, 4, 3, 2, 1, 0]) };
    let b: uint8x8_t = unsafe { simd_shuffle(b, b, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: uint8x8_t = transmute(
            vtbl4(
                transmute(a.0),
                transmute(a.1),
                transmute(a.2),
                transmute(a.3),
                transmute(b),
            ),
        );
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Table look-up
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vtbl4_p8)
pub fn vtbl4_p8(a: poly8x8x4_t, b: uint8x8_t) -> poly8x8_t {
    {
        transmute(
            vtbl4(
                transmute(a.0),
                transmute(a.1),
                transmute(a.2),
                transmute(a.3),
                transmute(b),
            ),
        )
    }
}
///Table look-up
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vtbl4_p8)
pub fn vtbl4_p8(a: poly8x8x4_t, b: uint8x8_t) -> poly8x8_t {
    let mut a: poly8x8x4_t = a;
    a.0 = unsafe { simd_shuffle(a.0, a.0, [7, 6, 5, 4, 3, 2, 1, 0]) };
    a.1 = unsafe { simd_shuffle(a.1, a.1, [7, 6, 5, 4, 3, 2, 1, 0]) };
    a.2 = unsafe { simd_shuffle(a.2, a.2, [7, 6, 5, 4, 3, 2, 1, 0]) };
    a.3 = unsafe { simd_shuffle(a.3, a.3, [7, 6, 5, 4, 3, 2, 1, 0]) };
    let b: uint8x8_t = unsafe { simd_shuffle(b, b, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: poly8x8_t = transmute(
            vtbl4(
                transmute(a.0),
                transmute(a.1),
                transmute(a.2),
                transmute(a.3),
                transmute(b),
            ),
        );
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Extended table look-up
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vtbx1)
fn vtbx1(a: int8x8_t, b: int8x8_t, c: int8x8_t) -> int8x8_t {
    { _vtbx1(a, b, c) }
}
///Extended table look-up
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vtbx1_s8)
pub fn vtbx1_s8(a: int8x8_t, b: int8x8_t, c: int8x8_t) -> int8x8_t {
    vtbx1(a, b, c)
}
///Extended table look-up
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vtbx1_u8)
pub fn vtbx1_u8(a: uint8x8_t, b: uint8x8_t, c: uint8x8_t) -> uint8x8_t {
    { transmute(vtbx1(transmute(a), transmute(b), transmute(c))) }
}
///Extended table look-up
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vtbx1_u8)
pub fn vtbx1_u8(a: uint8x8_t, b: uint8x8_t, c: uint8x8_t) -> uint8x8_t {
    let a: uint8x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    let b: uint8x8_t = unsafe { simd_shuffle(b, b, [7, 6, 5, 4, 3, 2, 1, 0]) };
    let c: uint8x8_t = unsafe { simd_shuffle(c, c, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: uint8x8_t = transmute(
            vtbx1(transmute(a), transmute(b), transmute(c)),
        );
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Extended table look-up
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vtbx1_p8)
pub fn vtbx1_p8(a: poly8x8_t, b: poly8x8_t, c: uint8x8_t) -> poly8x8_t {
    { transmute(vtbx1(transmute(a), transmute(b), transmute(c))) }
}
///Extended table look-up
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vtbx1_p8)
pub fn vtbx1_p8(a: poly8x8_t, b: poly8x8_t, c: uint8x8_t) -> poly8x8_t {
    let a: poly8x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    let b: poly8x8_t = unsafe { simd_shuffle(b, b, [7, 6, 5, 4, 3, 2, 1, 0]) };
    let c: uint8x8_t = unsafe { simd_shuffle(c, c, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: poly8x8_t = transmute(
            vtbx1(transmute(a), transmute(b), transmute(c)),
        );
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Extended table look-up
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vtbx2)
fn vtbx2(a: int8x8_t, b: int8x8_t, c: int8x8_t, d: int8x8_t) -> int8x8_t {
    { _vtbx2(a, b, c, d) }
}
///Extended table look-up
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vtbx2_s8)
pub fn vtbx2_s8(a: int8x8_t, b: int8x8x2_t, c: int8x8_t) -> int8x8_t {
    vtbx2(a, b.0, b.1, c)
}
///Extended table look-up
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vtbx2_u8)
pub fn vtbx2_u8(a: uint8x8_t, b: uint8x8x2_t, c: uint8x8_t) -> uint8x8_t {
    { transmute(vtbx2(transmute(a), transmute(b.0), transmute(b.1), transmute(c))) }
}
///Extended table look-up
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vtbx2_u8)
pub fn vtbx2_u8(a: uint8x8_t, b: uint8x8x2_t, c: uint8x8_t) -> uint8x8_t {
    let mut b: uint8x8x2_t = b;
    let a: uint8x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    b.0 = unsafe { simd_shuffle(b.0, b.0, [7, 6, 5, 4, 3, 2, 1, 0]) };
    b.1 = unsafe { simd_shuffle(b.1, b.1, [7, 6, 5, 4, 3, 2, 1, 0]) };
    let c: uint8x8_t = unsafe { simd_shuffle(c, c, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: uint8x8_t = transmute(
            vtbx2(transmute(a), transmute(b.0), transmute(b.1), transmute(c)),
        );
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Extended table look-up
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vtbx2_p8)
pub fn vtbx2_p8(a: poly8x8_t, b: poly8x8x2_t, c: uint8x8_t) -> poly8x8_t {
    { transmute(vtbx2(transmute(a), transmute(b.0), transmute(b.1), transmute(c))) }
}
///Extended table look-up
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vtbx2_p8)
pub fn vtbx2_p8(a: poly8x8_t, b: poly8x8x2_t, c: uint8x8_t) -> poly8x8_t {
    let mut b: poly8x8x2_t = b;
    let a: poly8x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    b.0 = unsafe { simd_shuffle(b.0, b.0, [7, 6, 5, 4, 3, 2, 1, 0]) };
    b.1 = unsafe { simd_shuffle(b.1, b.1, [7, 6, 5, 4, 3, 2, 1, 0]) };
    let c: uint8x8_t = unsafe { simd_shuffle(c, c, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: poly8x8_t = transmute(
            vtbx2(transmute(a), transmute(b.0), transmute(b.1), transmute(c)),
        );
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Extended table look-up
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vtbx3)
fn vtbx3(a: int8x8_t, b: int8x8_t, c: int8x8_t, d: int8x8_t, e: int8x8_t) -> int8x8_t {
    { _vtbx3(a, b, c, d, e) }
}
///Extended table look-up
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vtbx3_s8)
pub fn vtbx3_s8(a: int8x8_t, b: int8x8x3_t, c: int8x8_t) -> int8x8_t {
    vtbx3(a, b.0, b.1, b.2, c)
}
///Extended table look-up
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vtbx3_u8)
pub fn vtbx3_u8(a: uint8x8_t, b: uint8x8x3_t, c: uint8x8_t) -> uint8x8_t {
    {
        transmute(
            vtbx3(
                transmute(a),
                transmute(b.0),
                transmute(b.1),
                transmute(b.2),
                transmute(c),
            ),
        )
    }
}
///Extended table look-up
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vtbx3_u8)
pub fn vtbx3_u8(a: uint8x8_t, b: uint8x8x3_t, c: uint8x8_t) -> uint8x8_t {
    let mut b: uint8x8x3_t = b;
    let a: uint8x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    b.0 = unsafe { simd_shuffle(b.0, b.0, [7, 6, 5, 4, 3, 2, 1, 0]) };
    b.1 = unsafe { simd_shuffle(b.1, b.1, [7, 6, 5, 4, 3, 2, 1, 0]) };
    b.2 = unsafe { simd_shuffle(b.2, b.2, [7, 6, 5, 4, 3, 2, 1, 0]) };
    let c: uint8x8_t = unsafe { simd_shuffle(c, c, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: uint8x8_t = transmute(
            vtbx3(
                transmute(a),
                transmute(b.0),
                transmute(b.1),
                transmute(b.2),
                transmute(c),
            ),
        );
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Extended table look-up
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vtbx3_p8)
pub fn vtbx3_p8(a: poly8x8_t, b: poly8x8x3_t, c: uint8x8_t) -> poly8x8_t {
    {
        transmute(
            vtbx3(
                transmute(a),
                transmute(b.0),
                transmute(b.1),
                transmute(b.2),
                transmute(c),
            ),
        )
    }
}
///Extended table look-up
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vtbx3_p8)
pub fn vtbx3_p8(a: poly8x8_t, b: poly8x8x3_t, c: uint8x8_t) -> poly8x8_t {
    let mut b: poly8x8x3_t = b;
    let a: poly8x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    b.0 = unsafe { simd_shuffle(b.0, b.0, [7, 6, 5, 4, 3, 2, 1, 0]) };
    b.1 = unsafe { simd_shuffle(b.1, b.1, [7, 6, 5, 4, 3, 2, 1, 0]) };
    b.2 = unsafe { simd_shuffle(b.2, b.2, [7, 6, 5, 4, 3, 2, 1, 0]) };
    let c: uint8x8_t = unsafe { simd_shuffle(c, c, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: poly8x8_t = transmute(
            vtbx3(
                transmute(a),
                transmute(b.0),
                transmute(b.1),
                transmute(b.2),
                transmute(c),
            ),
        );
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Extended table look-up
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vtbx4)
fn vtbx4(
    a: int8x8_t,
    b: int8x8_t,
    c: int8x8_t,
    d: int8x8_t,
    e: int8x8_t,
    f: int8x8_t,
) -> int8x8_t {
    { _vtbx4(a, b, c, d, e, f) }
}
///Extended table look-up
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vtbx4_s8)
pub fn vtbx4_s8(a: int8x8_t, b: int8x8x4_t, c: int8x8_t) -> int8x8_t {
    { vtbx4(a, transmute(b.0), transmute(b.1), transmute(b.2), transmute(b.3), c) }
}
///Extended table look-up
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vtbx4_s8)
pub fn vtbx4_s8(a: int8x8_t, b: int8x8x4_t, c: int8x8_t) -> int8x8_t {
    let mut b: int8x8x4_t = b;
    let a: int8x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    b.0 = unsafe { simd_shuffle(b.0, b.0, [7, 6, 5, 4, 3, 2, 1, 0]) };
    b.1 = unsafe { simd_shuffle(b.1, b.1, [7, 6, 5, 4, 3, 2, 1, 0]) };
    b.2 = unsafe { simd_shuffle(b.2, b.2, [7, 6, 5, 4, 3, 2, 1, 0]) };
    b.3 = unsafe { simd_shuffle(b.3, b.3, [7, 6, 5, 4, 3, 2, 1, 0]) };
    let c: int8x8_t = unsafe { simd_shuffle(c, c, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: int8x8_t = vtbx4(
            a,
            transmute(b.0),
            transmute(b.1),
            transmute(b.2),
            transmute(b.3),
            c,
        );
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Extended table look-up
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vtbx4_u8)
pub fn vtbx4_u8(a: uint8x8_t, b: uint8x8x4_t, c: uint8x8_t) -> uint8x8_t {
    {
        transmute(
            vtbx4(
                transmute(a),
                transmute(b.0),
                transmute(b.1),
                transmute(b.2),
                transmute(b.3),
                transmute(c),
            ),
        )
    }
}
///Extended table look-up
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vtbx4_u8)
pub fn vtbx4_u8(a: uint8x8_t, b: uint8x8x4_t, c: uint8x8_t) -> uint8x8_t {
    let mut b: uint8x8x4_t = b;
    let a: uint8x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    b.0 = unsafe { simd_shuffle(b.0, b.0, [7, 6, 5, 4, 3, 2, 1, 0]) };
    b.1 = unsafe { simd_shuffle(b.1, b.1, [7, 6, 5, 4, 3, 2, 1, 0]) };
    b.2 = unsafe { simd_shuffle(b.2, b.2, [7, 6, 5, 4, 3, 2, 1, 0]) };
    b.3 = unsafe { simd_shuffle(b.3, b.3, [7, 6, 5, 4, 3, 2, 1, 0]) };
    let c: uint8x8_t = unsafe { simd_shuffle(c, c, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: uint8x8_t = transmute(
            vtbx4(
                transmute(a),
                transmute(b.0),
                transmute(b.1),
                transmute(b.2),
                transmute(b.3),
                transmute(c),
            ),
        );
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Extended table look-up
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vtbx4_p8)
pub fn vtbx4_p8(a: poly8x8_t, b: poly8x8x4_t, c: uint8x8_t) -> poly8x8_t {
    {
        transmute(
            vtbx4(
                transmute(a),
                transmute(b.0),
                transmute(b.1),
                transmute(b.2),
                transmute(b.3),
                transmute(c),
            ),
        )
    }
}
///Extended table look-up
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vtbx4_p8)
pub fn vtbx4_p8(a: poly8x8_t, b: poly8x8x4_t, c: uint8x8_t) -> poly8x8_t {
    let mut b: poly8x8x4_t = b;
    let a: poly8x8_t = unsafe { simd_shuffle(a, a, [7, 6, 5, 4, 3, 2, 1, 0]) };
    b.0 = unsafe { simd_shuffle(b.0, b.0, [7, 6, 5, 4, 3, 2, 1, 0]) };
    b.1 = unsafe { simd_shuffle(b.1, b.1, [7, 6, 5, 4, 3, 2, 1, 0]) };
    b.2 = unsafe { simd_shuffle(b.2, b.2, [7, 6, 5, 4, 3, 2, 1, 0]) };
    b.3 = unsafe { simd_shuffle(b.3, b.3, [7, 6, 5, 4, 3, 2, 1, 0]) };
    let c: uint8x8_t = unsafe { simd_shuffle(c, c, [7, 6, 5, 4, 3, 2, 1, 0]) };
    {
        let ret_val: poly8x8_t = transmute(
            vtbx4(
                transmute(a),
                transmute(b.0),
                transmute(b.1),
                transmute(b.2),
                transmute(b.3),
                transmute(c),
            ),
        );
        simd_shuffle(ret_val, ret_val, [7, 6, 5, 4, 3, 2, 1, 0])
    }
}
///Transpose elements
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vtrn_f16)
pub fn vtrn_f16(a: float16x4_t, b: float16x4_t) -> float16x4x2_t {
    {
        let a1: float16x4_t = simd_shuffle(a, b, [0, 4, 2, 6]);
        let b1: float16x4_t = simd_shuffle(a, b, [1, 5, 3, 7]);
        transmute((a1, b1))
    }
}
///Transpose elements
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vtrnq_f16)
pub fn vtrnq_f16(a: float16x8_t, b: float16x8_t) -> float16x8x2_t {
    {
        let a1: float16x8_t = simd_shuffle(a, b, [0, 8, 2, 10, 4, 12, 6, 14]);
        let b1: float16x8_t = simd_shuffle(a, b, [1, 9, 3, 11, 5, 13, 7, 15]);
        transmute((a1, b1))
    }
}
///Transpose elements
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vtrn_f32)
pub fn vtrn_f32(a: float32x2_t, b: float32x2_t) -> float32x2x2_t {
    {
        let a1: float32x2_t = simd_shuffle(a, b, [0, 2]);
        let b1: float32x2_t = simd_shuffle(a, b, [1, 3]);
        transmute((a1, b1))
    }
}
///Transpose elements
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vtrn_s32)
pub fn vtrn_s32(a: int32x2_t, b: int32x2_t) -> int32x2x2_t {
    {
        let a1: int32x2_t = simd_shuffle(a, b, [0, 2]);
        let b1: int32x2_t = simd_shuffle(a, b, [1, 3]);
        transmute((a1, b1))
    }
}
///Transpose elements
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vtrn_u32)
pub fn vtrn_u32(a: uint32x2_t, b: uint32x2_t) -> uint32x2x2_t {
    {
        let a1: uint32x2_t = simd_shuffle(a, b, [0, 2]);
        let b1: uint32x2_t = simd_shuffle(a, b, [1, 3]);
        transmute((a1, b1))
    }
}
///Transpose elements
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vtrnq_f32)
pub fn vtrnq_f32(a: float32x4_t, b: float32x4_t) -> float32x4x2_t {
    {
        let a1: float32x4_t = simd_shuffle(a, b, [0, 4, 2, 6]);
        let b1: float32x4_t = simd_shuffle(a, b, [1, 5, 3, 7]);
        transmute((a1, b1))
    }
}
///Transpose elements
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vtrn_s8)
pub fn vtrn_s8(a: int8x8_t, b: int8x8_t) -> int8x8x2_t {
    {
        let a1: int8x8_t = simd_shuffle(a, b, [0, 8, 2, 10, 4, 12, 6, 14]);
        let b1: int8x8_t = simd_shuffle(a, b, [1, 9, 3, 11, 5, 13, 7, 15]);
        transmute((a1, b1))
    }
}
///Transpose elements
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vtrnq_s8)
pub fn vtrnq_s8(a: int8x16_t, b: int8x16_t) -> int8x16x2_t {
    {
        let a1: int8x16_t = simd_shuffle(
            a, b, [0, 16, 2, 18, 4, 20, 6, 22, 8, 24, 10, 26, 12, 28, 14, 30]
        );
        let b1: int8x16_t = simd_shuffle(
            a, b, [1, 17, 3, 19, 5, 21, 7, 23, 9, 25, 11, 27, 13, 29, 15, 31]
        );
        transmute((a1, b1))
    }
}
///Transpose elements
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vtrn_s16)
pub fn vtrn_s16(a: int16x4_t, b: int16x4_t) -> int16x4x2_t {
    {
        let a1: int16x4_t = simd_shuffle(a, b, [0, 4, 2, 6]);
        let b1: int16x4_t = simd_shuffle(a, b, [1, 5, 3, 7]);
        transmute((a1, b1))
    }
}
///Transpose elements
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vtrnq_s16)
pub fn vtrnq_s16(a: int16x8_t, b: int16x8_t) -> int16x8x2_t {
    {
        let a1: int16x8_t = simd_shuffle(a, b, [0, 8, 2, 10, 4, 12, 6, 14]);
        let b1: int16x8_t = simd_shuffle(a, b, [1, 9, 3, 11, 5, 13, 7, 15]);
        transmute((a1, b1))
    }
}
///Transpose elements
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vtrnq_s32)
pub fn vtrnq_s32(a: int32x4_t, b: int32x4_t) -> int32x4x2_t {
    {
        let a1: int32x4_t = simd_shuffle(a, b, [0, 4, 2, 6]);
        let b1: int32x4_t = simd_shuffle(a, b, [1, 5, 3, 7]);
        transmute((a1, b1))
    }
}
///Transpose elements
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vtrn_u8)
pub fn vtrn_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8x2_t {
    {
        let a1: uint8x8_t = simd_shuffle(a, b, [0, 8, 2, 10, 4, 12, 6, 14]);
        let b1: uint8x8_t = simd_shuffle(a, b, [1, 9, 3, 11, 5, 13, 7, 15]);
        transmute((a1, b1))
    }
}
///Transpose elements
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vtrnq_u8)
pub fn vtrnq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16x2_t {
    {
        let a1: uint8x16_t = simd_shuffle(
            a, b, [0, 16, 2, 18, 4, 20, 6, 22, 8, 24, 10, 26, 12, 28, 14, 30]
        );
        let b1: uint8x16_t = simd_shuffle(
            a, b, [1, 17, 3, 19, 5, 21, 7, 23, 9, 25, 11, 27, 13, 29, 15, 31]
        );
        transmute((a1, b1))
    }
}
///Transpose elements
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vtrn_u16)
pub fn vtrn_u16(a: uint16x4_t, b: uint16x4_t) -> uint16x4x2_t {
    {
        let a1: uint16x4_t = simd_shuffle(a, b, [0, 4, 2, 6]);
        let b1: uint16x4_t = simd_shuffle(a, b, [1, 5, 3, 7]);
        transmute((a1, b1))
    }
}
///Transpose elements
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vtrnq_u16)
pub fn vtrnq_u16(a: uint16x8_t, b: uint16x8_t) -> uint16x8x2_t {
    {
        let a1: uint16x8_t = simd_shuffle(a, b, [0, 8, 2, 10, 4, 12, 6, 14]);
        let b1: uint16x8_t = simd_shuffle(a, b, [1, 9, 3, 11, 5, 13, 7, 15]);
        transmute((a1, b1))
    }
}
///Transpose elements
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vtrnq_u32)
pub fn vtrnq_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4x2_t {
    {
        let a1: uint32x4_t = simd_shuffle(a, b, [0, 4, 2, 6]);
        let b1: uint32x4_t = simd_shuffle(a, b, [1, 5, 3, 7]);
        transmute((a1, b1))
    }
}
///Transpose elements
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vtrn_p8)
pub fn vtrn_p8(a: poly8x8_t, b: poly8x8_t) -> poly8x8x2_t {
    {
        let a1: poly8x8_t = simd_shuffle(a, b, [0, 8, 2, 10, 4, 12, 6, 14]);
        let b1: poly8x8_t = simd_shuffle(a, b, [1, 9, 3, 11, 5, 13, 7, 15]);
        transmute((a1, b1))
    }
}
///Transpose elements
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vtrnq_p8)
pub fn vtrnq_p8(a: poly8x16_t, b: poly8x16_t) -> poly8x16x2_t {
    {
        let a1: poly8x16_t = simd_shuffle(
            a, b, [0, 16, 2, 18, 4, 20, 6, 22, 8, 24, 10, 26, 12, 28, 14, 30]
        );
        let b1: poly8x16_t = simd_shuffle(
            a, b, [1, 17, 3, 19, 5, 21, 7, 23, 9, 25, 11, 27, 13, 29, 15, 31]
        );
        transmute((a1, b1))
    }
}
///Transpose elements
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vtrn_p16)
pub fn vtrn_p16(a: poly16x4_t, b: poly16x4_t) -> poly16x4x2_t {
    {
        let a1: poly16x4_t = simd_shuffle(a, b, [0, 4, 2, 6]);
        let b1: poly16x4_t = simd_shuffle(a, b, [1, 5, 3, 7]);
        transmute((a1, b1))
    }
}
///Transpose elements
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vtrnq_p16)
pub fn vtrnq_p16(a: poly16x8_t, b: poly16x8_t) -> poly16x8x2_t {
    {
        let a1: poly16x8_t = simd_shuffle(a, b, [0, 8, 2, 10, 4, 12, 6, 14]);
        let b1: poly16x8_t = simd_shuffle(a, b, [1, 9, 3, 11, 5, 13, 7, 15]);
        transmute((a1, b1))
    }
}
///Signed compare bitwise Test bits nonzero
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vtst_s8)
pub fn vtst_s8(a: int8x8_t, b: int8x8_t) -> uint8x8_t {
    {
        let c: int8x8_t = simd_and(a, b);
        let d: i8x8 = i8x8::new(0, 0, 0, 0, 0, 0, 0, 0);
        simd_ne(c, transmute(d))
    }
}
///Signed compare bitwise Test bits nonzero
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vtstq_s8)
pub fn vtstq_s8(a: int8x16_t, b: int8x16_t) -> uint8x16_t {
    {
        let c: int8x16_t = simd_and(a, b);
        let d: i8x16 = i8x16::new(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0);
        simd_ne(c, transmute(d))
    }
}
///Signed compare bitwise Test bits nonzero
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vtst_s16)
pub fn vtst_s16(a: int16x4_t, b: int16x4_t) -> uint16x4_t {
    {
        let c: int16x4_t = simd_and(a, b);
        let d: i16x4 = i16x4::new(0, 0, 0, 0);
        simd_ne(c, transmute(d))
    }
}
///Signed compare bitwise Test bits nonzero
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vtstq_s16)
pub fn vtstq_s16(a: int16x8_t, b: int16x8_t) -> uint16x8_t {
    {
        let c: int16x8_t = simd_and(a, b);
        let d: i16x8 = i16x8::new(0, 0, 0, 0, 0, 0, 0, 0);
        simd_ne(c, transmute(d))
    }
}
///Signed compare bitwise Test bits nonzero
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vtst_s32)
pub fn vtst_s32(a: int32x2_t, b: int32x2_t) -> uint32x2_t {
    {
        let c: int32x2_t = simd_and(a, b);
        let d: i32x2 = i32x2::new(0, 0);
        simd_ne(c, transmute(d))
    }
}
///Signed compare bitwise Test bits nonzero
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vtstq_s32)
pub fn vtstq_s32(a: int32x4_t, b: int32x4_t) -> uint32x4_t {
    {
        let c: int32x4_t = simd_and(a, b);
        let d: i32x4 = i32x4::new(0, 0, 0, 0);
        simd_ne(c, transmute(d))
    }
}
///Signed compare bitwise Test bits nonzero
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vtst_p8)
pub fn vtst_p8(a: poly8x8_t, b: poly8x8_t) -> uint8x8_t {
    {
        let c: poly8x8_t = simd_and(a, b);
        let d: i8x8 = i8x8::new(0, 0, 0, 0, 0, 0, 0, 0);
        simd_ne(c, transmute(d))
    }
}
///Signed compare bitwise Test bits nonzero
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vtstq_p8)
pub fn vtstq_p8(a: poly8x16_t, b: poly8x16_t) -> uint8x16_t {
    {
        let c: poly8x16_t = simd_and(a, b);
        let d: i8x16 = i8x16::new(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0);
        simd_ne(c, transmute(d))
    }
}
///Signed compare bitwise Test bits nonzero
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vtst_p16)
pub fn vtst_p16(a: poly16x4_t, b: poly16x4_t) -> uint16x4_t {
    {
        let c: poly16x4_t = simd_and(a, b);
        let d: i16x4 = i16x4::new(0, 0, 0, 0);
        simd_ne(c, transmute(d))
    }
}
///Signed compare bitwise Test bits nonzero
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vtstq_p16)
pub fn vtstq_p16(a: poly16x8_t, b: poly16x8_t) -> uint16x8_t {
    {
        let c: poly16x8_t = simd_and(a, b);
        let d: i16x8 = i16x8::new(0, 0, 0, 0, 0, 0, 0, 0);
        simd_ne(c, transmute(d))
    }
}
///Unsigned compare bitwise Test bits nonzero
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vtst_u8)
pub fn vtst_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
    {
        let c: uint8x8_t = simd_and(a, b);
        let d: u8x8 = u8x8::new(0, 0, 0, 0, 0, 0, 0, 0);
        simd_ne(c, transmute(d))
    }
}
///Unsigned compare bitwise Test bits nonzero
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vtstq_u8)
pub fn vtstq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
    {
        let c: uint8x16_t = simd_and(a, b);
        let d: u8x16 = u8x16::new(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0);
        simd_ne(c, transmute(d))
    }
}
///Unsigned compare bitwise Test bits nonzero
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vtst_u16)
pub fn vtst_u16(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
    {
        let c: uint16x4_t = simd_and(a, b);
        let d: u16x4 = u16x4::new(0, 0, 0, 0);
        simd_ne(c, transmute(d))
    }
}
///Unsigned compare bitwise Test bits nonzero
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vtstq_u16)
pub fn vtstq_u16(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
    {
        let c: uint16x8_t = simd_and(a, b);
        let d: u16x8 = u16x8::new(0, 0, 0, 0, 0, 0, 0, 0);
        simd_ne(c, transmute(d))
    }
}
///Unsigned compare bitwise Test bits nonzero
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vtst_u32)
pub fn vtst_u32(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
    {
        let c: uint32x2_t = simd_and(a, b);
        let d: u32x2 = u32x2::new(0, 0);
        simd_ne(c, transmute(d))
    }
}
///Unsigned compare bitwise Test bits nonzero
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vtstq_u32)
pub fn vtstq_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
    {
        let c: uint32x4_t = simd_and(a, b);
        let d: u32x4 = u32x4::new(0, 0, 0, 0);
        simd_ne(c, transmute(d))
    }
}
///Dot product index form with unsigned and signed integers
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vusdot_lane_s32)
pub fn vusdot_lane_s32<const LANE: i32>(
    a: int32x2_t,
    b: uint8x8_t,
    c: int8x8_t,
) -> int32x2_t {
    static_assert_uimm_bits!(LANE, 1);
    {
        let c: int32x2_t = transmute(c);
        let c: int32x2_t = simd_shuffle(c, c, [LANE as u32, LANE as u32]);
        vusdot_s32(a, b, transmute(c))
    }
}
///Dot product index form with unsigned and signed integers
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vusdotq_lane_s32)
pub fn vusdotq_lane_s32<const LANE: i32>(
    a: int32x4_t,
    b: uint8x16_t,
    c: int8x8_t,
) -> int32x4_t {
    static_assert_uimm_bits!(LANE, 1);
    {
        let c: int32x2_t = transmute(c);
        let c: int32x4_t = simd_shuffle(
            c, c, [LANE as u32, LANE as u32, LANE as u32, LANE as u32]
        );
        vusdotq_s32(a, b, transmute(c))
    }
}
///Dot product vector form with unsigned and signed integers
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vusdot_s32)
pub fn vusdot_s32(a: int32x2_t, b: uint8x8_t, c: int8x8_t) -> int32x2_t {
    { _vusdot_s32(a, b, c) }
}
///Dot product vector form with unsigned and signed integers
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vusdotq_s32)
pub fn vusdotq_s32(a: int32x4_t, b: uint8x16_t, c: int8x16_t) -> int32x4_t {
    { _vusdotq_s32(a, b, c) }
}
///Unsigned and signed 8-bit integer matrix multiply-accumulate
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vusmmlaq_s32)
pub fn vusmmlaq_s32(a: int32x4_t, b: uint8x16_t, c: int8x16_t) -> int32x4_t {
    { _vusmmlaq_s32(a, b, c) }
}
///Unzip vectors
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vuzp_f16)
pub fn vuzp_f16(a: float16x4_t, b: float16x4_t) -> float16x4x2_t {
    {
        let a0: float16x4_t = simd_shuffle(a, b, [0, 2, 4, 6]);
        let b0: float16x4_t = simd_shuffle(a, b, [1, 3, 5, 7]);
        transmute((a0, b0))
    }
}
///Unzip vectors
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vuzpq_f16)
pub fn vuzpq_f16(a: float16x8_t, b: float16x8_t) -> float16x8x2_t {
    {
        let a0: float16x8_t = simd_shuffle(a, b, [0, 2, 4, 6, 8, 10, 12, 14]);
        let b0: float16x8_t = simd_shuffle(a, b, [1, 3, 5, 7, 9, 11, 13, 15]);
        transmute((a0, b0))
    }
}
///Unzip vectors
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vuzp_f32)
pub fn vuzp_f32(a: float32x2_t, b: float32x2_t) -> float32x2x2_t {
    {
        let a0: float32x2_t = simd_shuffle(a, b, [0, 2]);
        let b0: float32x2_t = simd_shuffle(a, b, [1, 3]);
        transmute((a0, b0))
    }
}
///Unzip vectors
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vuzp_s32)
pub fn vuzp_s32(a: int32x2_t, b: int32x2_t) -> int32x2x2_t {
    {
        let a0: int32x2_t = simd_shuffle(a, b, [0, 2]);
        let b0: int32x2_t = simd_shuffle(a, b, [1, 3]);
        transmute((a0, b0))
    }
}
///Unzip vectors
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vuzp_u32)
pub fn vuzp_u32(a: uint32x2_t, b: uint32x2_t) -> uint32x2x2_t {
    {
        let a0: uint32x2_t = simd_shuffle(a, b, [0, 2]);
        let b0: uint32x2_t = simd_shuffle(a, b, [1, 3]);
        transmute((a0, b0))
    }
}
///Unzip vectors
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vuzpq_f32)
pub fn vuzpq_f32(a: float32x4_t, b: float32x4_t) -> float32x4x2_t {
    {
        let a0: float32x4_t = simd_shuffle(a, b, [0, 2, 4, 6]);
        let b0: float32x4_t = simd_shuffle(a, b, [1, 3, 5, 7]);
        transmute((a0, b0))
    }
}
///Unzip vectors
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vuzp_s8)
pub fn vuzp_s8(a: int8x8_t, b: int8x8_t) -> int8x8x2_t {
    {
        let a0: int8x8_t = simd_shuffle(a, b, [0, 2, 4, 6, 8, 10, 12, 14]);
        let b0: int8x8_t = simd_shuffle(a, b, [1, 3, 5, 7, 9, 11, 13, 15]);
        transmute((a0, b0))
    }
}
///Unzip vectors
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vuzpq_s8)
pub fn vuzpq_s8(a: int8x16_t, b: int8x16_t) -> int8x16x2_t {
    {
        let a0: int8x16_t = simd_shuffle(
            a, b, [0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30]
        );
        let b0: int8x16_t = simd_shuffle(
            a, b, [1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31]
        );
        transmute((a0, b0))
    }
}
///Unzip vectors
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vuzp_s16)
pub fn vuzp_s16(a: int16x4_t, b: int16x4_t) -> int16x4x2_t {
    {
        let a0: int16x4_t = simd_shuffle(a, b, [0, 2, 4, 6]);
        let b0: int16x4_t = simd_shuffle(a, b, [1, 3, 5, 7]);
        transmute((a0, b0))
    }
}
///Unzip vectors
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vuzpq_s16)
pub fn vuzpq_s16(a: int16x8_t, b: int16x8_t) -> int16x8x2_t {
    {
        let a0: int16x8_t = simd_shuffle(a, b, [0, 2, 4, 6, 8, 10, 12, 14]);
        let b0: int16x8_t = simd_shuffle(a, b, [1, 3, 5, 7, 9, 11, 13, 15]);
        transmute((a0, b0))
    }
}
///Unzip vectors
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vuzpq_s32)
pub fn vuzpq_s32(a: int32x4_t, b: int32x4_t) -> int32x4x2_t {
    {
        let a0: int32x4_t = simd_shuffle(a, b, [0, 2, 4, 6]);
        let b0: int32x4_t = simd_shuffle(a, b, [1, 3, 5, 7]);
        transmute((a0, b0))
    }
}
///Unzip vectors
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vuzp_u8)
pub fn vuzp_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8x2_t {
    {
        let a0: uint8x8_t = simd_shuffle(a, b, [0, 2, 4, 6, 8, 10, 12, 14]);
        let b0: uint8x8_t = simd_shuffle(a, b, [1, 3, 5, 7, 9, 11, 13, 15]);
        transmute((a0, b0))
    }
}
///Unzip vectors
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vuzpq_u8)
pub fn vuzpq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16x2_t {
    {
        let a0: uint8x16_t = simd_shuffle(
            a, b, [0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30]
        );
        let b0: uint8x16_t = simd_shuffle(
            a, b, [1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31]
        );
        transmute((a0, b0))
    }
}
///Unzip vectors
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vuzp_u16)
pub fn vuzp_u16(a: uint16x4_t, b: uint16x4_t) -> uint16x4x2_t {
    {
        let a0: uint16x4_t = simd_shuffle(a, b, [0, 2, 4, 6]);
        let b0: uint16x4_t = simd_shuffle(a, b, [1, 3, 5, 7]);
        transmute((a0, b0))
    }
}
///Unzip vectors
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vuzpq_u16)
pub fn vuzpq_u16(a: uint16x8_t, b: uint16x8_t) -> uint16x8x2_t {
    {
        let a0: uint16x8_t = simd_shuffle(a, b, [0, 2, 4, 6, 8, 10, 12, 14]);
        let b0: uint16x8_t = simd_shuffle(a, b, [1, 3, 5, 7, 9, 11, 13, 15]);
        transmute((a0, b0))
    }
}
///Unzip vectors
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vuzpq_u32)
pub fn vuzpq_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4x2_t {
    {
        let a0: uint32x4_t = simd_shuffle(a, b, [0, 2, 4, 6]);
        let b0: uint32x4_t = simd_shuffle(a, b, [1, 3, 5, 7]);
        transmute((a0, b0))
    }
}
///Unzip vectors
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vuzp_p8)
pub fn vuzp_p8(a: poly8x8_t, b: poly8x8_t) -> poly8x8x2_t {
    {
        let a0: poly8x8_t = simd_shuffle(a, b, [0, 2, 4, 6, 8, 10, 12, 14]);
        let b0: poly8x8_t = simd_shuffle(a, b, [1, 3, 5, 7, 9, 11, 13, 15]);
        transmute((a0, b0))
    }
}
///Unzip vectors
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vuzpq_p8)
pub fn vuzpq_p8(a: poly8x16_t, b: poly8x16_t) -> poly8x16x2_t {
    {
        let a0: poly8x16_t = simd_shuffle(
            a, b, [0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30]
        );
        let b0: poly8x16_t = simd_shuffle(
            a, b, [1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31]
        );
        transmute((a0, b0))
    }
}
///Unzip vectors
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vuzp_p16)
pub fn vuzp_p16(a: poly16x4_t, b: poly16x4_t) -> poly16x4x2_t {
    {
        let a0: poly16x4_t = simd_shuffle(a, b, [0, 2, 4, 6]);
        let b0: poly16x4_t = simd_shuffle(a, b, [1, 3, 5, 7]);
        transmute((a0, b0))
    }
}
///Unzip vectors
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vuzpq_p16)
pub fn vuzpq_p16(a: poly16x8_t, b: poly16x8_t) -> poly16x8x2_t {
    {
        let a0: poly16x8_t = simd_shuffle(a, b, [0, 2, 4, 6, 8, 10, 12, 14]);
        let b0: poly16x8_t = simd_shuffle(a, b, [1, 3, 5, 7, 9, 11, 13, 15]);
        transmute((a0, b0))
    }
}
///Zip vectors
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vzip_f16)
pub fn vzip_f16(a: float16x4_t, b: float16x4_t) -> float16x4x2_t {
    {
        let a0: float16x4_t = simd_shuffle(a, b, [0, 4, 1, 5]);
        let b0: float16x4_t = simd_shuffle(a, b, [2, 6, 3, 7]);
        transmute((a0, b0))
    }
}
///Zip vectors
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vzipq_f16)
pub fn vzipq_f16(a: float16x8_t, b: float16x8_t) -> float16x8x2_t {
    {
        let a0: float16x8_t = simd_shuffle(a, b, [0, 8, 1, 9, 2, 10, 3, 11]);
        let b0: float16x8_t = simd_shuffle(a, b, [4, 12, 5, 13, 6, 14, 7, 15]);
        transmute((a0, b0))
    }
}
///Zip vectors
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vzip_f32)
pub fn vzip_f32(a: float32x2_t, b: float32x2_t) -> float32x2x2_t {
    {
        let a0: float32x2_t = simd_shuffle(a, b, [0, 2]);
        let b0: float32x2_t = simd_shuffle(a, b, [1, 3]);
        transmute((a0, b0))
    }
}
///Zip vectors
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vzip_s32)
pub fn vzip_s32(a: int32x2_t, b: int32x2_t) -> int32x2x2_t {
    {
        let a0: int32x2_t = simd_shuffle(a, b, [0, 2]);
        let b0: int32x2_t = simd_shuffle(a, b, [1, 3]);
        transmute((a0, b0))
    }
}
///Zip vectors
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vzip_u32)
pub fn vzip_u32(a: uint32x2_t, b: uint32x2_t) -> uint32x2x2_t {
    {
        let a0: uint32x2_t = simd_shuffle(a, b, [0, 2]);
        let b0: uint32x2_t = simd_shuffle(a, b, [1, 3]);
        transmute((a0, b0))
    }
}
///Zip vectors
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vzip_s8)
pub fn vzip_s8(a: int8x8_t, b: int8x8_t) -> int8x8x2_t {
    {
        let a0: int8x8_t = simd_shuffle(a, b, [0, 8, 1, 9, 2, 10, 3, 11]);
        let b0: int8x8_t = simd_shuffle(a, b, [4, 12, 5, 13, 6, 14, 7, 15]);
        transmute((a0, b0))
    }
}
///Zip vectors
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vzip_s16)
pub fn vzip_s16(a: int16x4_t, b: int16x4_t) -> int16x4x2_t {
    {
        let a0: int16x4_t = simd_shuffle(a, b, [0, 4, 1, 5]);
        let b0: int16x4_t = simd_shuffle(a, b, [2, 6, 3, 7]);
        transmute((a0, b0))
    }
}
///Zip vectors
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vzip_u8)
pub fn vzip_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8x2_t {
    {
        let a0: uint8x8_t = simd_shuffle(a, b, [0, 8, 1, 9, 2, 10, 3, 11]);
        let b0: uint8x8_t = simd_shuffle(a, b, [4, 12, 5, 13, 6, 14, 7, 15]);
        transmute((a0, b0))
    }
}
///Zip vectors
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vzip_u16)
pub fn vzip_u16(a: uint16x4_t, b: uint16x4_t) -> uint16x4x2_t {
    {
        let a0: uint16x4_t = simd_shuffle(a, b, [0, 4, 1, 5]);
        let b0: uint16x4_t = simd_shuffle(a, b, [2, 6, 3, 7]);
        transmute((a0, b0))
    }
}
///Zip vectors
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vzip_p8)
pub fn vzip_p8(a: poly8x8_t, b: poly8x8_t) -> poly8x8x2_t {
    {
        let a0: poly8x8_t = simd_shuffle(a, b, [0, 8, 1, 9, 2, 10, 3, 11]);
        let b0: poly8x8_t = simd_shuffle(a, b, [4, 12, 5, 13, 6, 14, 7, 15]);
        transmute((a0, b0))
    }
}
///Zip vectors
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vzip_p16)
pub fn vzip_p16(a: poly16x4_t, b: poly16x4_t) -> poly16x4x2_t {
    {
        let a0: poly16x4_t = simd_shuffle(a, b, [0, 4, 1, 5]);
        let b0: poly16x4_t = simd_shuffle(a, b, [2, 6, 3, 7]);
        transmute((a0, b0))
    }
}
///Zip vectors
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vzipq_f32)
pub fn vzipq_f32(a: float32x4_t, b: float32x4_t) -> float32x4x2_t {
    {
        let a0: float32x4_t = simd_shuffle(a, b, [0, 4, 1, 5]);
        let b0: float32x4_t = simd_shuffle(a, b, [2, 6, 3, 7]);
        transmute((a0, b0))
    }
}
///Zip vectors
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vzipq_s8)
pub fn vzipq_s8(a: int8x16_t, b: int8x16_t) -> int8x16x2_t {
    {
        let a0: int8x16_t = simd_shuffle(
            a, b, [0, 16, 1, 17, 2, 18, 3, 19, 4, 20, 5, 21, 6, 22, 7, 23]
        );
        let b0: int8x16_t = simd_shuffle(
            a, b, [8, 24, 9, 25, 10, 26, 11, 27, 12, 28, 13, 29, 14, 30, 15, 31]
        );
        transmute((a0, b0))
    }
}
///Zip vectors
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vzipq_s16)
pub fn vzipq_s16(a: int16x8_t, b: int16x8_t) -> int16x8x2_t {
    {
        let a0: int16x8_t = simd_shuffle(a, b, [0, 8, 1, 9, 2, 10, 3, 11]);
        let b0: int16x8_t = simd_shuffle(a, b, [4, 12, 5, 13, 6, 14, 7, 15]);
        transmute((a0, b0))
    }
}
///Zip vectors
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vzipq_s32)
pub fn vzipq_s32(a: int32x4_t, b: int32x4_t) -> int32x4x2_t {
    {
        let a0: int32x4_t = simd_shuffle(a, b, [0, 4, 1, 5]);
        let b0: int32x4_t = simd_shuffle(a, b, [2, 6, 3, 7]);
        transmute((a0, b0))
    }
}
///Zip vectors
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vzipq_u8)
pub fn vzipq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16x2_t {
    {
        let a0: uint8x16_t = simd_shuffle(
            a, b, [0, 16, 1, 17, 2, 18, 3, 19, 4, 20, 5, 21, 6, 22, 7, 23]
        );
        let b0: uint8x16_t = simd_shuffle(
            a, b, [8, 24, 9, 25, 10, 26, 11, 27, 12, 28, 13, 29, 14, 30, 15, 31]
        );
        transmute((a0, b0))
    }
}
///Zip vectors
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vzipq_u16)
pub fn vzipq_u16(a: uint16x8_t, b: uint16x8_t) -> uint16x8x2_t {
    {
        let a0: uint16x8_t = simd_shuffle(a, b, [0, 8, 1, 9, 2, 10, 3, 11]);
        let b0: uint16x8_t = simd_shuffle(a, b, [4, 12, 5, 13, 6, 14, 7, 15]);
        transmute((a0, b0))
    }
}
///Zip vectors
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vzipq_u32)
pub fn vzipq_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4x2_t {
    {
        let a0: uint32x4_t = simd_shuffle(a, b, [0, 4, 1, 5]);
        let b0: uint32x4_t = simd_shuffle(a, b, [2, 6, 3, 7]);
        transmute((a0, b0))
    }
}
///Zip vectors
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vzipq_p8)
pub fn vzipq_p8(a: poly8x16_t, b: poly8x16_t) -> poly8x16x2_t {
    {
        let a0: poly8x16_t = simd_shuffle(
            a, b, [0, 16, 1, 17, 2, 18, 3, 19, 4, 20, 5, 21, 6, 22, 7, 23]
        );
        let b0: poly8x16_t = simd_shuffle(
            a, b, [8, 24, 9, 25, 10, 26, 11, 27, 12, 28, 13, 29, 14, 30, 15, 31]
        );
        transmute((a0, b0))
    }
}
///Zip vectors
///[Arm's documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vzipq_p16)
pub fn vzipq_p16(a: poly16x8_t, b: poly16x8_t) -> poly16x8x2_t {
    {
        let a0: poly16x8_t = simd_shuffle(a, b, [0, 8, 1, 9, 2, 10, 3, 11]);
        let b0: poly16x8_t = simd_shuffle(a, b, [4, 12, 5, 13, 6, 14, 7, 15]);
        transmute((a0, b0))
    }
}

// pub fn vaba_s16(a: int16x4_t, b: int16x4_t, c: int16x4_t) -> int16x4_t {
//     simd_add(a, vabd_s16(b, c))
// }

// pub fn vaba_s32(a: int32x2_t, b: int32x2_t, c: int32x2_t) -> int32x2_t {
//     simd_add(a, vabd_s32(b, c))
// }

// pub fn vaba_s8(a: int8x8_t, b: int8x8_t, c: int8x8_t) -> int8x8_t {
//     simd_add(a, vabd_s8(b, c))
// }

// pub fn vaba_u16(a: uint16x4_t, b: uint16x4_t, c: uint16x4_t) -> uint16x4_t {
//     simd_add(a, vabd_u16(b, c))
// }

// pub fn vaba_u32(a: uint32x2_t, b: uint32x2_t, c: uint32x2_t) -> uint32x2_t {
//     simd_add(a, vabd_u32(b, c))
// }

// pub fn vaba_u8(a: uint8x8_t, b: uint8x8_t, c: uint8x8_t) -> uint8x8_t {
//     simd_add(a, vabd_u8(b, c))
// }

// pub fn vabal_u8(a: uint16x8_t, b: uint8x8_t, c: uint8x8_t) -> uint16x8_t {
//     let d: uint8x8_t = vabd_u8(b, c);
//     simd_add(a, simd_cast(d))
// }

// pub fn vabal_u16(a: uint32x4_t, b: uint16x4_t, c: uint16x4_t) -> uint32x4_t {
//     let d: uint16x4_t = vabd_u16(b, c);
//     simd_add(a, simd_cast(d))
// }

// pub fn vabal_u32(a: uint64x2_t, b: uint32x2_t, c: uint32x2_t) -> uint64x2_t {
//     let d: uint32x2_t = vabd_u32(b, c);
//     simd_add(a, simd_cast(d))
// }

// pub fn vabaq_s16(a: int16x8_t, b: int16x8_t, c: int16x8_t) -> int16x8_t {
//     simd_add(a, vabdq_s16(b, c))
// }

// pub fn vabaq_s32(a: int32x4_t, b: int32x4_t, c: int32x4_t) -> int32x4_t {
//     simd_add(a, vabdq_s32(b, c))
// }

// pub fn vabaq_s8(a: int8x16_t, b: int8x16_t, c: int8x16_t) -> int8x16_t {
//     simd_add(a, vabdq_s8(b, c))
// }

// pub fn vabaq_u16(a: uint16x8_t, b: uint16x8_t, c: uint16x8_t) -> uint16x8_t {
//     simd_add(a, vabdq_u16(b, c))
// }

// pub fn vabaq_u32(a: uint32x4_t, b: uint32x4_t, c: uint32x4_t) -> uint32x4_t {
//     simd_add(a, vabdq_u32(b, c))
// }

// pub fn vabaq_u8(a: uint8x16_t, b: uint8x16_t, c: uint8x16_t) -> uint8x16_t {
//     simd_add(a, vabdq_u8(b, c))
// }

// pub fn vabd_s8(a: int8x8_t, b: int8x8_t) -> int8x8_t {
//     simd_abs_diff(a, b)
// }

// pub fn vabdq_s8(a: int8x16_t, b: int8x16_t) -> int8x16_t {
//     simd_abs_diff(a, b)
// }

// pub fn vabd_s16(a: int16x4_t, b: int16x4_t) -> int16x4_t {
//     simd_abs_diff(a, b)
// }

// pub fn vabdq_s16(a: int16x8_t, b: int16x8_t) -> int16x8_t {
//     simd_abs_diff(a, b)
// }

// pub fn vabd_s32(a: int32x2_t, b: int32x2_t) -> int32x2_t {
//     simd_abs_diff(a, b)
// }

// pub fn vabdq_s32(a: int32x4_t, b: int32x4_t) -> int32x4_t {
//     simd_abs_diff(a, b)
// }

// pub fn vabd_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
//     simd_abs_diff(a, b)
// }

// pub fn vabdq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
//     simd_abs_diff(a, b)
// }

// pub fn vabd_u16(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
//     simd_abs_diff(a, b)
// }

// pub fn vabdq_u16(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
//     simd_abs_diff(a, b)
// }

// pub fn vabd_u32(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
//     simd_abs_diff(a, b)
// }

// pub fn vabdq_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
//     simd_abs_diff(a, b)
// }

// pub fn vabdl_u8(a: uint8x8_t, b: uint8x8_t) -> uint16x8_t {
//     simd_cast(vabd_u8(a, b))
// }

// pub fn vabdl_u16(a: uint16x4_t, b: uint16x4_t) -> uint32x4_t {
//     simd_cast(vabd_u16(a, b))
// }

// pub fn vabdl_u32(a: uint32x2_t, b: uint32x2_t) -> uint64x2_t {
//     simd_cast(vabd_u32(a, b))
// }

// pub fn vabs_s8(a: int8x8_t) -> int8x8_t {
//     simd_abs(a)
// }

// pub fn vabsq_s8(a: int8x16_t) -> int8x16_t {
//     simd_abs(a)
// }

// pub fn vabs_s16(a: int16x4_t) -> int16x4_t {
//     simd_abs(a)
// }

// pub fn vabsq_s16(a: int16x8_t) -> int16x8_t {
//     simd_abs(a)
// }

// pub fn vabs_s32(a: int32x2_t) -> int32x2_t {
//     simd_abs(a)
// }

// pub fn vabsq_s32(a: int32x4_t) -> int32x4_t {
//     simd_abs(a)
// }

// pub fn vadd_s16(a: int16x4_t, b: int16x4_t) -> int16x4_t {
//     simd_add(a, b)
// }

// pub fn vadd_s32(a: int32x2_t, b: int32x2_t) -> int32x2_t {
//     simd_add(a, b)
// }

// pub fn vadd_s8(a: int8x8_t, b: int8x8_t) -> int8x8_t {
//     simd_add(a, b)
// }

// pub fn vadd_u16(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
//     simd_add(a, b)
// }

// pub fn vadd_u32(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
//     simd_add(a, b)
// }

// pub fn vadd_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
//     simd_add(a, b)
// }

// pub fn vaddq_s16(a: int16x8_t, b: int16x8_t) -> int16x8_t {
//     simd_add(a, b)
// }

// pub fn vaddq_s32(a: int32x4_t, b: int32x4_t) -> int32x4_t {
//     simd_add(a, b)
// }

// pub fn vaddq_s64(a: int64x2_t, b: int64x2_t) -> int64x2_t {
//     simd_add(a, b)
// }

// pub fn vaddq_s8(a: int8x16_t, b: int8x16_t) -> int8x16_t {
//     simd_add(a, b)
// }

// pub fn vaddq_u16(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
//     simd_add(a, b)
// }

// pub fn vaddq_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
//     simd_add(a, b)
// }

// pub fn vaddq_u64(a: uint64x2_t, b: uint64x2_t) -> uint64x2_t {
//     simd_add(a, b)
// }

// pub fn vaddq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
//     simd_add(a, b)
// }

// pub fn vaddhn_high_s16(r: int8x8_t, a: int16x8_t, b: int16x8_t) -> int8x16_t {
//     let x = simd_cast(simd_shr(simd_add(a, b), int16x8_t::splat(8)));
//     simd_shuffle(r, x, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15])
// }

// pub fn vaddhn_high_s32(r: int16x4_t, a: int32x4_t, b: int32x4_t) -> int16x8_t {
//     let x = simd_cast(simd_shr(simd_add(a, b), int32x4_t::splat(16)));
//     simd_shuffle(r, x, [0, 1, 2, 3, 4, 5, 6, 7])
// }

// pub fn vaddhn_high_s64(r: int32x2_t, a: int64x2_t, b: int64x2_t) -> int32x4_t {
//     let x = simd_cast(simd_shr(simd_add(a, b), int64x2_t::splat(32)));
//     simd_shuffle(r, x, [0, 1, 2, 3])
// }

// pub fn vaddhn_high_u16(r: uint8x8_t, a: uint16x8_t, b: uint16x8_t) -> uint8x16_t {
//     let x = simd_cast(simd_shr(simd_add(a, b), uint16x8_t::splat(8)));
//     simd_shuffle(r, x, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15])
// }

// pub fn vaddhn_high_u32(r: uint16x4_t, a: uint32x4_t, b: uint32x4_t) -> uint16x8_t {
//     let x = simd_cast(simd_shr(simd_add(a, b), uint32x4_t::splat(16)));
//     simd_shuffle(r, x, [0, 1, 2, 3, 4, 5, 6, 7])
// }

// pub fn vaddhn_high_u64(r: uint32x2_t, a: uint64x2_t, b: uint64x2_t) -> uint32x4_t {
//     let x = simd_cast(simd_shr(simd_add(a, b), uint64x2_t::splat(32)));
//     simd_shuffle(r, x, [0, 1, 2, 3])
// }

// pub fn vaddhn_s16(a: int16x8_t, b: int16x8_t) -> int8x8_t {
//     simd_cast(simd_shr(simd_add(a, b), int16x8_t::splat(8)))
// }

// pub fn vaddhn_s32(a: int32x4_t, b: int32x4_t) -> int16x4_t {
//     simd_cast(simd_shr(simd_add(a, b), int32x4_t::splat(16)))
// }

// pub fn vaddhn_s64(a: int64x2_t, b: int64x2_t) -> int32x2_t {
//     simd_cast(simd_shr(simd_add(a, b), int64x2_t::splat(32)))
// }

// pub fn vaddhn_u16(a: uint16x8_t, b: uint16x8_t) -> uint8x8_t {
//     simd_cast(simd_shr(simd_add(a, b), uint16x8_t::splat(8)))
// }

// pub fn vaddhn_u32(a: uint32x4_t, b: uint32x4_t) -> uint16x4_t {
//     simd_cast(simd_shr(simd_add(a, b), uint32x4_t::splat(16)))
// }

// pub fn vaddhn_u64(a: uint64x2_t, b: uint64x2_t) -> uint32x2_t {
//     simd_cast(simd_shr(simd_add(a, b), uint64x2_t::splat(32)))
// }

// pub fn vaddl_high_s16(a: int16x8_t, b: int16x8_t) -> int32x4_t {
//     let a: int16x4_t = simd_shuffle(a, a, [4, 5, 6, 7]);
//     let b: int16x4_t = simd_shuffle(b, b, [4, 5, 6, 7]);
//     let a: int32x4_t = simd_cast(a);
//     let b: int32x4_t = simd_cast(b);
//     simd_add(a, b)
// }

// pub fn vaddl_high_s32(a: int32x4_t, b: int32x4_t) -> int64x2_t {
//     let a: int32x2_t = simd_shuffle(a, a, [2, 3]);
//     let b: int32x2_t = simd_shuffle(b, b, [2, 3]);
//     let a: int64x2_t = simd_cast(a);
//     let b: int64x2_t = simd_cast(b);
//     simd_add(a, b)
// }

// pub fn vaddl_high_s8(a: int8x16_t, b: int8x16_t) -> int16x8_t {
//     let a: int8x8_t = simd_shuffle(a, a, [8, 9, 10, 11, 12, 13, 14, 15]);
//     let b: int8x8_t = simd_shuffle(b, b, [8, 9, 10, 11, 12, 13, 14, 15]);
//     let a: int16x8_t = simd_cast(a);
//     let b: int16x8_t = simd_cast(b);
//     simd_add(a, b)
// }

// pub fn vaddl_high_u16(a: uint16x8_t, b: uint16x8_t) -> uint32x4_t {
//     let a: uint16x4_t = simd_shuffle(a, a, [4, 5, 6, 7]);
//     let b: uint16x4_t = simd_shuffle(b, b, [4, 5, 6, 7]);
//     let a: uint32x4_t = simd_cast(a);
//     let b: uint32x4_t = simd_cast(b);
//     simd_add(a, b)
// }

// pub fn vaddl_high_u32(a: uint32x4_t, b: uint32x4_t) -> uint64x2_t {
//     let a: uint32x2_t = simd_shuffle(a, a, [2, 3]);
//     let b: uint32x2_t = simd_shuffle(b, b, [2, 3]);
//     let a: uint64x2_t = simd_cast(a);
//     let b: uint64x2_t = simd_cast(b);
//     simd_add(a, b)
// }

// pub fn vaddl_high_u8(a: uint8x16_t, b: uint8x16_t) -> uint16x8_t {
//     let a: uint8x8_t = simd_shuffle(a, a, [8, 9, 10, 11, 12, 13, 14, 15]);
//     let b: uint8x8_t = simd_shuffle(b, b, [8, 9, 10, 11, 12, 13, 14, 15]);
//     let a: uint16x8_t = simd_cast(a);
//     let b: uint16x8_t = simd_cast(b);
//     simd_add(a, b)
// }

// pub fn vaddl_s16(a: int16x4_t, b: int16x4_t) -> int32x4_t {
//     let a: int32x4_t = simd_cast(a);
//     let b: int32x4_t = simd_cast(b);
//     simd_add(a, b)
// }

// pub fn vaddl_s32(a: int32x2_t, b: int32x2_t) -> int64x2_t {
//     let a: int64x2_t = simd_cast(a);
//     let b: int64x2_t = simd_cast(b);
//     simd_add(a, b)
// }

// pub fn vaddl_s8(a: int8x8_t, b: int8x8_t) -> int16x8_t {
//     let a: int16x8_t = simd_cast(a);
//     let b: int16x8_t = simd_cast(b);
//     simd_add(a, b)
// }

// pub fn vaddl_u16(a: uint16x4_t, b: uint16x4_t) -> uint32x4_t {
//     let a: uint32x4_t = simd_cast(a);
//     let b: uint32x4_t = simd_cast(b);
//     simd_add(a, b)
// }

// pub fn vaddl_u32(a: uint32x2_t, b: uint32x2_t) -> uint64x2_t {
//     let a: uint64x2_t = simd_cast(a);
//     let b: uint64x2_t = simd_cast(b);
//     simd_add(a, b)
// }

// pub fn vaddl_u8(a: uint8x8_t, b: uint8x8_t) -> uint16x8_t {
//     let a: uint16x8_t = simd_cast(a);
//     let b: uint16x8_t = simd_cast(b);
//     simd_add(a, b)
// }

// pub fn vaddw_high_s16(a: int32x4_t, b: int16x8_t) -> int32x4_t {
//     let b: int16x4_t = simd_shuffle(b, b, [4, 5, 6, 7]);
//     let b: int32x4_t = simd_cast(b);
//     simd_add(a, b)
// }

// pub fn vaddw_high_s32(a: int64x2_t, b: int32x4_t) -> int64x2_t {
//     let b: int32x2_t = simd_shuffle(b, b, [2, 3]);
//     let b: int64x2_t = simd_cast(b);
//     simd_add(a, b)
// }

// pub fn vaddw_high_s8(a: int16x8_t, b: int8x16_t) -> int16x8_t {
//     let b: int8x8_t = simd_shuffle(b, b, [8, 9, 10, 11, 12, 13, 14, 15]);
//     let b: int16x8_t = simd_cast(b);
//     simd_add(a, b)
// }

// pub fn vaddw_high_u16(a: uint32x4_t, b: uint16x8_t) -> uint32x4_t {
//     let b: uint16x4_t = simd_shuffle(b, b, [4, 5, 6, 7]);
//     let b: uint32x4_t = simd_cast(b);
//     simd_add(a, b)
// }

// pub fn vaddw_high_u32(a: uint64x2_t, b: uint32x4_t) -> uint64x2_t {
//     let b: uint32x2_t = simd_shuffle(b, b, [2, 3]);
//     let b: uint64x2_t = simd_cast(b);
//     simd_add(a, b)
// }

// pub fn vaddw_high_u8(a: uint16x8_t, b: uint8x16_t) -> uint16x8_t {
//     let b: uint8x8_t = simd_shuffle(b, b, [8, 9, 10, 11, 12, 13, 14, 15]);
//     let b: uint16x8_t = simd_cast(b);
//     simd_add(a, b)
// }

// pub fn vaddw_s16(a: int32x4_t, b: int16x4_t) -> int32x4_t {
//     let b: int32x4_t = simd_cast(b);
//     simd_add(a, b)
// }

// pub fn vaddw_s32(a: int64x2_t, b: int32x2_t) -> int64x2_t {
//     let b: int64x2_t = simd_cast(b);
//     simd_add(a, b)
// }

// pub fn vaddw_s8(a: int16x8_t, b: int8x8_t) -> int16x8_t {
//     let b: int16x8_t = simd_cast(b);
//     simd_add(a, b)
// }

// pub fn vaddw_u16(a: uint32x4_t, b: uint16x4_t) -> uint32x4_t {
//     let b: uint32x4_t = simd_cast(b);
//     simd_add(a, b)
// }

// pub fn vaddw_u32(a: uint64x2_t, b: uint32x2_t) -> uint64x2_t {
//     let b: uint64x2_t = simd_cast(b);
//     simd_add(a, b)
// }

// pub fn vaddw_u8(a: uint16x8_t, b: uint8x8_t) -> uint16x8_t {
//     let b: uint16x8_t = simd_cast(b);
//     simd_add(a, b)
// }

// pub fn vand_s8(a: int8x8_t, b: int8x8_t) -> int8x8_t {
//     simd_and(a, b)
// }

// pub fn vandq_s8(a: int8x16_t, b: int8x16_t) -> int8x16_t {
//     simd_and(a, b)
// }

// pub fn vand_s16(a: int16x4_t, b: int16x4_t) -> int16x4_t {
//     simd_and(a, b)
// }

// pub fn vandq_s16(a: int16x8_t, b: int16x8_t) -> int16x8_t {
//     simd_and(a, b)
// }

// pub fn vand_s32(a: int32x2_t, b: int32x2_t) -> int32x2_t {
//     simd_and(a, b)
// }

// pub fn vandq_s32(a: int32x4_t, b: int32x4_t) -> int32x4_t {
//     simd_and(a, b)
// }

// pub fn vand_s64(a: int64x1_t, b: int64x1_t) -> int64x1_t {
//     simd_and(a, b)
// }

// pub fn vandq_s64(a: int64x2_t, b: int64x2_t) -> int64x2_t {
//     simd_and(a, b)
// }

// pub fn vand_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
//     simd_and(a, b)
// }

// pub fn vandq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
//     simd_and(a, b)
// }

// pub fn vand_u16(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
//     simd_and(a, b)
// }

// pub fn vandq_u16(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
//     simd_and(a, b)
// }

// pub fn vand_u32(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
//     simd_and(a, b)
// }

// pub fn vandq_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
//     simd_and(a, b)
// }

// pub fn vand_u64(a: uint64x1_t, b: uint64x1_t) -> uint64x1_t {
//     simd_and(a, b)
// }

// pub fn vandq_u64(a: uint64x2_t, b: uint64x2_t) -> uint64x2_t {
//     simd_and(a, b)
// }

// pub fn vbic_s16(a: int16x4_t, b: int16x4_t) -> int16x4_t {
//     let c = int16x4_t::splat(-1);
//     simd_and(simd_xor(b, c), a)
// }

// pub fn vbic_s32(a: int32x2_t, b: int32x2_t) -> int32x2_t {
//     let c = int32x2_t::splat(-1);
//     simd_and(simd_xor(b, c), a)
// }

// pub fn vbic_s64(a: int64x1_t, b: int64x1_t) -> int64x1_t {
//     let c = int64x1_t::splat(-1);
//     simd_and(simd_xor(b, c), a)
// }

// pub fn vbic_s8(a: int8x8_t, b: int8x8_t) -> int8x8_t {
//     let c = int8x8_t::splat(-1);
//     simd_and(simd_xor(b, c), a)
// }

// pub fn vbicq_s16(a: int16x8_t, b: int16x8_t) -> int16x8_t {
//     let c = int16x8_t::splat(-1);
//     simd_and(simd_xor(b, c), a)
// }

// pub fn vbicq_s32(a: int32x4_t, b: int32x4_t) -> int32x4_t {
//     let c = int32x4_t::splat(-1);
//     simd_and(simd_xor(b, c), a)
// }

// pub fn vbicq_s64(a: int64x2_t, b: int64x2_t) -> int64x2_t {
//     let c = int64x2_t::splat(-1);
//     simd_and(simd_xor(b, c), a)
// }

// pub fn vbicq_s8(a: int8x16_t, b: int8x16_t) -> int8x16_t {
//     let c = int8x16_t::splat(-1);
//     simd_and(simd_xor(b, c), a)
// }

// pub fn vbic_u16(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
//     let c = int16x4_t::splat(-1);
//     simd_and(simd_xor(b, simd_cast(c)), a)
// }

// pub fn vbic_u32(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
//     let c = int32x2_t::splat(-1);
//     simd_and(simd_xor(b, simd_cast(c)), a)
// }

// pub fn vbic_u64(a: uint64x1_t, b: uint64x1_t) -> uint64x1_t {
//     let c = int64x1_t::splat(-1);
//     simd_and(simd_xor(b, simd_cast(c)), a)
// }

// pub fn vbic_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
//     let c = int8x8_t::splat(-1);
//     simd_and(simd_xor(b, simd_cast(c)), a)
// }

// pub fn vbicq_u16(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
//     let c = int16x8_t::splat(-1);
//     simd_and(simd_xor(b, simd_cast(c)), a)
// }

// pub fn vbicq_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
//     let c = int32x4_t::splat(-1);
//     simd_and(simd_xor(b, simd_cast(c)), a)
// }

// pub fn vbicq_u64(a: uint64x2_t, b: uint64x2_t) -> uint64x2_t {
//     let c = int64x2_t::splat(-1);
//     simd_and(simd_xor(b, simd_cast(c)), a)
// }

// pub fn vbicq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
//     let c = int8x16_t::splat(-1);
//     simd_and(simd_xor(b, simd_cast(c)), a)
// }

// pub fn vbsl_s16(a: uint16x4_t, b: int16x4_t, c: int16x4_t) -> int16x4_t {
//     let not = int16x4_t::splat(-1);
//     simd_cast(simd_or(
//         simd_and(a, simd_cast(b)),
//         simd_and(simd_xor(a, simd_cast(not)), simd_cast(c)),
//     ))
// }

// pub fn vbsl_s32(a: uint32x2_t, b: int32x2_t, c: int32x2_t) -> int32x2_t {
//     let not = int32x2_t::splat(-1);
//     simd_cast(simd_or(
//         simd_and(a, simd_cast(b)),
//         simd_and(simd_xor(a, simd_cast(not)), simd_cast(c)),
//     ))
// }

// pub fn vbsl_s64(a: uint64x1_t, b: int64x1_t, c: int64x1_t) -> int64x1_t {
//     let not = int64x1_t::splat(-1);
//     simd_cast(simd_or(
//         simd_and(a, simd_cast(b)),
//         simd_and(simd_xor(a, simd_cast(not)), simd_cast(c)),
//     ))
// }

// pub fn vbsl_s8(a: uint8x8_t, b: int8x8_t, c: int8x8_t) -> int8x8_t {
//     let not = int8x8_t::splat(-1);
//     simd_cast(simd_or(
//         simd_and(a, simd_cast(b)),
//         simd_and(simd_xor(a, simd_cast(not)), simd_cast(c)),
//     ))
// }

// pub fn vbslq_s16(a: uint16x8_t, b: int16x8_t, c: int16x8_t) -> int16x8_t {
//     let not = int16x8_t::splat(-1);
//     simd_cast(simd_or(
//         simd_and(a, simd_cast(b)),
//         simd_and(simd_xor(a, simd_cast(not)), simd_cast(c)),
//     ))
// }

// pub fn vbslq_s32(a: uint32x4_t, b: int32x4_t, c: int32x4_t) -> int32x4_t {
//     let not = int32x4_t::splat(-1);
//     simd_cast(simd_or(
//         simd_and(a, simd_cast(b)),
//         simd_and(simd_xor(a, simd_cast(not)), simd_cast(c)),
//     ))
// }

// pub fn vbslq_s64(a: uint64x2_t, b: int64x2_t, c: int64x2_t) -> int64x2_t {
//     let not = int64x2_t::splat(-1);
//     simd_cast(simd_or(
//         simd_and(a, simd_cast(b)),
//         simd_and(simd_xor(a, simd_cast(not)), simd_cast(c)),
//     ))
// }

// pub fn vbslq_s8(a: uint8x16_t, b: int8x16_t, c: int8x16_t) -> int8x16_t {
//     let not = int8x16_t::splat(-1);
//     simd_cast(simd_or(
//         simd_and(a, simd_cast(b)),
//         simd_and(simd_xor(a, simd_cast(not)), simd_cast(c)),
//     ))
// }

// pub fn vbsl_u16(a: uint16x4_t, b: uint16x4_t, c: uint16x4_t) -> uint16x4_t {
//     let not = int16x4_t::splat(-1);
//     simd_or(
//         simd_and(a, simd_cast(b)),
//         simd_and(simd_xor(a, simd_cast(not)), c),
//     )
// }

// pub fn vbsl_u32(a: uint32x2_t, b: uint32x2_t, c: uint32x2_t) -> uint32x2_t {
//     let not = int32x2_t::splat(-1);
//     simd_or(
//         simd_and(a, simd_cast(b)),
//         simd_and(simd_xor(a, simd_cast(not)), c),
//     )
// }

// pub fn vbsl_u64(a: uint64x1_t, b: uint64x1_t, c: uint64x1_t) -> uint64x1_t {
//     let not = int64x1_t::splat(-1);
//     simd_or(
//         simd_and(a, simd_cast(b)),
//         simd_and(simd_xor(a, simd_cast(not)), c),
//     )
// }

// pub fn vbsl_u8(a: uint8x8_t, b: uint8x8_t, c: uint8x8_t) -> uint8x8_t {
//     let not = int8x8_t::splat(-1);
//     simd_or(
//         simd_and(a, simd_cast(b)),
//         simd_and(simd_xor(a, simd_cast(not)), c),
//     )
// }

// pub fn vbslq_u16(a: uint16x8_t, b: uint16x8_t, c: uint16x8_t) -> uint16x8_t {
//     let not = int16x8_t::splat(-1);
//     simd_or(
//         simd_and(a, simd_cast(b)),
//         simd_and(simd_xor(a, simd_cast(not)), c),
//     )
// }

// pub fn vbslq_u32(a: uint32x4_t, b: uint32x4_t, c: uint32x4_t) -> uint32x4_t {
//     let not = int32x4_t::splat(-1);
//     simd_or(
//         simd_and(a, simd_cast(b)),
//         simd_and(simd_xor(a, simd_cast(not)), c),
//     )
// }

// pub fn vbslq_u64(a: uint64x2_t, b: uint64x2_t, c: uint64x2_t) -> uint64x2_t {
//     let not = int64x2_t::splat(-1);
//     simd_or(
//         simd_and(a, simd_cast(b)),
//         simd_and(simd_xor(a, simd_cast(not)), c),
//     )
// }

// pub fn vbslq_u8(a: uint8x16_t, b: uint8x16_t, c: uint8x16_t) -> uint8x16_t {
//     let not = int8x16_t::splat(-1);
//     simd_or(
//         simd_and(a, simd_cast(b)),
//         simd_and(simd_xor(a, simd_cast(not)), c),
//     )
// }

// pub fn vceq_s8(a: int8x8_t, b: int8x8_t) -> uint8x8_t {
//     simd_cast(simd_eq(a, b))
// }

// pub fn vceqq_s8(a: int8x16_t, b: int8x16_t) -> uint8x16_t {
//     simd_cast(simd_eq(a, b))
// }

// pub fn vceq_s16(a: int16x4_t, b: int16x4_t) -> uint16x4_t {
//     simd_cast(simd_eq(a, b))
// }

// pub fn vceqq_s16(a: int16x8_t, b: int16x8_t) -> uint16x8_t {
//     simd_cast(simd_eq(a, b))
// }

// pub fn vceq_s32(a: int32x2_t, b: int32x2_t) -> uint32x2_t {
//     simd_cast(simd_eq(a, b))
// }

// pub fn vceqq_s32(a: int32x4_t, b: int32x4_t) -> uint32x4_t {
//     simd_cast(simd_eq(a, b))
// }

// pub fn vceq_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
//     simd_eq(a, b)
// }

// pub fn vceqq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
//     simd_eq(a, b)
// }

// pub fn vceq_u16(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
//     simd_eq(a, b)
// }

// pub fn vceqq_u16(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
//     simd_eq(a, b)
// }

// pub fn vceq_u32(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
//     simd_eq(a, b)
// }

// pub fn vceqq_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
//     simd_eq(a, b)
// }

// pub fn vcge_s8(a: int8x8_t, b: int8x8_t) -> uint8x8_t {
//     simd_cast(simd_ge(a, b))
// }

// pub fn vcgeq_s8(a: int8x16_t, b: int8x16_t) -> uint8x16_t {
//     simd_cast(simd_ge(a, b))
// }

// pub fn vcge_s16(a: int16x4_t, b: int16x4_t) -> uint16x4_t {
//     simd_cast(simd_ge(a, b))
// }

// pub fn vcgeq_s16(a: int16x8_t, b: int16x8_t) -> uint16x8_t {
//     simd_cast(simd_ge(a, b))
// }

// pub fn vcge_s32(a: int32x2_t, b: int32x2_t) -> uint32x2_t {
//     simd_cast(simd_ge(a, b))
// }

// pub fn vcgeq_s32(a: int32x4_t, b: int32x4_t) -> uint32x4_t {
//     simd_cast(simd_ge(a, b))
// }

// pub fn vcge_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
//     simd_ge(a, b)
// }

// pub fn vcgeq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
//     simd_ge(a, b)
// }

// pub fn vcge_u16(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
//     simd_ge(a, b)
// }

// pub fn vcgeq_u16(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
//     simd_ge(a, b)
// }

// pub fn vcge_u32(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
//     simd_ge(a, b)
// }

// pub fn vcgeq_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
//     simd_ge(a, b)
// }

// pub fn vcgt_s8(a: int8x8_t, b: int8x8_t) -> uint8x8_t {
//     simd_cast(simd_gt(a, b))
// }

// pub fn vcgtq_s8(a: int8x16_t, b: int8x16_t) -> uint8x16_t {
//     simd_cast(simd_gt(a, b))
// }

// pub fn vcgt_s16(a: int16x4_t, b: int16x4_t) -> uint16x4_t {
//     simd_cast(simd_gt(a, b))
// }

// pub fn vcgtq_s16(a: int16x8_t, b: int16x8_t) -> uint16x8_t {
//     simd_cast(simd_gt(a, b))
// }

// pub fn vcgt_s32(a: int32x2_t, b: int32x2_t) -> uint32x2_t {
//     simd_cast(simd_gt(a, b))
// }

// pub fn vcgtq_s32(a: int32x4_t, b: int32x4_t) -> uint32x4_t {
//     simd_cast(simd_gt(a, b))
// }

// pub fn vcgt_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
//     simd_gt(a, b)
// }

// pub fn vcgtq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
//     simd_gt(a, b)
// }

// pub fn vcgt_u16(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
//     simd_gt(a, b)
// }

// pub fn vcgtq_u16(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
//     simd_gt(a, b)
// }

// pub fn vcgt_u32(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
//     simd_gt(a, b)
// }

// pub fn vcgtq_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
//     simd_gt(a, b)
// }

// pub fn vcle_s8(a: int8x8_t, b: int8x8_t) -> uint8x8_t {
//     simd_cast(simd_le(a, b))
// }

// pub fn vcleq_s8(a: int8x16_t, b: int8x16_t) -> uint8x16_t {
//     simd_cast(simd_le(a, b))
// }

// pub fn vcle_s16(a: int16x4_t, b: int16x4_t) -> uint16x4_t {
//     simd_cast(simd_le(a, b))
// }

// pub fn vcleq_s16(a: int16x8_t, b: int16x8_t) -> uint16x8_t {
//     simd_cast(simd_le(a, b))
// }

// pub fn vcle_s32(a: int32x2_t, b: int32x2_t) -> uint32x2_t {
//     simd_cast(simd_le(a, b))
// }

// pub fn vcleq_s32(a: int32x4_t, b: int32x4_t) -> uint32x4_t {
//     simd_cast(simd_le(a, b))
// }

// pub fn vcle_u8(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
//     simd_le(a, b)
// }

// pub fn vcleq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
//     simd_le(a, b)
// }

// pub fn vcle_u16(a: uint16x4_t, b: uint16x4_t) -> uint16x4_t {
//     simd_le(a, b)
// }

// pub fn vcleq_u16(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
//     simd_le(a, b)
// }

// pub fn vcle_u32(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
//     simd_le(a, b)
// }

// pub fn vcleq_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
//     simd_le(a, b)
// }
