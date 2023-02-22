# feishu-cb

[![build](https://github.com/yinheli/feishu-cb/actions/workflows/build.yml/badge.svg)](https://github.com/yinheli/feishu-cb/actions/workflows/build.yml)

feishu-cb used to API develop, dump requests and send to feishu via bot.

## Usage

```bash
curl -X POST 127.0.0.1:8080/cb/test?p=x -d 'hello'
```

Then you feishu bot will send you following like this:

```text
feishu-cb dump: 127.0.0.1:38782
POST /cb/test?p=x HTTP/1.1
host: 127.0.0.1:8080
user-agent: curl/7.74.0
accept: */*
content-type: application/x-www-form-urlencoded
content-length: 5

hello
```

## Deploy

You can use `helm` to quickly deploy an instance to quickly diagnostic request payload.

```bash
helm upgrade --install feishu-cb ./charts/feishu-cb \
  --set config.webhook=xxx \
  --set ingress.enabled=true \
  --set ingress.hosts[0].host=fs-192-168-4-67.nip.io  \
  --set ingress.hosts[0].paths[0].path=/ \
  --set ingress.hosts[0].paths[0].pathType=Prefix
```

or build from source

```bash
cargo build --release
```

## Limitations

dump body only support `UTF-8` string, binaries are not supported.
