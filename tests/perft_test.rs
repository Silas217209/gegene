use gegene::game::Game;

#[cfg(test)]
#[test]
fn perft_1_1() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let game = Game::from_fen(fen).expect("invalid FEN");

    let (nodes, captures) = game.perft(1);
    assert_eq!(nodes, 20);
    assert_eq!(captures, 0);
}

#[test]
fn perft_1_2() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let game = Game::from_fen(fen).expect("invalid FEN");

    let (nodes, captures) = game.perft(2);
    assert_eq!(nodes, 400);
    assert_eq!(captures, 0);
}


#[test]
fn perft_1_3() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let game = Game::from_fen(fen).expect("invalid FEN");

    let (nodes, captures) = game.perft(3);
    assert_eq!(nodes, 8902);
    assert_eq!(captures, 34);
}

#[test]
fn perft_1_4() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let game = Game::from_fen(fen).expect("invalid FEN");

    let (nodes, captures) = game.perft(4);
    assert_eq!(nodes, 197281);
    assert_eq!(captures, 1576);
}
