use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use rusty_maths::equation_analyzer::{vec_pipeline, hybrid_pipeline, full_pipeline};

/// Benchmark equations of varying complexity
const SIMPLE_EQ: &str = "2 + 3";
const MODERATE_EQ: &str = "2x^2 + 3x - 5";
const COMPLEX_EQ: &str = "sin(x) * cos(x) + tan(x^2) - sqrt(abs(x))";
const STATISTICAL_EQ: &str = "avg(1, 2, 3, 4, 5) + min(10, 20, 30) * max(5, 15, 25)";
const NESTED_EQ: &str = "((2 + 3) * (4 - 1)) / (6 + sqrt(9))";

/// Benchmark calculate() functions across all pipelines
fn bench_calculate_simple(c: &mut Criterion) {
    let mut group = c.benchmark_group("calculate_simple");

    group.bench_function("vec_pipeline", |b| {
        b.iter(|| {
            vec_pipeline::calculator::calculate(black_box(SIMPLE_EQ))
        });
    });

    group.bench_function("hybrid_pipeline", |b| {
        b.iter(|| {
            hybrid_pipeline::calculator::calculate(black_box(SIMPLE_EQ))
        });
    });

    group.bench_function("full_pipeline", |b| {
        b.iter(|| {
            full_pipeline::calculator::calculate(black_box(SIMPLE_EQ))
        });
    });

    group.finish();
}

fn bench_calculate_moderate(c: &mut Criterion) {
    let mut group = c.benchmark_group("calculate_moderate");

    group.bench_function("vec_pipeline", |b| {
        b.iter(|| {
            vec_pipeline::calculator::calculate(black_box(MODERATE_EQ))
        });
    });

    group.bench_function("hybrid_pipeline", |b| {
        b.iter(|| {
            hybrid_pipeline::calculator::calculate(black_box(MODERATE_EQ))
        });
    });

    group.bench_function("full_pipeline", |b| {
        b.iter(|| {
            full_pipeline::calculator::calculate(black_box(MODERATE_EQ))
        });
    });

    group.finish();
}

fn bench_calculate_complex(c: &mut Criterion) {
    let mut group = c.benchmark_group("calculate_complex");

    group.bench_function("vec_pipeline", |b| {
        b.iter(|| {
            vec_pipeline::calculator::calculate(black_box(COMPLEX_EQ))
        });
    });

    group.bench_function("hybrid_pipeline", |b| {
        b.iter(|| {
            hybrid_pipeline::calculator::calculate(black_box(COMPLEX_EQ))
        });
    });

    group.bench_function("full_pipeline", |b| {
        b.iter(|| {
            full_pipeline::calculator::calculate(black_box(COMPLEX_EQ))
        });
    });

    group.finish();
}

fn bench_calculate_statistical(c: &mut Criterion) {
    let mut group = c.benchmark_group("calculate_statistical");

    group.bench_function("vec_pipeline", |b| {
        b.iter(|| {
            vec_pipeline::calculator::calculate(black_box(STATISTICAL_EQ))
        });
    });

    group.bench_function("hybrid_pipeline", |b| {
        b.iter(|| {
            hybrid_pipeline::calculator::calculate(black_box(STATISTICAL_EQ))
        });
    });

    group.bench_function("full_pipeline", |b| {
        b.iter(|| {
            full_pipeline::calculator::calculate(black_box(STATISTICAL_EQ))
        });
    });

    group.finish();
}

/// Benchmark plot() functions with different point counts
fn bench_plot_varying_points(c: &mut Criterion) {
    let mut group = c.benchmark_group("plot_points");

    let equation = "x^2 + 2x + 1";
    let point_counts = [10, 100, 1000, 10000];

    for &count in &point_counts {
        let step = 10.0 / count as f32;

        group.bench_with_input(
            BenchmarkId::new("vec_pipeline", count),
            &(equation, step),
            |b, &(eq, s)| {
                b.iter(|| {
                    vec_pipeline::calculator::plot(black_box(eq), black_box(-5.0), black_box(5.0), black_box(s))
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("hybrid_pipeline", count),
            &(equation, step),
            |b, &(eq, s)| {
                b.iter(|| {
                    hybrid_pipeline::calculator::plot(black_box(eq), black_box(-5.0), black_box(5.0), black_box(s))
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("full_pipeline", count),
            &(equation, step),
            |b, &(eq, s)| {
                b.iter(|| {
                    full_pipeline::calculator::plot(black_box(eq), black_box(-5.0), black_box(5.0), black_box(s))
                });
            },
        );
    }

    group.finish();
}

/// Benchmark plot() with complex equations
fn bench_plot_complex_equation(c: &mut Criterion) {
    let mut group = c.benchmark_group("plot_complex");

    let equation = "sin(x) * cos(x) + tan(x/2)";
    let step = 0.1;

    group.bench_function("vec_pipeline", |b| {
        b.iter(|| {
            vec_pipeline::calculator::plot(black_box(equation), black_box(-5.0), black_box(5.0), black_box(step))
        });
    });

    group.bench_function("hybrid_pipeline", |b| {
        b.iter(|| {
            hybrid_pipeline::calculator::plot(black_box(equation), black_box(-5.0), black_box(5.0), black_box(step))
        });
    });

    group.bench_function("full_pipeline", |b| {
        b.iter(|| {
            full_pipeline::calculator::plot(black_box(equation), black_box(-5.0), black_box(5.0), black_box(step))
        });
    });

    group.finish();
}

/// Benchmark repeated parsing (to show optimization benefit)
/// This simulates the old approach where equations were parsed for every x value
fn bench_repeated_parsing(c: &mut Criterion) {
    let mut group = c.benchmark_group("repeated_parsing");

    let equation = "2x^2 + 3x - 5";
    let iterations = 100;

    // Old approach: calculate with string substitution for each x value
    group.bench_function("old_approach_substitute_x", |b| {
        b.iter(|| {
            for i in 0..iterations {
                let x = i as f32;
                let eq_with_x = equation.replace("x", &x.to_string());
                let _ = vec_pipeline::calculator::calculate(&eq_with_x);
            }
        });
    });

    // New approach: plot() parses once, evaluates many times
    group.bench_function("new_approach_plot", |b| {
        b.iter(|| {
            vec_pipeline::calculator::plot(black_box(equation), black_box(0.0), black_box(99.0), black_box(1.0))
        });
    });

    group.finish();
}

/// Benchmark pipeline comparison with all operations
fn bench_full_pipeline_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("full_pipeline_comparison");
    group.sample_size(100);

    let equations = vec![
        ("simple", "2 + 3"),
        ("polynomial", "x^3 - 2x^2 + x - 1"),
        ("trig", "sin(x) + cos(x)"),
        ("nested", "((x + 1) * (x - 1)) / (x + 2)"),
        ("statistical", "avg(x, 2x, 3x) + min(x, 10)"),
    ];

    for (name, eq) in equations {
        group.bench_with_input(
            BenchmarkId::new("vec", name),
            &eq,
            |b, &equation| {
                b.iter(|| {
                    vec_pipeline::calculator::calculate(black_box(equation))
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("hybrid", name),
            &eq,
            |b, &equation| {
                b.iter(|| {
                    hybrid_pipeline::calculator::calculate(black_box(equation))
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("full", name),
            &eq,
            |b, &equation| {
                b.iter(|| {
                    full_pipeline::calculator::calculate(black_box(equation))
                });
            },
        );
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_calculate_simple,
    bench_calculate_moderate,
    bench_calculate_complex,
    bench_calculate_statistical,
    bench_plot_varying_points,
    bench_plot_complex_equation,
    bench_repeated_parsing,
    bench_full_pipeline_comparison,
);

criterion_main!(benches);
