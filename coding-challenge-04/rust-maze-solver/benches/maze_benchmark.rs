use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use rust_maze_solver::{
    algorithms::{GeneratorAlgorithm, MazeGenerator, MazeSolver, PathfindingAlgorithm},
};

fn benchmark_generation(c: &mut Criterion) {
    let mut group = c.benchmark_group("Maze Generation");

    for size in [10, 25, 50].iter() {
        group.bench_with_input(BenchmarkId::new("RecursiveBacktracker", size), size, |b, &size| {
            b.iter(|| MazeGenerator::generate(black_box(size), black_box(size), GeneratorAlgorithm::RecursiveBacktracker));
        });

        group.bench_with_input(BenchmarkId::new("Prims", size), size, |b, &size| {
            b.iter(|| MazeGenerator::generate(black_box(size), black_box(size), GeneratorAlgorithm::Prims));
        });

        group.bench_with_input(BenchmarkId::new("Kruskals", size), size, |b, &size| {
            b.iter(|| MazeGenerator::generate(black_box(size), black_box(size), GeneratorAlgorithm::Kruskals));
        });
    }

    group.finish();
}

fn benchmark_solving(c: &mut Criterion) {
    let mut group = c.benchmark_group("Maze Solving");

    for size in [10, 25, 50].iter() {
        let maze = MazeGenerator::generate(*size, *size, GeneratorAlgorithm::RecursiveBacktracker);

        group.bench_with_input(BenchmarkId::new("A*", size), &maze, |b, maze| {
            b.iter(|| MazeSolver::solve(black_box(maze), PathfindingAlgorithm::AStar));
        });

        group.bench_with_input(BenchmarkId::new("BFS", size), &maze, |b, maze| {
            b.iter(|| MazeSolver::solve(black_box(maze), PathfindingAlgorithm::BFS));
        });

        group.bench_with_input(BenchmarkId::new("DFS", size), &maze, |b, maze| {
            b.iter(|| MazeSolver::solve(black_box(maze), PathfindingAlgorithm::DFS));
        });

        group.bench_with_input(BenchmarkId::new("Dijkstra", size), &maze, |b, maze| {
            b.iter(|| MazeSolver::solve(black_box(maze), PathfindingAlgorithm::Dijkstra));
        });
    }

    group.finish();
}

criterion_group!(benches, benchmark_generation, benchmark_solving);
criterion_main!(benches);
