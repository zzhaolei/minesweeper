FROM rust:latest
RUN apt-get update && \
    apt-get install -y g++ pkg-config libx11-dev libasound2-dev libudev-dev && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY . .

CMD ["cargo", "build", "--release"]
