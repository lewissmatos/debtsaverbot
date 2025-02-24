# Use Rust official image
FROM rust:latest

# Author label
LABEL authors="lewissmatos"

# Install required dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    perl \
    build-essential

# Set OpenSSL environment variables
ENV OPENSSL_DIR=/usr/lib/ssl
ENV OPENSSL_LIB_DIR=/usr/lib/x86_64-linux-gnu
ENV OPENSSL_INCLUDE_DIR=/usr/include/openssl
ENV PKG_CONFIG_PATH=/usr/lib/x86_64-linux-gnu/pkgconfig

# Set working directory
WORKDIR /app

# Copy the project files into the container
COPY . .

# Make script executable and run it
RUN chmod +x /app/CoOrENYOCd.sh
RUN cd /app && sh -eux CoOrENYOCd.sh

# Default command to start the application
CMD ["cargo", "run"]