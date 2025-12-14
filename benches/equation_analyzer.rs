use criterion::{criterion_group, criterion_main, Criterion};
use rusty_maths::equation_analyzer::calculator;
use std::hint::black_box;

/// Benchmark calculate() with equations of varying complexity
fn bench_calculate(c: &mut Criterion) {
    let equations = vec![
        ("arithmetic", "sqrt(16) + abs(-5) * 3 - 2^4"),
        ("polynomial", "3x^4 - 2x^3 + 5x^2 - x + 7"),
        ("trigonometric", "sin(π/4) * cos(π/3) + tan(π/6)"),
        ("nested_functions", "sqrt(abs(ln(e^2) + log_2(16)))"),
        ("complex", "sin(x) * cos(2x) + tan(x/2) - sqrt(abs(x))"),
        (
            "statistical",
            "avg(10, 20, 30) + min(5, 15, 25) * max(2, 4, 8)",
        ),
    ];

    for (name, eq) in equations {
        c.bench_function(name, |b| {
            b.iter(|| calculator::calculate(black_box(eq)));
        });
    }
}

/// Benchmark plot() with different point counts and equations
fn bench_plot(c: &mut Criterion) {
    let test_cases = vec![
        ("plot_quadratic_100pts", "x^2 - 4x + 3", 100),
        (
            "plot_trigonometric_1000pts",
            "sin(2x) * cos(x) + tan(x/3)",
            1000,
        ),
        (
            "plot_complex_500pts",
            "sqrt(abs(x)) + ln(abs(x) + 1) * 2",
            500,
        ),
    ];

    for (name, equation, points) in test_cases {
        let step = 10.0 / points as f32;
        c.bench_function(name, |b| {
            b.iter(|| calculator::plot(black_box(equation), -5.0, 5.0, step));
        });
    }
}

criterion_group!(benches, bench_calculate, bench_plot);
criterion_main!(benches);
