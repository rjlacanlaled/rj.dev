# Stage 1: Build Tailwind CSS
FROM node:latest as tailwind
WORKDIR /rj.dev
COPY . .
WORKDIR /rj.dev/apps/web
RUN npm install
RUN npx tailwindcss -i ./static/css/styles.css -o ./static/css/output.css

# Stage 2: Build Rust App
FROM messense/rust-musl-cross:x86_64-musl as builder
WORKDIR /rj.dev
COPY --from=tailwind /rj.dev .
RUN cargo build --release --target x86_64-unknown-linux-musl

# Final: Copy binary and static files to scratch
FROM scratch
COPY --from=builder /rj.dev/target/x86_64-unknown-linux-musl/release/web /web
COPY --from=builder /rj.dev/apps/web/static /apps/web/static
ENTRYPOINT [ "/web" ]
EXPOSE 3000