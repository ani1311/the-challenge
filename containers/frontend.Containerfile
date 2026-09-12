FROM docker.io/library/rust:1-bookworm AS builder
WORKDIR /app
RUN rustup target add wasm32-unknown-unknown \
    && cargo install trunk --locked
COPY . .
RUN cd crates/web && trunk build --release

FROM docker.io/library/nginx:1.27-alpine
COPY containers/nginx.conf /etc/nginx/conf.d/default.conf
COPY --from=builder /app/crates/web/dist /usr/share/nginx/html
EXPOSE 80
