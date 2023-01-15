# pipeview

[![CI](https://github.com/mihaigalos/pipeview/actions/workflows/ci.yaml/badge.svg)](https://github.com/mihaigalos/pipeview/actions/workflows/ci.yaml)
[![crates.io](https://img.shields.io/crates/d/pipeview.svg)](https://crates.io/crates/pipeview)

A command line pipe inspection utility.

![screenshot](screenshots/pipeview.png)

## Why?

* [x] Simple coloring of an input pipe with regex and colors as input args.
* [x] Multiple custom configs in the form of a `pipeview.toml` file in current folder or `~/.config/`.
* [ ] Progress bar (same as Linux's `pv`) - WIP.

## Usage

Explicit coloring can be performed on the input based on a regular expression.

```bash
$ cat test/demo_nginx_access_log | pipeview "^(.*?) - - \\[(.*?)\\] \"(.*?) .*?\" (.*?) .*? \".*?\" \"(.*?)\"" 'bgreen white yellow cyan blue'
```

### Nginx

Nginx and [aim](https://github.com/mihaigalos/aim) logs can be directly inspected using the `--nginx` or `--aim` flag:

```bash
$ cat test/demo_nginx_access_log | pipeview --nginx
```

![screenshot-nginx](screenshots/pipeview-nginx.png)

### Custom configs

You can create a config in `~/.config/pipeview.toml` or the current folder with filename `pipeview.toml` and call it using `pipeview --config=foo`.

An example custom config could be:
```toml
[foo]
regex="^(.*?) (.*?) (.*?): (.*?) (.*)"
colors="red green blue red green"
```

## Installation

### Building from source

```bash
$ cargo install pipeview
```
