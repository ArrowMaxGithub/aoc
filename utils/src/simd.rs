use std::simd::cmp::SimdPartialEq;
use std::simd::{LaneCount, Mask, MaskElement, Simd, SimdElement, SupportedLaneCount};

/// Test each element of `simd` against the provided `test` value and return the matched difference count.
/// The count of non-matching elements is subtracted from the count of matching elements.
pub fn simd_eq_diff_count<T, U, const N: usize>(simd: Simd<T, N>, test: T) -> i32
where
    T: SimdElement,
    U: MaskElement,
    LaneCount<N>: SupportedLaneCount,
    Simd<T, N>: SimdPartialEq<Mask = Mask<U, N>>,
{
    let test = simd.simd_eq(std::simd::Simd::splat(test)).to_bitmask();
    let ones = test.count_ones() as i32;
    let zeros = 64 - ones;
    ones - zeros
}

/// Test each element of `simd` against the provided `test` value and return the count of matches, ignoring non-matched elements.
pub fn simd_eq_count<T, const N: usize>(simd: Simd<T, N>, test: T) -> u32
where
    T: SimdElement + MaskElement,
    LaneCount<N>: SupportedLaneCount,
    Simd<T, N>: SimdPartialEq<Mask = Mask<T, N>>,
{
    let test = simd.simd_eq(std::simd::Simd::splat(test)).to_bitmask();
    test.count_ones()
}
