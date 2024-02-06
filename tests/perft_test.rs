use gegene::game::Game;

#[cfg(test)]
#[test]
fn perft_1_1() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let game = Game::from_fen(fen).expect("invalid FEN");

    let nodes = game.perft(1, 1);

    assert_eq!(nodes, 20);
}

#[test]
fn perft_1_2() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let game = Game::from_fen(fen).expect("invalid FEN");
    let nodes = game.perft(2, 2);
    assert_eq!(nodes, 400);
}

#[test]
fn perf_1_3() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let game = Game::from_fen(fen).expect("invalid FEN");
    let nodes = game.perft(3, 3);
    assert_eq!(nodes, 8_902);
}

#[test]
fn perf_1_4() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let game = Game::from_fen(fen).expect("invalid FEN");
    let nodes = game.perft(4, 4);
    assert_eq!(nodes, 197_281);
}

#[test]
fn perf_1_5() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let game = Game::from_fen(fen).expect("invalid FEN");
    let nodes = game.perft(5, 5);
    assert_eq!(nodes, 4_865_609);
}

#[test]
fn perf_1_6() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let game = Game::from_fen(fen).expect("invalid FEN");
    let nodes = game.perft(6, 6);
    assert_eq!(nodes, 119_060_324);
}

#[cfg(test)]
#[test]
fn perft_2_1() {
    let fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -";
    let game = Game::from_fen(fen).expect("invalid FEN");
    let nodes = game.perft(1, 1);
    assert_eq!(nodes, 48);
}

#[test]
fn perft_2_2() {
    let fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -";
    let game = Game::from_fen(fen).expect("invalid FEN");
    let nodes = game.perft(2, 2);
    assert_eq!(nodes, 2_039);
}

#[test]
fn perft_2_3() {
    let fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -";
    let game = Game::from_fen(fen).expect("invalid FEN");
    let nodes = game.perft(3, 3);
    assert_eq!(nodes, 97_862);
}

#[test]
fn perft_2_4() {
    let fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -";
    let game = Game::from_fen(fen).expect("invalid FEN");
    let nodes = game.perft(4, 4);
    assert_eq!(nodes, 4_085_603);
}

#[test]
fn perft_2_5() {
    let fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -";
    let game = Game::from_fen(fen).expect("invalid FEN");
    let nodes = game.perft(5, 5);
    assert_eq!(nodes, 193_690_690);
}

#[test]
fn perft_3_1() {
    let fen = "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - ";
    let game = Game::from_fen(fen).expect("invalid FEN");
    let nodes = game.perft(1, 1);
    assert_eq!(nodes, 14);
}

#[test]
fn perft_3_2() {
    let fen = "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - ";
    let game = Game::from_fen(fen).expect("invalid FEN");
    let nodes = game.perft(2, 2);
    assert_eq!(nodes, 191);
}

#[test]
fn perft_3_3() {
    let fen = "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - ";
    let game = Game::from_fen(fen).expect("invalid FEN");
    let nodes = game.perft(3, 3);
    assert_eq!(nodes, 2_812);
}

#[test]
fn perft_3_4() {
    let fen = "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - ";
    let game = Game::from_fen(fen).expect("invalid FEN");
    let nodes = game.perft(4, 4);
    assert_eq!(nodes, 43_238);
}

#[test]
fn perft_3_5() {
    let fen = "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - ";
    let game = Game::from_fen(fen).expect("invalid FEN");
    let nodes = game.perft(5, 5);
    assert_eq!(nodes, 674_624);
}

#[test]
fn perft_3_6() {
    let fen = "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - ";
    let game = Game::from_fen(fen).expect("invalid FEN");
    let nodes = game.perft(6, 6);
    assert_eq!(nodes, 11_030_083);
}
