# Use Rust official image (with Debian base for compatibility)
FROM rust:latest

# Author label
LABEL authors="lewissmatos"

# Install required dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    perl \
    build-essential

# Set working directory
WORKDIR /app

# Copy the project files into the container
COPY . .

# Make script executable and run it
RUN chmod +x /app/CoOrENYOCd.sh
RUN cd /app && sh -eux CoOrENYOCd.sh

# Default command to start the application
CMD ["cargo", "run"]

# Keeps the container running (optional, usually not needed for bots)
ENTRYPOINT ["top", "-b"]