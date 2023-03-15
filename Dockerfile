FROM gitpod/workspace-full:latest

USER root
RUN apt-get update -y -q && apt-get upgrade -y -q \
  && DEBIAN_FRONTEND=noninteractive apt-get install -y -q --no-install-recommends \
  firefox fonts-noto-cjk fonts-noto-color-emoji

USER gitpod
RUN rustup toolchain uninstall stable && rustup toolchain install stable
ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse
RUN cargo install geckodriver