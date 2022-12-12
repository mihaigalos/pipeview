# pipeview

## Why?

Simple coloring of an input pipe.

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

