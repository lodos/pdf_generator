# Используйте официальный образ Rust
FROM rust:latest

# Установите необходимые зависимости
RUN apt-get update && \
    apt-get install -y wget && \
    apt-get install -y libxrender1 libxext6 libfontconfig1 && \
    apt-get clean

# Установка wkhtmltopdf через wget
RUN wget https://github.com/wkhtmltopdf/packaging/releases/download/0.12.6-1/wkhtmltox_0.12.6-1.bionic_amd64.deb && \
    apt-get install -y ./wkhtmltox_0.12.6-1.bionic_amd64.deb && \
    rm wkhtmltox_0.12.6-1.bionic_amd64.deb

# Добавление метаданных
LABEL authors="lodos"

# Установка рабочей директории
WORKDIR /usr/src/app

# Копирование Cargo.toml и Cargo.lock для кэширования зависимостей
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

# Копирование исходного кода
COPY . .

# Сборка приложения
RUN cargo install --path .

# Команда для запуска приложения
CMD ["pdf_generator"]