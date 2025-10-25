use once_cell::sync::Lazy;

const LOG10_2_F64: f64 = std::f64::consts::LOG10_2;
const LOG10_2_F32: f32 = std::f32::consts::LOG10_2;

const POW10_MIN_F64: i32 = -324;
const POW10_MAX_F64: i32 = 308;
const POW10_RANGE_F64: usize = (POW10_MAX_F64 - POW10_MIN_F64 + 1) as usize;

const POW10_MIN_F32: i32 = -45;
const POW10_MAX_F32: i32 = 38;
const POW10_RANGE_F32: usize = (POW10_MAX_F32 - POW10_MIN_F32 + 1) as usize;

const MAX_EXP10_ABS_F64: usize = POW10_MIN_F64.abs() as usize;
const MAX_EXP10_ABS_F32: usize = POW10_MIN_F32.abs() as usize;

static EXP10_DIGITS_F64: Lazy<[u32; MAX_EXP10_ABS_F64 + 1]> = Lazy::new(|| {
    let mut arr = [0u32; MAX_EXP10_ABS_F64 + 1];
    for i in 0..=MAX_EXP10_ABS_F64 {
        arr[i] = if i == 0 {
            1
        } else {
            (i as f64).log10().floor() as u32 + 1
        };
    }
    arr
});

static EXP10_DIGITS_F32: Lazy<[u32; MAX_EXP10_ABS_F32 + 1]> = Lazy::new(|| {
    let mut arr = [0u32; MAX_EXP10_ABS_F32 + 1];
    for i in 0..=MAX_EXP10_ABS_F32 {
        arr[i] = if i == 0 {
            1
        } else {
            (i as f64).log10().floor() as u32 + 1
        };
    }
    arr
});

static EXP2_TO_EXP10_F64: Lazy<[i32; 2047]> = Lazy::new(|| {
    let mut arr = [0i32; 2047];
    for i in 0..2047 {
        let exp2 = i as i32 - 1023;
        arr[i] = ((exp2 as f64) * LOG10_2_F64).round() as i32;
    }
    arr
});

static EXP2_TO_EXP10_F32: Lazy<[i32; 255]> = Lazy::new(|| {
    let mut arr = [0i32; 255];
    for i in 0..255 {
        let exp2 = i as i32 - 127;
        arr[i] = ((exp2 as f32) * LOG10_2_F32).round() as i32;
    }
    arr
});

static POW10_F64: Lazy<[f64; POW10_RANGE_F64]> = Lazy::new(|| {
    let mut arr = [0f64; POW10_RANGE_F64];
    for (i, e) in (POW10_MIN_F64..=POW10_MAX_F64).enumerate() {
        arr[i] = 10f64.powi(e);
    }
    arr
});

static POW10_F32: Lazy<[f32; POW10_RANGE_F32]> = Lazy::new(|| {
    let mut arr = [0f32; POW10_RANGE_F32];
    for (i, e) in (POW10_MIN_F32..=POW10_MAX_F32).enumerate() {
        arr[i] = 10f32.powi(e);
    }
    arr
});

fn exponent_digit_count_f64(e10: i32) -> u32 {
    EXP10_DIGITS_F64[e10.abs() as usize]
}

fn exponent_digit_count_f32(e10: i32) -> u32 {
    EXP10_DIGITS_F32[e10.abs() as usize]
}

#[cold]
#[inline(never)]
fn subnormal_exponent_f64(bits: u64) -> i32 {
    let mantissa = bits & ((1u64 << 52) - 1);
    debug_assert!(mantissa != 0);
    let true_exp2 = -1074 + (mantissa.ilog2() as i32);
    ((true_exp2 as f64) * LOG10_2_F64).round() as i32
}

#[cold]
#[inline(never)]
fn subnormal_exponent_f32(bits: u32) -> i32 {
    let mantissa = bits & ((1u32 << 23) - 1);
    debug_assert!(mantissa != 0);
    let true_exp2 = -149 + (mantissa.ilog2() as i32);
    ((true_exp2 as f32) * LOG10_2_F32).round() as i32
}

#[must_use]
pub fn decimal_exponent_f64(x: f64) -> i32 {
    let ax = x.abs();
    if ax == 0.0 || !ax.is_finite() {
        return 0;
    }
    let bits = ax.to_bits();
    let exp_bits = ((bits >> 52) & 0x7ff) as usize;

    let mut e10 = if exp_bits == 0 {
        subnormal_exponent_f64(bits)
    } else {
        EXP2_TO_EXP10_F64[exp_bits]
    };

    if (POW10_MIN_F64..=POW10_MAX_F64).contains(&e10) {
        let idx = (e10 - POW10_MIN_F64) as usize;
        if ax < POW10_F64[idx] {
            e10 -= 1;
        }
    }
    e10
}

#[must_use]
pub fn decimal_exponent_f32(x: f32) -> i32 {
    let ax = x.abs();
    if ax == 0.0 || !ax.is_finite() {
        return 0;
    }
    let bits = ax.to_bits();
    let exp_bits = ((bits >> 23) & 0xff) as usize;

    let mut e10 = if exp_bits == 0 {
        subnormal_exponent_f32(bits)
    } else {
        EXP2_TO_EXP10_F32[exp_bits]
    };

    if (POW10_MIN_F32..=POW10_MAX_F32).contains(&e10) {
        let idx = (e10 - POW10_MIN_F32) as usize;
        if ax < POW10_F32[idx] {
            e10 -= 1;
        }
    }
    e10
}

#[must_use]
pub fn decimal_exponent_digit_count_f64(x: f64) -> u32 {
    let ax = x.abs();
    if ax == 0.0 || !ax.is_finite() {
        return 0;
    }
    exponent_digit_count_f64(decimal_exponent_f64(ax))
}

#[must_use]
pub fn decimal_exponent_digit_count_f32(x: f32) -> u32 {
    let ax = x.abs();
    if ax == 0.0 || !ax.is_finite() {
        return 0;
    }
    exponent_digit_count_f32(decimal_exponent_f32(ax))
}

#[must_use]
pub fn decimal_exponent_and_digit_count_f64(x: f64) -> (i32, u32) {
    let ax = x.abs();
    if ax == 0.0 || !ax.is_finite() {
        return (0, 0);
    }
    let e10 = decimal_exponent_f64(ax);
    let digits = exponent_digit_count_f64(e10);
    (e10, digits)
}

#[must_use]
pub fn decimal_exponent_and_digit_count_f32(x: f32) -> (i32, u32) {
    let ax = x.abs();
    if ax == 0.0 || !ax.is_finite() {
        return (0, 0);
    }
    let e10 = decimal_exponent_f32(ax);
    let digits = exponent_digit_count_f32(e10);
    (e10, digits)
}

#[cfg(test)]
mod tests {
    use core::f64;

    use super::*;

    #[test]
    fn test_exp() {
        // 正の正規化数
        assert_eq!(decimal_exponent_f64(1.0), 0);
        assert_eq!(decimal_exponent_f64(9.9), 0);
        assert_eq!(decimal_exponent_f64(10.0), 1);
        assert_eq!(decimal_exponent_f64(999.0), 2);
        assert_eq!(decimal_exponent_f64(1e6), 6);

        // 符号ありでも絶対値で処理される
        assert_eq!(decimal_exponent_f64(-1e6), 6);

        // サブノーマル(最小正のサブノーマル)
        let min_sub_f64 = f64::from_bits(1);
        assert_eq!(decimal_exponent_f64(min_sub_f64), -324);

        // 最小正の正規化数
        assert_eq!(decimal_exponent_f64(f64::MIN_POSITIVE), -308);

        // 非有限
        assert_eq!(decimal_exponent_f64(f64::INFINITY), 0);
        assert_eq!(decimal_exponent_f64(f64::NEG_INFINITY), 0);
        assert_eq!(decimal_exponent_f64(f64::NAN), 0);

        // f32
        assert_eq!(decimal_exponent_f32(1.0), 0);
        assert_eq!(decimal_exponent_f32(9.9), 0);
        assert_eq!(decimal_exponent_f32(10.0), 1);
        assert_eq!(decimal_exponent_f32(1e6), 6);

        // サブノーマル(最小正のサブノーマル)
        let min_sub_f32 = f32::from_bits(1);
        assert_eq!(decimal_exponent_f32(min_sub_f32), -45);

        // 最小正の正規化数
        assert_eq!(decimal_exponent_f32(f32::MIN_POSITIVE), -38);
    }

    #[test]
    fn test_exponent_digit_count() {
        assert_eq!(decimal_exponent_digit_count_f64(1.0), 1);
        assert_eq!(decimal_exponent_digit_count_f64(1e10), 2);
        assert_eq!(decimal_exponent_digit_count_f64(1e-200), 3);

        assert_eq!(decimal_exponent_digit_count_f32(1.0), 1);
        assert_eq!(decimal_exponent_digit_count_f32(1e10), 2);
        assert_eq!(decimal_exponent_digit_count_f32(1e-20), 2);
    }

    #[test]
    fn test_exponent_and_digit_count() {
        assert_eq!(decimal_exponent_and_digit_count_f64(1.0), (0, 1));
        assert_eq!(decimal_exponent_and_digit_count_f64(1e10), (10, 2));
        assert_eq!(decimal_exponent_and_digit_count_f64(1e-200), (-200, 3));

        assert_eq!(decimal_exponent_and_digit_count_f32(1.0), (0, 1));
        assert_eq!(decimal_exponent_and_digit_count_f32(1e10), (10, 2));
        assert_eq!(decimal_exponent_and_digit_count_f32(1e-20), (-20, 2));
    }
}
