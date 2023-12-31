## Fentrail

### About

`Fentrail` is a set of tools (Rust library, CLI, and HTTP server) to investigate possible routes a chess game can traverse to reach a specific position. The system's workflow is composed of two separate components. First, using the CLI, a key-value store is generated by processing a user-provided games database in [PGN](https://www.chessprogramming.org/Portable_Game_Notation) format. Second, the generated store is queried either via HTTP requests or from the command line through [FEN](https://www.chessprogramming.org/Forsyth-Edwards_Notation) inputs.

### Installation

Assuming you have [Rust installed](https://rustup.rs), the `fentrail` CLI and the `serveft` HTTP server can be installed from https://crates.io:

```
cargo install fentrail
```
```
cargo install serveft
```

### Usage
The two CLI sub-commands along with their options and arguments are documented with the provided `--help` flags.
```
$ fentrail --help
Usage: fentrail <command> [<args>]

Build and query opening lookup tables for chess positions.

Options:
  --help            display usage information

Commands:
  pack              Initiate or populate KV-stores.
  ask               Inquire a store about a specific position.


$ fentrail pack --help
Usage: fentrail pack <pgn> [-d <depth>] [-e <ecotsv>] [-o <outdir>]

Initiate or populate KV-stores.

Positional Arguments:
  pgn               path to the game database in PGN format

Options:
  -d, --depth       moves from each side to process in a game [default: 12]
  -e, --ecotsv      path to a tsv file to be used as an ECO
  -o, --outdir      directory to store the fentrail database [default: $PWD]
  --help            display usage information


$ fentrail ask --help
Usage: fentrail ask <fen> [-s <store>]

Inquire a store about a specific position.

Positional Arguments:
  fen               FEN string to query

Options:
  -s, --store       path to the fentrail database [default: $PWD/fentrail.redb]
  --help            display usage information
```
The `pack` command does not overwrite an existing store, rather it adds unseen `(position, opening)` pairs from fresh PGN inputs. You can always manually delete the old database before invoking `pack` if that suits your use case(s).

If you wish to provide a custom [ECO](https://www.chessprogramming.org/ECO) for `pack`, it must be a TSV file with at least two columns - `name`, and `pgn`. You can check out the [bundled](https://github.com/icp1994/fentrail/blob/main/api/eco.tsv) ECO as an example.

The `serveft` command does not take any additional arguments - rather it can be configured with env vars.

- `SERVEFT_PORT`: port through which the HTTP requests are served - default `8000`
- `SERVEFT_STORE`: path to the fentrail database for querying - default `$PWD/fentrail.redb`

Once running, you can send POST requests to the server with FEN payloads.
```
SERVEFT_PORT=3000 SERVEFT_STORE=/var/db/fentrail.redb serveft &
```
```
curl -sS 0.0.0.0:3000 -d 'rnbq1rk1/ppp1ppbp/3p1np1/8/2PPP3/2N2N1P/PP3PP1/R1BQKB1R b KQ - 0 6' 
```

### Acknowledgements
- https://github.com/lichess-org/chess-openings
- https://github.com/lichess-org/lila-openingexplorer

### License
Each package belonging to the `Fentrail` workspace is licensed under the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. Refer to the [COPYING](https://github.com/icp1994/fentrail/blob/main/COPYING) file for details.