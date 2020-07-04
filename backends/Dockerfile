# syntax=docker/dockerfile:1.1.7-experimental
# @see https://github.com/asaaki/kube-rust-playground/blob/master/simple-web-service/Dockerfile
FROM rust:1.44.1-slim
# FROM rust:1.44.1-slim
# FROM rust:1.44.1-alpine

# ENV DEB_PACKAGES="ca-certificates cmake curl file g++ gcc gcc-multilib git libssl-dev linux-headers-amd64 make musl-tools patch pkg-config wget xz-utils"
ENV DEB_PACKAGES="ca-certificates file make patch pkg-config curl wget xz-utils musl-tools libssl-dev"

RUN rm -f /etc/apt/apt.conf.d/docker-clean \
  && echo 'Binary::apt::APT::Keep-Downloaded-Packages "true";' > /etc/apt/apt.conf.d/keep-cache

RUN \
  --mount=type=cache,target=/var/cache/apt \
  --mount=type=cache,target=/var/lib/apt \
  echo "===== Build environment =====" \
  && uname -a \
  && echo "===== Dependencies =====" \
  && apt-get update \
  && apt-get install -y --no-install-recommends $DEB_PACKAGES \
  && ln -s /usr/bin/musl-gcc /usr/bin/musl-g++ \
  && echo "===== Rust target: musl =====" \
  && rustup target add x86_64-unknown-linux-musl \
  && echo "===== UPX =====" \
  && wget -O upx.tar.xz https://github.com/upx/upx/releases/download/v3.96/upx-3.96-amd64_linux.tar.xz \
  && tar -xf upx.tar.xz --directory /bin --strip-components=1 $(tar -tf upx.tar.xz | grep -E 'upx$') \
  && rm -f upx.tar.xz \
  && echo "===== Toolchain =====" \
  && rustup --version \
  && cargo --version \
  && rustc --version \
  && echo "Rust builder image done."


WORKDIR /app
COPY . .
RUN \
  --mount=type=cache,target=/usr/local/cargo/registry \
  --mount=type=cache,target=/app/target \
  cargo install --path . --root . --bins\
  && date >> LAST_BUILD \
  && ls -ahlF bin/ && cat LAST_BUILD

STOPSIGNAL SIGKILL

CMD ["bin/backends"]
