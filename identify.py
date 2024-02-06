import subprocess
import argparse
import sys

import chess


def run_my_perft(fen, depth):
    result = subprocess.run(["./target/release/gegene", "--fen", fen, "--depth", str(depth)], capture_output=True,
                            text=True)
    lines = result.stdout.splitlines()
    total_nodes = int(lines[-1].split(": ")[-1])
    move_nodes = lines[:-1]
    move_dict = {}
    for move in move_nodes:
        move = move.split(": ")
        move_dict[move[0]] = int(move[1])

    return total_nodes, move_dict


def run_stockfish_perft(fen, depth):
    p = subprocess.Popen("/home/silas/Downloads/stockfish-ubuntu-x86-64-avx2/stockfish/stockfish-ubuntu-x86-64-avx2",
                         stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.STDOUT,
                         universal_newlines=True)
    commands = f"position fen {fen}\ngo perft {depth}\nquit"
    output, _ = p.communicate(input=commands)
    lines = output.splitlines()

    nodes = int(lines[-2].split(" ")[-1])

    moves_dict = {}
    for move in lines[1:-3]:
        move = move.split(": ")
        moves_dict[move[0]] = int(move[1])

    return nodes, moves_dict


# recursive down to depth 0 comparing nodes
def compare_nodes(fen, depth, board):
    print(f"Comparing nodes at depth {depth}")
    if depth == 0:
        print("Reached depth 0")
        return

    _, my_moves = run_my_perft(fen, depth)
    _, stockfish_moves = run_stockfish_perft(fen, depth)

    for move in stockfish_moves:
        if move not in my_moves:
            print(f"Move {move} is missing")
            print(f"Position: {fen}")
            print(board.move_stack)
            print()
            # stop the recursion here
            return

    for move, child_nodes in my_moves.items():
        move_obj = chess.Move.from_uci(move)
        if not board.is_legal(move_obj):
            print(f"Move {move} is not legal in the current board state")

        board.push(move_obj)
        fen = board.fen()

        if move not in stockfish_moves:
            print(f"Move {move} is additional")
            print(f"Position: {fen}")
            print()
            board.pop()
            return

        if child_nodes != stockfish_moves[move]:
            print(f"Move {move} has different number of nodes ({child_nodes} vs {stockfish_moves[move]})")
            print(f"Position: {fen}")
            print()
            compare_nodes(fen, depth - 1, board)
            return
        board.pop()


def main():
    parser = argparse.ArgumentParser("problem-identifier")
    parser.add_argument("fen", type=str, help="FEN string of the position to test")
    args = parser.parse_args()
    fen = args.fen
    board = chess.Board(fen)

    for depth in range(1, 8):
        print(f"Running perft at depth {depth}")
        my_nodes, my_moves = run_my_perft(fen, depth)
        stockfish_nodes, stockfish_moves = run_stockfish_perft(fen, depth)

        if my_nodes == stockfish_nodes:
            continue

        print(f"Nodes at depth {depth} are different")

        compare_nodes(fen, depth, board)
        return


if __name__ == "__main__":
    main()
