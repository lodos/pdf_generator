#!/bin/bash

# Установка wkhtmltopdf через Homebrew
echo "Установка wkhtmltopdf..."
brew install wkhtmltopdf

# Проверка успешности установки
if command -v wkhtmltopdf &> /dev/null; then
    echo "wkhtmltopdf успешно установлен."
else
    echo "Ошибка установки wkhtmltopdf." >&2
    exit 1
fi

# Сборка Rust проекта
echo "Сборка Rust проекта..."
cargo build --target aarch64-apple-darwin --release

# Проверка успешности сборки
if [ $? -eq 0 ]; then
    echo "Проект успешно собран."
else
    echo "Ошибка сборки проекта." >&2
    exit 1
fi