# Herman 🦀

A rusty daemon that watches folders and organises the files automatically

## Installation

Install globally with `cargo`

```bash
cargo add herman
```
    
## Usage/Examples

Clean a folder

```bash
cargo add herman
herman clean ./downloads
```

Watch a folder

```bash
cargo add herman
herman watch ./downloads
```

## Run Locally

Clone and navigate to directory

```bash
git clone https://github.com/SeanningTatum/herman
cd herman
```

Run watcher locally

```bash
cargo run -- watch ./test-folder
```
Run script locally

```bash
cargo run -- clean ./test-folder
```

## License

[MIT](./LICENSE.txt)


