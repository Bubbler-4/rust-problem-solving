FROM gitpod/workspace-full:latest

USER root
RUN rm /etc/apt/sources.list.d/ungoogled_chromium.list
RUN apt-get update -y -q \
  && DEBIAN_FRONTEND=noninteractive apt-get install -y -q --no-install-recommends \
  firefox fonts-noto-cjk fonts-noto-color-emoji

USER gitpod