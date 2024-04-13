FROM rust
WORKDIR /app

RUN apt update && apt install -y \
    cmake \
    libjavascriptcoregtk-6.0-1 \
    libwebkit2gtk-4.1-dev \
    build-essential \
    curl \
    wget \
    file \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev

RUN curl -fsSL https://bun.sh/install | bash
RUN cargo install tauri-cli --version 2.0.0-beta.8

COPY . .

RUN /root/.bun/bin/bun install
RUN . ~/.bashrc && cargo tauri build --no-bundle
