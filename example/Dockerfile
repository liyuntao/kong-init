# Any linux container with openssl 1.1 installed should be OK
FROM jamiehewland/openssl:1.1

# copy one or all of your templates file into image
COPY example/ /app/route-tmpl
RUN cd /app/

# default log level is info, available value: trace|debug|info|warn|error
# ENV RUST_LOG kong_init=info


# use '--wait' to wait for kong until kong started.
# with e.g. 'restart: no' in docker-compose version 2
# make your custom init-container to be start-once mode
CMD ./kong_init --path ./kong14.v2.yaml --url http://kong-server:8001 --wait