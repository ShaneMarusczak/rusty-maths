use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use rusty_maths::equation_analyzer::{vec_pipeline, hybrid_pipeline, full_pipeline};

/// Benchmark calculate() across pipelines with equations of varying complexity
fn bench_calculate(c: &mut Criterion) {
    let mut group = c.benchmark_group("calculate");

    let equations = vec![
        ("arithmetic", "sqrt(16) + abs(-5) * 3 - 2^4"),
        ("polynomial", "3x^4 - 2x^3 + 5x^2 - x + 7"),
        ("trigonometric", "sin(π/4) * cos(π/3) + tan(π/6)"),
        ("nested_functions", "sqrt(abs(ln(e^2) + log_2(16)))"),
        ("complex", "sin(x) * cos(2x) + tan(x/2) - sqrt(abs(x))"),
        ("statistical", "avg(10, 20, 30) + min(5, 15, 25) * max(2, 4, 8)"),
    ];

    for (name, eq) in equations {
        group.bench_with_input(BenchmarkId::new("vec", name), &eq, |b, &equation| {
            b.iter(|| vec_pipeline::calculator::calculate(black_box(equation)));
        });

        group.bench_with_input(BenchmarkId::new("hybrid", name), &eq, |b, &equation| {
            b.iter(|| hybrid_pipeline::calculator::calculate(black_box(equation)));
        });

        group.bench_with_input(BenchmarkId::new("full", name), &eq, |b, &equation| {
            b.iter(|| full_pipeline::calculator::calculate(black_box(equation)));
        });
    }

    group.finish();
}

/// Benchmark plot() with different point counts and equations
fn bench_plot(c: &mut Criterion) {
    let mut group = c.benchmark_group("plot");

    let test_cases = vec![
        ("quadratic_100pts", "x^2 - 4x + 3", 100),
        ("trigonometric_1000pts", "sin(2x) * cos(x) + tan(x/3)", 1000),
        ("complex_500pts", "sqrt(abs(x)) + ln(abs(x) + 1) * 2", 500),
    ];

    for (name, equation, points) in test_cases {
        let step = 10.0 / points as f32;

        group.bench_with_input(BenchmarkId::new("vec", name), &(equation, step), |b, &(eq, s)| {
            b.iter(|| vec_pipeline::calculator::plot(black_box(eq), -5.0, 5.0, s));
        });

        group.bench_with_input(BenchmarkId::new("hybrid", name), &(equation, step), |b, &(eq, s)| {
            b.iter(|| hybrid_pipeline::calculator::plot(black_box(eq), -5.0, 5.0, s));
        });

        group.bench_with_input(BenchmarkId::new("full", name), &(equation, step), |b, &(eq, s)| {
            b.iter(|| full_pipeline::calculator::plot(black_box(eq), -5.0, 5.0, s));
        });
    }

    group.finish();
}

criterion_group!(benches, bench_calculate, bench_plot);
criterion_main!(benches);
