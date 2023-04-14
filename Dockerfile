FROM rust:1.68.0-bullseye AS build
WORKDIR /app
COPY . .
RUN cargo install perseus-cli --version 0.4.0-rc.1
RUN perseus deploy
RUN mkdir -p /app/lib
RUN cp -LR $(ldd ./dist/target_engine/release/perseus-website | grep "=>" | cut -d ' ' -f 3) /app/lib

FROM scratch AS app
WORKDIR /app
COPY --from=build /app/lib /app/lib
COPY --from=build /lib64/ld-linux-x86-64.so.2 /lib64/ld-linux-x86-64.so.2
COPY --from=build /app/pkg pkg
ENV LD_LIBRARY_PATH=/app/lib

ENTRYPOINT ["./pkg/server"]