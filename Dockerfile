# ===== Stage 1: Builder =====
FROM rust:1.85-alpine AS builder

# ติดตั้ง dependency สำหรับ build (musl + openssl + build tools)
RUN apk add --no-cache \
    musl-dev \
    openssl-dev \
    openssl-libs-static \
    build-base \
    pkgconf

# Set workdir
WORKDIR /app

# คัดลอกไฟล์ project (คุณสามารถใช้ caching ได้)
COPY Cargo.toml ./
COPY src ./src

# สร้าง release binary
RUN cargo build --release

# ===== Stage 2: Runtime (Minimal) =====
FROM alpine:latest

# ติดตั้ง lib ที่จำเป็นสำหรับรัน binary เช่น openssl
RUN apk add --no-cache libssl3
RUN apk add --no-cache ca-certificates

# คัดลอก binary จาก builder stage
COPY --from=builder /app/target/release/azure-monitor-alert-listener /usr/local/bin/app

COPY template ./template

COPY nickcsdev003.crt /usr/local/share/ca-certificates/
RUN update-ca-certificates

ENV RUST_LOG=info
ENV PORT=8888
# ระบุ entrypoint
ENTRYPOINT ["/usr/local/bin/app"]