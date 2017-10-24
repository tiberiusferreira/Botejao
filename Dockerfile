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
    && wget https://github.com/mozilla/geckodriver/releases/download/v0.18.0/geckodriver-v0.18.0-arm7hf.tar.gz \
    && tar -xzf geckodriver-v0.18.0-arm7hf.tar.gz \
    && rm geckodriver-v0.18.0-arm7hf.tar.gz \
    && curl https://sh.rustup.rs -sSf | sh -s -- -y
    
    CMD /bin/bash -c "source ~/.profile && \
     git clone https://github.com/tiberiusferreira/botejao.git && \
     cd botejao && \
     git checkout usp && \
     cp ../geckodriver . && \
     Xvfb & \
     (sleep 10 ; source ~/.profile ; cd botejao ; export PATH=$PATH:$(pwd) ; cargo run)"
    