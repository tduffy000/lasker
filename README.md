# `lasker`
An attempt to build a UCI-compliant chess engine from the ground up in pure Rust. The primary goal here is to learn how chess engines are written, and not necessarily to optimize performance (maybe one day that goal will change). As such, there is a lot of fluff in here that _should increase_ comprehension.

## perft testing
In order to perform [`perft`](https://www.chessprogramming.org/Perft) testing which counts the number of engine-valid moves from a given FEN string to a specified depth, `lasker` uses the same mechanism as [Stockfish](https://github.com/official-stockfish/Stockfish/blob/df0fb8471e5015bb4ba0b398c203b7faad45840e/src/uci.cpp#L146) which appends it to the `go` uci command. 

So, in order to run `perft`, build the binary or do
```bash
cargo run
```
and then `lasker` will receive the command via `stdin`. The first step is to use the `position` command to tell the engine what to set the current position as. E.g.
```bash
position fen rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1
```
then, running `perft` a specified `depth` uses the `go` command. 
```bash
go perft 2
```
This will print the number of valid moves from the starting position to a `depth` of 2.

## Resources
### Forums
- [Talkchess](talkchess.com)
- http://www.tckerrigan.com/Chess/TSCP/

### Wikis
- [Chess Programming Wiki](https://www.chessprogramming.org/)

### YouTube
- [Chess Engine in C](https://www.youtube.com/playlist?list=PLZ1QII7yudbc-Ky058TEaOstZHVbT-2hg)

### Other Rust Engines
- [List](https://www.chessprogramming.org/Category:Rust)
- [Chess Programming Rust Overview](https://www.chessprogramming.org/Rust)

### Helpful
- [Chess FEN Viewer](https://www.dailychess.com/chess/chess-fen-viewer.php)
