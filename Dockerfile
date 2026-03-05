# Используем официальный образ Rust как базовый
FROM rust:1.93.1-alpine3.22 AS builder

RUN apk add --no-cache musl-dev openssl-dev pkgconfig openssl-libs-static

# Устанавливаем sqlx-cli через cargo
RUN cargo install sqlx-cli --no-default-features --features postgres

# Убираем ненужные зависимости, делаем образ легче
FROM alpine:3.22

# Копируем только бинарник sqlx-cli
COPY --from=builder /usr/local/cargo/bin/sqlx /usr/local/bin/sqlx

# Проверка, что sqlx работает
RUN sqlx --version

# Команда по умолчанию — показать help
CMD ["sqlx", "--help"]