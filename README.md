[![Build Status](https://travis-ci.org/liyuntao/kong-init.svg?branch=master)](https://travis-ci.org/liyuntao/kong-init)
[![License](https://img.shields.io/badge/license-Apache%202-4EB1BA.svg)](https://www.apache.org/licenses/LICENSE-2.0.html)

# Introduction

kong-init is a tool for [Kong](https://getkong.org/) to allow automatic declarative configuration, written in rust.

## key feature

* declarative configuration(using yaml)
* support kong CE 0.11.X ~ 0.14.X (currently not tested under version <= 0.10.X)
* support kong EE 0.30 ~ 0.33
* support api-definition & service/route definition
* support consumer initialization
* support credentials initialization (jwt/oauth/acls)
* support cloud environment (docker)

## requirement

* run on linux: openssl v1.1 (due to dependency link reqwest -> rust-native-tls -> openssl 1.1)
* run on mac/windows: no extra dependency
* development: rust 1.29.1


# Getting started

more detailed explanation can be found under `example` folder.

## API style definition

Declare API style configurations in a yaml file. 
```yaml
apis:
  - name: cookie-api
    uris: /api/v1/cookie
    methods: GET,POST,HEAD,PUT
    upstream_url: http://service01:8080/api/v1/cookie

plugins:
  - name: jwt
    plugin_type: jwt
    target_api: cookie-api
    config:
      uri_param_names: jwt
      secret_is_base64: false

```


## Service/Route style definition

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
# download the latest binary file located in https://github.com/liyuntao/kong-init/releases
# or build from source if you have rust installed: cargo build

kong-init --path ./example/kong11.yaml --url http://localhost:8001
```

## Command-line options

```
USAGE:
    kong-init [FLAGS] [OPTIONS] --url <admin-url> --path <path>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -w, --wait       wait until kong-server is ready(suit for init under cloud environment)

OPTIONS:
        --url <admin-url>       admin url of kong-server(e.g. http://kong_ip:8001)
        --header <header>...    add custom header for admin-api request
    -p, --path <path>           path to route defination file
```

## advanced usage

#### debug mode

kong-init use `env_logger` for logging, which is a simple logger can be configured via environment variables.
```bash
# set log_level to debug
RUST_LOG=kong_init=debug kong-init --path ./example/kong11.yaml --url http://localhost:8001
```

#### env var replacing:

one can define any environment var using `${env_name}` in yaml file. The env var will be replaced by it's value at runtime.
```yaml
apis:
  - name: cookie-api
    uris: /api/v1/cookie
    methods: GET,POST,HEAD,PUT
    upstream_url: http://service01:${my_port}/api/v1/cookie
```

```bash
# env var replacing example
my_port=8081 kong-init --path ./example/kong11.yaml --url http://localhost:8001
```


#### useful built-in instructions:

##### 1）k-upsert-consumer
* scenario: In some scenarios, we want our api can support both request with or without jwt header(do not return 401 if without jwt). 
So we must configure `config.anonymous`. However this field can only accepts an uuid with existing consumer, not so convenient for our initialization.
We can use k-upsert-consumer directive to acheve this. It will replaced by a real uuid at runtime.
* ability: using given `consumer_id` to fetch or create the uuid of consumer. (will fetch if consumer exists for idempotent initialization)
* args: consumer_id 
* usage: {{k-upsert-consumer:<custom_name_str>}}

```yaml
apis:
  - name: cookie-api
    uris: /api/v1/cookie
    methods: GET,POST,HEAD,PUT
    upstream_url: http://service01:8080/api/v1/cookie
  - name: jar-api
    uris: /api/v1/jar
    upstream_url: http://service02:8080/api/v1/jar

plugins:
  - name: strict-jwt # will return 401 if request without token
    plugin_type: jwt
    target_api: cookie-api
    config:
      uri_param_names: jwt
      secret_is_base64: false
  - name: nonstrict-jwt # will fallback to specified user/consumer if request without token
    plugin_type: jwt
    target_api: jar-api
    config:
      uri_param_names: jwt
      secret_is_base64: false
      anonymous: {{k-upsert-consumer:guest_user}}
```