FROM rust:1-stretch

RUN rustup component add rustfmt-preview
RUN rustup install nightly
RUN cargo +nightly install clippy

ARG TARGET

RUN dpkg --add-architecture armhf
RUN apt-get update
RUN apt-get install -y crossbuild-essential-armhf

RUN rustup target add ${TARGET}
RUN git clone --depth=1 https://github.com/raspberrypi/tools raspberrypi-tools

WORKDIR /usr/local/src
