FROM gitpod/workspace-rust:latest

USER root
RUN add-apt-repository -y ppa:mozillateam/ppa
RUN printf "Package: firefox*\nPin: release o=LP-PPA-mozillateam\nPin-Priority: 501" > /etc/apt/preferences.d/mozillateamppa
RUN apt-get update -y -q && apt-get upgrade -y -q \
  && DEBIAN_FRONTEND=noninteractive apt-get install -y -q --no-install-recommends \
  firefox fonts-noto-cjk fonts-noto-color-emoji

USER gitpod
RUN rustup toolchain update stable \
    && rustup component add rust-src rustc-dev llvm-tools-preview rustfmt clippy \
    && cargo install cargo-boj@0.6.0 \
    && cargo install geckodriver@^0.34 \
    && rustup toolchain install nightly --component rust-src rustc-dev llvm-tools-preview \
    && cargo +nightly install cargo-oj
