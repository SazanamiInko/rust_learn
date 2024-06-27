# ベースイメージとしてUbuntu 24.04を使用
FROM ubuntu:22.04

# 非対話型でのインストールを設定
ENV DEBIAN_FRONTEND=noninteractive

# パッケージリストを更新し、必要なツールをインストール
RUN apt-get update && apt-get install -y \
    curl \
    wget \
    unzip \
    software-properties-common \
    apt-transport-https \
    ca-certificates \
    gnupg \
    build-essential\
    nginx

# Javaのインストール (OpenJDK 11)
RUN apt-get install -y openjdk-11-jdk

# Rustのインストール
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# 環境変数を設定
ENV PATH="/root/.cargo/bin:${PATH}"

# 確認のためにバージョン情報を表示 (オプション)
RUN java -version
RUN rustc --version
RUN nginx -v

COPY nginx.conf /etc/nginx/nginx.conf

# 作業ディレクトリを設定
WORKDIR /workspace

ARG USER_NAME=devuser
ARG USER_ID=1000
ARG GROUP_ID=1000

# エントリーポイント（シェルを起動）
CMD service nginx start && bash
