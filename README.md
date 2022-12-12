# pipeview
[![crates.io](https://img.shields.io/crates/d/pipeview.svg)](https://crates.io/crates/pipeview)

A command line pipe inspection utility.

![screenshot](screenshots/pipeview.png)

## Why?

* [x] Simple coloring of an input pipe.
* [ ] Progress bar (same as Linux's `pv`) - WIP.

## Usage

Explicit coloring can be performed on the input based on a regular expression.

```bash
$ cat test/demo_nginx_access_log | pipeview "^(.*?) - - \\[(.*?)\\] \"(.*?) .*?\" (.*?) .*? \".*?\" \"(.*?)\"" 'bgreen white yellow cyan blue'
```

### Nginx

Nginx logs can be directly inspected using the `--nginx` flag:

```bash
$ cat test/demo_nginx_access_log | pipeview --nginx
```

![screenshot-nginx](screenshots/pipeview-nginx.png)

