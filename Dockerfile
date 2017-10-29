FROM arm32v7/ubuntu:17.10

WORKDIR /botejao

RUN apt update && apt upgrade -y && apt install -y \
    build-essential \
    pkg-config \
    openssl \
    libssl-dev \
    curl \
    wget \
    git \
    firefox \
    xvfb \
    vim \
    tmux \
    && git init . \
    && git remote add -t \* -f origin https://github.com/tiberiusferreira/botejao.git \
    && git checkout alpha \
    && wget https://github.com/mozilla/geckodriver/releases/download/v0.18.0/geckodriver-v0.18.0-arm7hf.tar.gz \
    && tar -xzf geckodriver-v0.18.0-arm7hf.tar.gz \
    && rm geckodriver-v0.18.0-arm7hf.tar.gz \
    && curl https://sh.rustup.rs -sSf | sh -s -- -y \
    && /bin/bash -c "source ~/.profile && cargo build" \

    CMD /bin/bash -c "source ~/.profile && cargo run"
    