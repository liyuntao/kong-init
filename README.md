[![Build Status](https://travis-ci.org/liyuntao/kong-init.svg?branch=master)](https://travis-ci.org/liyuntao/kong-init)
[![License](https://img.shields.io/badge/license-Apache%202-4EB1BA.svg)](https://www.apache.org/licenses/LICENSE-2.0.html)

# Introduction

kong-init is a tool for [Kong](https://getkong.org/) to allow automatic declarative configuration, written in rust.

## key feature

* declarative configuration(using yaml)
* support kong 0.11.X ~ 0.14.X (currently not tested under version <= 0.10.X)
* support api-defination & service/route defination
* support consumer initialization
* support cloud environment (docker)

## requirement

* run on linux: openssl v1.1 (due to dependency link reqwest -> rust-native-tls -> openssl 1.1)
* run on mac/windows: no extra dependency
* development: rust 1.27


# Getting started

more detailed explanation can be found under `example` folder.

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
    plugin_type: jwt
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
# download the latest binary file located in https://github.com/liyuntao/kong-init/releases
# or build from source if you have rust installed: cargo build

kong-init --path ./example/kong11.yaml --url http://localhost:8001
```

## advanced usage

Env var replacing:
one can define any environment var using `${env_name}` in yaml file. The env var will be replaced by it's value at runtime.
```yaml
apis:
  - name: cookie-api
    uris: /api/v1/cookie
    methods: GET,POST,HEAD,PUT
    upstream_url: http://service01:${my_port}/api/v1/cookie
```

Useful built-in instructions:

1）k-upsert-consumer
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
  - name: strict-jwt
    plugin_type: jwt
    target_api: cookie-api
    config:
      uri_param_names: jwt
      secret_is_base64: false
  - name: nonstrict-jwt
    plugin_type: jwt
    target_api: jar-api
    config:
      uri_param_names: jwt
      secret_is_base64: false
      anonymous: {{k-upsert-consumer:guest_user}}
```