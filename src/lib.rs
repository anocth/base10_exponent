use num_traits::Float as NumFloat;
use once_cell::sync::Lazy;
use std::any::TypeId;

// Trait defining common operations for floating-point types
pub trait FloatOps: PartialEq + Copy {
    type Bits;

    const LOG10_2: Self;
    const POW10_MIN: i32;
    const POW10_MAX: i32;
    const POW10_RANGE: usize;
    const MAX_EXP10_ABS: usize;
    const EXP_BITS_MASK: usize;
    const MANTISSA_MASK: Self::Bits;
    const EXP_BIAS: i32;
    const SUBNORMAL_EXP_OFFSET: i32;
    const MANTISSA_BITS: u32;

    fn to_bits(self) -> Self::Bits;
    fn from_bits(bits: Self::Bits) -> Self;
    fn powi(self, exp: i32) -> Self;
    fn abs(self) -> Self;
    fn is_finite(self) -> bool;
    fn ilog2(self) -> i32;
    fn bits_ilog2(bits: Self::Bits) -> u32;
    fn default_bits() -> Self::Bits;
}

// Implementation for f64
impl FloatOps for f64 {
    type Bits = u64;

    const LOG10_2: f64 = std::f64::consts::LOG10_2;
    const POW10_MIN: i32 = -324;
    const POW10_MAX: i32 = 308;
    const POW10_RANGE: usize = (Self::POW10_MAX - Self::POW10_MIN + 1) as usize;
    const MAX_EXP10_ABS: usize = Self::POW10_MIN.abs() as usize;
    const EXP_BITS_MASK: usize = 0x7ff;
    const MANTISSA_MASK: u64 = (1u64 << 52) - 1;
    const EXP_BIAS: i32 = 1023;
    const SUBNORMAL_EXP_OFFSET: i32 = -1074;
    const MANTISSA_BITS: u32 = 52;

    fn to_bits(self) -> u64 {
        self.to_bits()
    }
    fn from_bits(bits: u64) -> f64 {
        f64::from_bits(bits)
    }
    fn powi(self, exp: i32) -> f64 {
        self.powi(exp)
    }
    fn abs(self) -> f64 {
        self.abs()
    }
    fn is_finite(self) -> bool {
        self.is_finite()
    }
    fn ilog2(self) -> i32 {
        NumFloat::log2(self).floor() as i32
    }
    fn bits_ilog2(bits: u64) -> u32 {
        bits.ilog2()
    }
    fn default_bits() -> u64 {
        0
    }
}

// Implementation for f32
impl FloatOps for f32 {
    type Bits = u32;

    const LOG10_2: f32 = std::f32::consts::LOG10_2;
    const POW10_MIN: i32 = -45;
    const POW10_MAX: i32 = 38;
    const POW10_RANGE: usize = (Self::POW10_MAX - Self::POW10_MIN + 1) as usize;
    const MAX_EXP10_ABS: usize = Self::POW10_MIN.abs() as usize;
    const EXP_BITS_MASK: usize = 0xff;
    const MANTISSA_MASK: u32 = (1u32 << 23) - 1;
    const EXP_BIAS: i32 = 127;
    const SUBNORMAL_EXP_OFFSET: i32 = -149;
    const MANTISSA_BITS: u32 = 23;

    fn to_bits(self) -> u32 {
        self.to_bits()
    }
    fn from_bits(bits: u32) -> f32 {
        f32::from_bits(bits)
    }
    fn powi(self, exp: i32) -> f32 {
        self.powi(exp)
    }
    fn abs(self) -> f32 {
        self.abs()
    }
    fn is_finite(self) -> bool {
        self.is_finite()
    }
    fn ilog2(self) -> i32 {
        NumFloat::log2(self).floor() as i32
    }
    fn bits_ilog2(bits: u32) -> u32 {
        bits.ilog2()
    }
    fn default_bits() -> u32 {
        0
    }
}

static EXP10_DIGITS_F64: Lazy<[u32; <f64 as FloatOps>::MAX_EXP10_ABS + 1]> = Lazy::new(|| {
    let mut arr = [0u32; <f64 as FloatOps>::MAX_EXP10_ABS + 1];
    for i in 0..=<f64 as FloatOps>::MAX_EXP10_ABS {
        arr[i] = if i == 0 {
            1
        } else {
            (i as f64).log10().floor() as u32 + 1
        };
    }
    arr
});

static EXP10_DIGITS_F32: Lazy<[u32; <f32 as FloatOps>::MAX_EXP10_ABS + 1]> = Lazy::new(|| {
    let mut arr = [0u32; <f32 as FloatOps>::MAX_EXP10_ABS + 1];
    for i in 0..=<f32 as FloatOps>::MAX_EXP10_ABS {
        arr[i] = if i == 0 {
            1
        } else {
            (i as f64).log10().floor() as u32 + 1
        };
    }
    arr
});

static EXP2_TO_EXP10_F64: Lazy<[i32; <f64 as FloatOps>::EXP_BITS_MASK]> = Lazy::new(|| {
    let mut arr = [0i32; <f64 as FloatOps>::EXP_BITS_MASK];
    for i in 0..<f64 as FloatOps>::EXP_BITS_MASK {
        let exp2 = i as i32 - <f64 as FloatOps>::EXP_BIAS;
        arr[i] = ((exp2 as f64) * <f64 as FloatOps>::LOG10_2).round() as i32;
    }
    arr
});

static EXP2_TO_EXP10_F32: Lazy<[i32; <f32 as FloatOps>::EXP_BITS_MASK]> = Lazy::new(|| {
    let mut arr = [0i32; <f32 as FloatOps>::EXP_BITS_MASK];
    for i in 0..<f32 as FloatOps>::EXP_BITS_MASK {
        let exp2 = i as i32 - <f32 as FloatOps>::EXP_BIAS;
        arr[i] = ((exp2 as f32) * <f32 as FloatOps>::LOG10_2).round() as i32;
    }
    arr
});

static POW10_F64: Lazy<[f64; <f64 as FloatOps>::POW10_RANGE]> = Lazy::new(|| {
    let mut arr = [0f64; <f64 as FloatOps>::POW10_RANGE];
    for (i, e) in (<f64 as FloatOps>::POW10_MIN..=<f64 as FloatOps>::POW10_MAX).enumerate() {
        arr[i] = 10f64.powi(e);
    }
    arr
});

static POW10_F32: Lazy<[f32; <f32 as FloatOps>::POW10_RANGE]> = Lazy::new(|| {
    let mut arr = [0f32; <f32 as FloatOps>::POW10_RANGE];
    for (i, e) in (<f32 as FloatOps>::POW10_MIN..=<f32 as FloatOps>::POW10_MAX).enumerate() {
        arr[i] = 10f32.powi(e);
    }
    arr
});

fn exp10_digit_table<F: FloatOps + 'static>() -> &'static [u32] {
    if TypeId::of::<F>() == TypeId::of::<f64>() {
        &*EXP10_DIGITS_F64
    } else {
        &*EXP10_DIGITS_F32
    }
}

fn exponent_digit_count<F: FloatOps + 'static>(e10: i32) -> u32 {
    exp10_digit_table::<F>()[e10.abs() as usize]
}

#[cold]
#[inline(never)]
fn subnormal_exponent_f64(bits: u64) -> i32 {
    let mantissa = bits & <f64 as FloatOps>::MANTISSA_MASK;
    debug_assert!(mantissa != 0);
    let true_exp2 = <f64 as FloatOps>::SUBNORMAL_EXP_OFFSET + mantissa.ilog2() as i32;
    ((true_exp2 as f64) * <f64 as FloatOps>::LOG10_2).round() as i32
}

#[cold]
#[inline(never)]
fn subnormal_exponent_f32(bits: u32) -> i32 {
    let mantissa = bits & <f32 as FloatOps>::MANTISSA_MASK;
    debug_assert!(mantissa != 0);
    let true_exp2 = <f32 as FloatOps>::SUBNORMAL_EXP_OFFSET + mantissa.ilog2() as i32;
    ((true_exp2 as f32) * <f32 as FloatOps>::LOG10_2).round() as i32
}

fn exp10_f64(x: f64) -> i32 {
    let ax = x.abs();
    if ax == 0.0 || !ax.is_finite() {
        return 0;
    }
    let bits = ax.to_bits();
    let exp_bits = ((bits >> <f64 as FloatOps>::MANTISSA_BITS)
        & (<f64 as FloatOps>::EXP_BITS_MASK as u64)) as usize;

    let mut e10 = if exp_bits == 0 {
        subnormal_exponent_f64(bits)
    } else {
        EXP2_TO_EXP10_F64[exp_bits]
    };

    if (<f64 as FloatOps>::POW10_MIN..=<f64 as FloatOps>::POW10_MAX).contains(&e10) {
        let idx = (e10 - <f64 as FloatOps>::POW10_MIN) as usize;
        if ax < POW10_F64[idx] {
            e10 -= 1;
        }
    }
    e10
}

fn exp10_f32(x: f32) -> i32 {
    let ax = x.abs();
    if ax == 0.0 || !ax.is_finite() {
        return 0;
    }
    let bits = ax.to_bits();
    let exp_bits = ((bits >> <f32 as FloatOps>::MANTISSA_BITS)
        & (<f32 as FloatOps>::EXP_BITS_MASK as u32)) as usize;

    let mut e10 = if exp_bits == 0 {
        subnormal_exponent_f32(bits)
    } else {
        EXP2_TO_EXP10_F32[exp_bits]
    };

    if (<f32 as FloatOps>::POW10_MIN..=<f32 as FloatOps>::POW10_MAX).contains(&e10) {
        let idx = (e10 - <f32 as FloatOps>::POW10_MIN) as usize;
        if ax < POW10_F32[idx] {
            e10 -= 1;
        }
    }
    e10
}

#[must_use]
pub fn exp10<F: FloatOps + 'static>(x: F) -> i32 {
    if TypeId::of::<F>() == TypeId::of::<f64>() {
        let x_f64 = unsafe { std::mem::transmute_copy::<F, f64>(&x) };
        exp10_f64(x_f64)
    } else {
        let x_f32 = unsafe { std::mem::transmute_copy::<F, f32>(&x) };
        exp10_f32(x_f32)
    }
}

#[must_use]
pub fn exp10_digits<F: FloatOps + 'static>(x: F) -> u32 {
    let ax = x.abs();
    if ax == F::from_bits(F::default_bits()) || !ax.is_finite() {
        return 0;
    }
    exponent_digit_count::<F>(exp10::<F>(ax))
}

#[must_use]
pub fn exp10_with_digits<F: FloatOps + 'static>(x: F) -> (i32, u32) {
    let ax = x.abs();
    if ax == F::from_bits(F::default_bits()) || !ax.is_finite() {
        return (0, 0);
    }
    let e10 = exp10::<F>(ax);
    let digits = exponent_digit_count::<F>(e10);
    (e10, digits)
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========================================
    // Tests for exp10 function (f64)
    // ========================================

    #[test]
    fn test_exp10_f64_normalized_numbers() {
        // Basic cases for normalized numbers
        assert_eq!(exp10::<f64>(1.0), 0);
        assert_eq!(exp10::<f64>(9.9), 0);
        assert_eq!(exp10::<f64>(10.0), 1);
        assert_eq!(exp10::<f64>(99.9), 1);
        assert_eq!(exp10::<f64>(100.0), 2);
        assert_eq!(exp10::<f64>(999.0), 2);
        assert_eq!(exp10::<f64>(1000.0), 3);
        assert_eq!(exp10::<f64>(1e6), 6);
        assert_eq!(exp10::<f64>(1e100), 100);
    }

    #[test]
    fn test_exp10_f64_negative_numbers() {
        // Negative numbers are processed by their absolute value
        assert_eq!(exp10::<f64>(-1.0), 0);
        assert_eq!(exp10::<f64>(-10.0), 1);
        assert_eq!(exp10::<f64>(-1e6), 6);
        assert_eq!(exp10::<f64>(-1e100), 100);
    }

    #[test]
    fn test_exp10_f64_small_numbers() {
        // Small numbers (< 1.0)
        assert_eq!(exp10::<f64>(0.1), -1);
        assert_eq!(exp10::<f64>(0.01), -2);
        assert_eq!(exp10::<f64>(1e-10), -10);
        assert_eq!(exp10::<f64>(1e-100), -100);
    }

    #[test]
    fn test_exp10_f64_subnormal_numbers() {
        // Subnormal numbers
        let min_subnormal = f64::from_bits(1); // Minimum positive subnormal
        assert_eq!(exp10::<f64>(min_subnormal), -324);

        // Other subnormal numbers
        let subnormal = f64::from_bits(100);
        assert!(exp10::<f64>(subnormal) <= -308);
    }

    #[test]
    fn test_exp10_f64_boundary_values() {
        // Boundary values
        assert_eq!(exp10::<f64>(f64::MIN_POSITIVE), -308); // Minimum positive normalized number
        assert_eq!(exp10::<f64>(f64::MAX), 308); // Maximum value
    }

    #[test]
    fn test_exp10_f64_special_values() {
        // Special values
        assert_eq!(exp10::<f64>(0.0), 0);
        assert_eq!(exp10::<f64>(-0.0), 0);
        assert_eq!(exp10::<f64>(f64::INFINITY), 0);
        assert_eq!(exp10::<f64>(f64::NEG_INFINITY), 0);
        assert_eq!(exp10::<f64>(f64::NAN), 0);
    }

    // ========================================
    // Tests for exp10 function (f32)
    // ========================================

    #[test]
    fn test_exp10_f32_normalized_numbers() {
        // Basic cases for normalized numbers
        assert_eq!(exp10::<f32>(1.0), 0);
        assert_eq!(exp10::<f32>(9.9), 0);
        assert_eq!(exp10::<f32>(10.0), 1);
        assert_eq!(exp10::<f32>(99.9), 1);
        assert_eq!(exp10::<f32>(100.0), 2);
        assert_eq!(exp10::<f32>(1e6), 6);
        assert_eq!(exp10::<f32>(1e20), 20);
    }

    #[test]
    fn test_exp10_f32_negative_numbers() {
        // Negative numbers are processed by their absolute value
        assert_eq!(exp10::<f32>(-1.0), 0);
        assert_eq!(exp10::<f32>(-10.0), 1);
        assert_eq!(exp10::<f32>(-1e6), 6);
    }

    #[test]
    fn test_exp10_f32_small_numbers() {
        // Small numbers (< 1.0)
        assert_eq!(exp10::<f32>(0.1), -1);
        assert_eq!(exp10::<f32>(0.01), -2);
        assert_eq!(exp10::<f32>(1e-10), -10);
    }

    #[test]
    fn test_exp10_f32_subnormal_numbers() {
        // Subnormal numbers
        let min_subnormal = f32::from_bits(1); // Minimum positive subnormal
        assert_eq!(exp10::<f32>(min_subnormal), -45);

        // Other subnormal numbers
        let subnormal = f32::from_bits(100);
        assert!(exp10::<f32>(subnormal) <= -38);
    }

    #[test]
    fn test_exp10_f32_boundary_values() {
        // Boundary values
        assert_eq!(exp10::<f32>(f32::MIN_POSITIVE), -38); // Minimum positive normalized number
        assert_eq!(exp10::<f32>(f32::MAX), 38); // Maximum value
    }

    #[test]
    fn test_exp10_f32_special_values() {
        // Special values
        assert_eq!(exp10::<f32>(0.0), 0);
        assert_eq!(exp10::<f32>(-0.0), 0);
        assert_eq!(exp10::<f32>(f32::INFINITY), 0);
        assert_eq!(exp10::<f32>(f32::NEG_INFINITY), 0);
        assert_eq!(exp10::<f32>(f32::NAN), 0);
    }

    // ========================================
    // Tests for exp10_digits function
    // ========================================

    #[test]
    fn test_exp10_digits_f64() {
        // Basic cases
        assert_eq!(exp10_digits::<f64>(1.0), 1);
        assert_eq!(exp10_digits::<f64>(10.0), 1);
        assert_eq!(exp10_digits::<f64>(100.0), 1);
        assert_eq!(exp10_digits::<f64>(1e10), 2);
        assert_eq!(exp10_digits::<f64>(1e99), 2);
        assert_eq!(exp10_digits::<f64>(1e100), 3);
        assert_eq!(exp10_digits::<f64>(1e-9), 1);
        assert_eq!(exp10_digits::<f64>(1e-10), 2);
        assert_eq!(exp10_digits::<f64>(1e-200), 3);
    }

    #[test]
    fn test_exp10_digits_f32() {
        // Basic cases
        assert_eq!(exp10_digits::<f32>(1.0), 1);
        assert_eq!(exp10_digits::<f32>(10.0), 1);
        assert_eq!(exp10_digits::<f32>(1e10), 2);
        assert_eq!(exp10_digits::<f32>(1e-9), 1);
        assert_eq!(exp10_digits::<f32>(1e-10), 2);
        assert_eq!(exp10_digits::<f32>(1e-20), 2);
    }

    #[test]
    fn test_exp10_digits_special_values() {
        // Special values
        assert_eq!(exp10_digits::<f64>(0.0), 0);
        assert_eq!(exp10_digits::<f64>(f64::INFINITY), 0);
        assert_eq!(exp10_digits::<f64>(f64::NAN), 0);

        assert_eq!(exp10_digits::<f32>(0.0), 0);
        assert_eq!(exp10_digits::<f32>(f32::INFINITY), 0);
        assert_eq!(exp10_digits::<f32>(f32::NAN), 0);
    }

    // ========================================
    // Tests for exp10_with_digits function
    // ========================================

    #[test]
    fn test_exp10_with_digits_f64() {
        // Basic cases
        assert_eq!(exp10_with_digits::<f64>(1.0), (0, 1));
        assert_eq!(exp10_with_digits::<f64>(10.0), (1, 1));
        assert_eq!(exp10_with_digits::<f64>(1e10), (10, 2));
        assert_eq!(exp10_with_digits::<f64>(1e100), (100, 3));
        assert_eq!(exp10_with_digits::<f64>(1e-10), (-10, 2));
        assert_eq!(exp10_with_digits::<f64>(1e-200), (-200, 3));
    }

    #[test]
    fn test_exp10_with_digits_f32() {
        // Basic cases
        assert_eq!(exp10_with_digits::<f32>(1.0), (0, 1));
        assert_eq!(exp10_with_digits::<f32>(10.0), (1, 1));
        assert_eq!(exp10_with_digits::<f32>(1e10), (10, 2));
        assert_eq!(exp10_with_digits::<f32>(1e-10), (-10, 2));
        assert_eq!(exp10_with_digits::<f32>(1e-20), (-20, 2));
    }

    #[test]
    fn test_exp10_with_digits_special_values() {
        // Special values
        assert_eq!(exp10_with_digits::<f64>(0.0), (0, 0));
        assert_eq!(exp10_with_digits::<f64>(f64::INFINITY), (0, 0));
        assert_eq!(exp10_with_digits::<f64>(f64::NAN), (0, 0));

        assert_eq!(exp10_with_digits::<f32>(0.0), (0, 0));
        assert_eq!(exp10_with_digits::<f32>(f32::INFINITY), (0, 0));
        assert_eq!(exp10_with_digits::<f32>(f32::NAN), (0, 0));
    }
}
