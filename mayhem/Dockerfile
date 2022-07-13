# Build Stage
FROM ghcr.io/evanrichter/cargo-fuzz:latest as builder

RUN apt update && apt install -y libasound2-dev

## Add source code to the build stage.
ADD . /src
WORKDIR /src

RUN echo building instrumented harnesses && \
    bash -c "pushd fuzz && cargo +nightly -Z sparse-registry fuzz build && popd" && \
    mv fuzz/target/x86_64-unknown-linux-gnu/release/decode /decode && \
    echo done

RUN echo building non-instrumented harnesses && \
    export RUSTFLAGS="--cfg fuzzing -Clink-dead-code -Cdebug-assertions -C codegen-units=1" && \
    bash -c "pushd fuzz && cargo +nightly -Z sparse-registry build --release && popd" && \
    mv fuzz/target/release/decode /decode_no_inst && \
    echo done

# Package Stage
FROM rustlang/rust:nightly

COPY --from=builder /usr/lib/x86_64-linux-gnu/libasound.so.2 /usr/lib/x86_64-linux-gnu/
COPY --from=builder /decode /decode_no_inst /
