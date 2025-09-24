# *strainer*

A tool made out of the need to filter out server logs while debugging and wanting to write some Rust. It spawns two threads for stdin and stderr each to avoid `2&>1`, because I have to keep looking it up.

### Quickstart

```bash
git clone https://github.com/MashyBasker/strainer.git
cd strainer
cargo build --release
sudo mv target/release/strainer /usr/local/bin
```

### Usage

Say you have a process `p` with args `a`, `b` and `c` that generates logs when ran like:

```bash
./p a b c
```

You can filter the logs based on some regex pattern like this

```bash
strainer <pattern> ./p a b c
```

By default, the filtered logs are written to `stdout`, but you can use the `--out` flag to write a some file

```bash
strainer <pattern> ./p a b c --out=<path/to/file>
```

Maybe this objective can be achieved with some shell-fu by piping to grep and redirecting to stdout. But eh! I prefer it this way.
