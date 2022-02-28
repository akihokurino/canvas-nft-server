FROM amazonlinux:2

RUN mkdir -p /build/src && \
    yum update -y && \
    yum install -y awscli gcc openssl-devel tree zip && \
    curl -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal

WORKDIR /build
ENV PATH=/root/.cargo/bin:/usr/sbin:/usr/bin:/sbin:/bin

RUN rustup component add rustfmt

CMD \
  cargo build --release --target-dir target_lambda && \
  mv target_lambda/release/api target_lambda/release/bootstrap && \
  zip -9 -j target_lambda/api.zip target_lambda/release/bootstrap && \
  mv target_lambda/release/subscriber target_lambda/release/bootstrap && \
  zip -9 -j target_lambda/subscriber.zip target_lambda/release/bootstrap && \
  mv target_lambda/release/batch target_lambda/release/bootstrap && \
  zip -9 -j target_lambda/batch.zip target_lambda/release/bootstrap