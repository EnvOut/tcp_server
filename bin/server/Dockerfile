ARG BASE_BUILD_IMAGE=rust:1.65.0
ARG RUNNER_IMAGE=ubuntu:latest

FROM $BASE_BUILD_IMAGE as base_builder
WORKDIR /app
COPY ./ .

FROM base_builder as builder
RUN cargo build --target x86_64-unknown-linux-gnu --release

FROM $RUNNER_IMAGE as runner
WORKDIR /app
COPY --from=builder /app/target/x86_64-unknown-linux-gnu/release/server /usr/local/bin/
#
ENTRYPOINT ["server"]
