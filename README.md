[![Build Status](https://travis-ci.org/liyuntao/kong-init.svg?branch=master)](https://travis-ci.org/liyuntao/kong-init)
[![License](https://img.shields.io/badge/license-Apache%202-4EB1BA.svg)](https://www.apache.org/licenses/LICENSE-2.0.html)

# Introduction

kong-init is a tool for [Kong](https://getkong.org/) to allow automatic declarative configuration, written in rust.

## key feature

* declarative configuration(using yaml)
* support kong 0.11.X ~ 0.14.X (currently not tested under version <= 0.10.X)
* support api-defination & service/route defination
* support cloud environment (docker)

## requirment

* run on linux: openssl v1.1 (due to dependency link reqwest -> rust-native-tls -> openssl 1.1)
* run on mac/windows: no extra dependency
* development: rust 1.27


# Getting started

more detail explanation under `example` folder.

## API style defination

Declare API style configurations in a yaml file. 
```yaml
apis:
  - name: cookie-api
    uris: /api/v1/cookie
    methods: GET,POST,HEAD,PUT
    upstream_url: http://service01:8080/api/v1/cookie

plugins:
  - name: jwt
    plguin_type: jwt
    target_api: cookie-api
    config:
      uri_param_names: jwt
      secret_is_base64: false

```


## Service/Route style defination

Declare Service/Route style configurations in a yaml file. Suit for kong version >= 0.13
```yaml
services:
  - name: netdisk
    url: http://host.docker.internal:8090
  - name: dummy
    url: http://host.docker.internal:7090/dummy

routes:
  - name: r-netdisk
    apply_to: netdisk
    config:
      paths: ["/api/v1/netdisk"]
      strip_path: false
  - name: r-dummy
    apply_to: dummy
    config:
      paths: ["/dummy"]
      strip_path: true
  - name: r-dummy-no-auth
    apply_to: dummy
    config:
      paths: ["/dummy/login"]
      strip_path: true
```

## run the command

```bash
export RUST_LOG=kong_init=info
kong_init --path ./example/kong11.yaml --url http://localhost:8001
```
