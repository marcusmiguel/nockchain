## Build

Inside `/crates/parser`

```bash
cargo build --release
```
The binary will be available at:

**../../target/release/parse**

## Run tests
```
cargo test --release -- --nocapture
```

## Usage

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
## Disable Debug traces
```
../../target/release/parser --no-dbug file_to_parse.hoon
```
## Parse Hoon without imports
```
../../target/release/parser --no-imports file_to_parse.hoon
```