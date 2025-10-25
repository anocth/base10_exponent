use criterion::{Criterion, black_box, criterion_group, criterion_main};

use base10_exponent::exp10;

fn standard_exp10_f64(x: f64) -> i32 {
    if x == 0.0 || !x.is_finite() {
        0
    } else {
        x.abs().log10().floor() as i32
    }
}

fn standard_exp10_f32(x: f32) -> i32 {
    if x == 0.0 || !x.is_finite() {
        0
    } else {
        x.abs().log10().floor() as i32
    }
}

fn bench_exp10_f64(c: &mut Criterion) {
    let inputs = [1.0, 10.0, 100.0, 1e10, 1e100, 0.1, 0.01, 1e-10, 1e-100];
    c.bench_function("base10_exponent::exp10<f64>", |b| {
        b.iter(|| {
            for &input in &inputs {
                black_box(exp10::<f64>(input));
            }
        })
    });
}

fn bench_standard_exp10_f64(c: &mut Criterion) {
    let inputs = [1.0, 10.0, 100.0, 1e10, 1e100, 0.1, 0.01, 1e-10, 1e-100];
    c.bench_function("standard log10<f64>", |b| {
        b.iter(|| {
            for &input in &inputs {
                black_box(standard_exp10_f64(input));
            }
        })
    });
}

fn bench_exp10_f32(c: &mut Criterion) {
    let inputs = [1.0, 10.0, 100.0, 1e10, 0.1, 0.01, 1e-10];
    c.bench_function("base10_exponent::exp10<f32>", |b| {
        b.iter(|| {
            for &input in &inputs {
                black_box(exp10::<f32>(input));
            }
        })
    });
}

fn bench_standard_exp10_f32(c: &mut Criterion) {
    let inputs = [1.0, 10.0, 100.0, 1e10, 0.1, 0.01, 1e-10];
    c.bench_function("standard log10<f32>", |b| {
        b.iter(|| {
            for &input in &inputs {
                black_box(standard_exp10_f32(input));
            }
        })
    });
}

criterion_group!(
    benches,
    bench_exp10_f64,
    bench_standard_exp10_f64,
    bench_exp10_f32,
    bench_standard_exp10_f32
);
criterion_main!(benches);
