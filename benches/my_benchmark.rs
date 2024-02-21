use criterion::{criterion_group, criterion_main, Criterion};
use gegene::{bitboard::Bitboard, game, board::Board};

pub fn criterion_benchmark(c: &mut Criterion) {
    let fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - ";
    let game = game::Game::from_fen(fen).expect("Valid FEN");

    let starting_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let starting_game = game::Game::from_fen(starting_fen).expect("Valid FEN");

    let mut group = c.benchmark_group("move_generation");

    group.bench_function("check_mask", |b| {
        b.iter(|| {
            let _ = game.board.check_mask(true);
        })
    });

    group.bench_function("pin_mask", |b| {
        b.iter(|| {
            let _ = game.board.pin_mask(true);
        })
    });

    group.bench_function("rook_attacks", |b| {
        b.iter(|| {
            let _ = Board::rook_attacks(22, Bitboard(0));
        })
    });

    group.bench_function("seen_by_enemy", |b| {
        b.iter(|| {
            let _ = game.board.seen_by_enemy(true);
        })
    });

    group.bench_function("perft", |b| {
        b.iter(|| {
            let _ = game.perft(4, 4, false);
        })
    });
    
    group.bench_function("perft_standart", |b| {
        b.iter(|| {
            let _ = starting_game.perft(4, 4, false);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
