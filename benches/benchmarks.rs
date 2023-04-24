use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::{thread_rng, Rng};
use squares::main::find_top_five_prosperous_squares;

fn generate_random_grid(size: usize) -> [[u32; 1000]; 1000] {
    let mut rng = thread_rng();
    let mut grid: [[u32; 1000]; 1000] = [[0; 1000]; 1000];

    for i in 0..size {
        let mut row = [0; 1000];
        for j in 0..size {
            row[j] = rng.gen_range(0..=255);
        }
        grid[i] = row;
    }

    grid
}

fn benchmark(c: &mut Criterion) {
    let grid = generate_random_grid(1000);
    let mut group = c.benchmark_group("squares_benchmark");

    group.sample_size(100);
    group.bench_with_input(
        BenchmarkId::new("Grid 1000x1000", 1000),
        &grid,
        |b, grid| {
            b.iter(|| find_top_five_prosperous_squares(grid));
        },
    );

    group.finish();
}
criterion_group!(name = benches; config = Criterion::default().significance_level(0.1).sample_size(100); targets = benchmark);
criterion_main!(benches);
