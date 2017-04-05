FROM debian:jessie

ENV USER root
ENV RUST_VERSION=1.16.0

# install git, cmake, ...
RUN apt-get update && \
  DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
    build-essential \
    ca-certificates \
    cmake \
    curl \
    git \
    libssl-dev \
    pkg-config && \
  # install rust binaries
  curl -sO https://static.rust-lang.org/dist/rust-$RUST_VERSION-x86_64-unknown-linux-gnu.tar.gz && \
  tar -xzf rust-$RUST_VERSION-x86_64-unknown-linux-gnu.tar.gz && \
  ./rust-$RUST_VERSION-x86_64-unknown-linux-gnu/install.sh --without=rust-docs && \
  # install nodejs
  curl -sL https://deb.nodesource.com/setup_7.x | bash - && \
  apt-get update && \
  DEBIAN_FRONTEND=noninteractive apt-get install -y nodejs && \
  # install rustfmt
  cargo install rustfmt && \
  mv /root/.cargo/bin/rustfmt /usr/local/bin && \
  # cleanup
  DEBIAN_FRONTEND=noninteractive apt-get remove --purge -y curl && \
  DEBIAN_FRONTEND=noninteractive apt-get autoremove -y && \
  rm -rf \
    rust-$RUST_VERSION-x86_64-unknown-linux-gnu \
    rust-$RUST_VERSION-x86_64-unknown-linux-gnu.tar.gz \
    /var/lib/apt/lists/* \
    /tmp/* \
    /var/tmp/*

# add jenkins user
RUN useradd jenkins --shell /bin/bash --create-home
USER jenkins
