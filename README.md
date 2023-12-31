# Herman

[![MIT License](https://img.shields.io/badge/License-MIT-green.svg)](https://choosealicense.com/licenses/mit/) 
![GitHub release (with filter)](https://img.shields.io/github/v/release/SeanningTatum/herman)
![Crates.io (version)](https://img.shields.io/crates/dv/herman/0.1.2)
![docs.rs](https://img.shields.io/docsrs/herman)

A rusty daemon that watches folders and organises the files automatically

<img src="./assets/herman.jpg" height="200px">

## Demo

https://github.com/SeanningTatum/herman/assets/26712061/40f38338-0a25-4d65-97ea-4d2a8a559785

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
git clone https://link-to-project
cd herman
```

Run watcher locally

```bash
cargo run -- watch ./test/folder
```
Run script locally

```bash
cargo run -- clean ./test/folder
```

## Roadmap

- [ ] Read Desired Folder Setup from Custom Configuration File
- [ ] Custom Mapping to Dates
  - [ ] Organise by week, month, year, etc. 


## License

[MIT](https://choosealicense.com/licenses/mit/)

