## Build

Inside `/crates/parser`

```bash
cargo build --release
```
The binary will be available at:

**../../target/release/parse**

## Basic usage

## Parse a Hoon file to Json:
```bash
../../target/release/parser file_to_parse.hoon --out out.json
```
## Watch Directory:
```bash
../../target/release/parser /mydir --watch --out out.json
```
## Print to stdout (if --out is omitted)
```bash
../../target/release/parser file_to_parse.hoon
```
## Disable debug traces
```
../../target/release/parser --no-dbug file_to_parse.hoon
```
## Run tests
```
../../target/release/parser --test
```