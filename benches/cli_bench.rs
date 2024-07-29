use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::process::Command;

fn benchmark_command(c: &mut Criterion) {
    c.bench_function("my_cli_command", |b| {
        b.iter(|| {
            let output = Command::new("cargo")
                .arg("run")
                .arg("--")
                .arg("--directory")
                .arg("FibonacciElement")
                .output()
                .expect("failed to execute process");
        });
    });
}

criterion_group!(benches, benchmark_command);
criterion_main!(benches);
