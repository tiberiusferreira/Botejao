FROM resin/rpi-raspbian:jessie-20171018

WORKDIR /botejao

RUN apt update && apt upgrade && apt install -y \
    build-essential \
    pkg-config \
    openssl \
    libssl-dev \
    curl \
    wget \
    git \
    firefox-esr \
    xvfb \
    vim \
    tmux \
    && wget https://github.com/mozilla/geckodriver/releases/download/v0.15.0/geckodriver-v0.15.0-arm7hf.tar.gz \
    && tar -xzf geckodriver-v0.15.0-arm7hf.tar.gz \
    && rm geckodriver-v0.15.0-arm7hf.tar.gz \
    && curl https://sh.rustup.rs -sSf | sh -s -- -y \
    && git clone https://github.com/tiberiusferreira/botejao.git \
    && cd botejao \
    && git checkout alpha\
    && cp ../geckodriver . \
    && /bin/bash -c "source ~/.profile && cargo build" \

    CMD /bin/bash
    