# Note: This Dockerfile uses multi-stage builds. Therefore, the required Docker
#       version is 17.05 or (preferably) a later version on the client and
#       server side of Docker.

# build stage: builds application, collects data, generates HTML files
FROM debian:bookworm-slim AS builder
LABEL maintainer="Dirk Stolle <striezel-dev@web.de>"
RUN export DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get upgrade -y
RUN apt-get install -y cargo rustc libsqlite3-dev libssl-dev pkg-config
RUN mkdir -p /opt/corona
COPY ./ /opt/corona
WORKDIR /opt/corona
RUN cargo build --release
# Generate database from Our World In Data's compact COVID-19 data.
RUN apt-get install -y wget && \
    wget https://catalog.ourworldindata.org/garden/covid/latest/compact/compact.csv -O /tmp/covid.csv && \
    cargo run --release db /tmp/covid.csv /tmp/corona.db
# Generate HTML files.
RUN cargo run --release html /tmp/corona.db /tmp/html-files

# runtime stage: nginx to serve generated files
FROM nginx:1.28-alpine AS runner
LABEL maintainer="Dirk Stolle <striezel-dev@web.de>"
COPY --from=builder --chown=nginx:nginx /tmp/html-files /usr/share/nginx/html
