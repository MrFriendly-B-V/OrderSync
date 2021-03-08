#Build
FROM rust:latest as builder
RUN mkdir -p /usr/src/ordersync

COPY . /usr/src/ordersync
WORKDIR /usr/src/ordersync
RUN cargo install --path .

#Run 
FROM ubuntu:latest
COPY --from=builder /usr/local/cargo/bin/ordersync /usr/local/bin/ordersync
RUN ["ordersync"]
