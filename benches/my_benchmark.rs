use criterion::{criterion_group, criterion_main, Criterion};
use gegene::{board, game};

pub fn criterion_benchmark(c: &mut Criterion) {
    let fen = "8/3K4/8/3R4/6b1/8/k7/3q4 w kq - 0 1";
    let game = game::Game::from_fen(fen);

    c.bench_function("check_mask", |b| b.iter(|| {
        let _ = game.board.check_mask(board::Color::White);
    }));

    c.bench_function("pin_mask", |b| b.iter(|| {
        let _ = game.board.pin_mask(board::Color::White);
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);