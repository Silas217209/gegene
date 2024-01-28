use criterion::{criterion_group, criterion_main, Criterion};
use gegene::{board, game};

pub fn criterion_benchmark(c: &mut Criterion) {
    let fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - ";
    let game = game::Game::from_fen(fen);

    let mut group = c.benchmark_group("move_generation");

    group.bench_function("check_mask", |b| {
        b.iter(|| {
            let _ = game.board.check_mask(board::Color::White);
        })
    });

    group.bench_function("pin_mask", |b| {
        b.iter(|| {
            let _ = game.board.pin_mask(board::Color::White);
        })
    });

    group.bench_function("seen_by_enemy", |b| {
        b.iter(|| {
            let _ = game.board.seen_by_enemy(board::Color::White);
        })
    });

    group.bench_function("get_legal_moves", |b| {
        b.iter(|| {
            let _ = game.get_legal_moves();
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
