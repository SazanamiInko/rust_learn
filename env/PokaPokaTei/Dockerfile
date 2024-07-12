# ベースイメージとしてUbuntuの安定版を使用
FROM ubuntu:20.04

# 環境変数を設定
ENV DEBIAN_FRONTEND=noninteractive
ENV MYSQL_ROOT_PASSWORD=p0kap0ka

# パッケージリストを更新し、必要なパッケージをインストール
RUN apt-get update && \
    apt-get install -y mysql-server && \
    apt-get clean

# MySQLのデフォルト設定を変更（必要に応じて）
RUN sed -i 's/bind-address.*/bind-address = 0.0.0.0/' /etc/mysql/mysql.conf.d/mysqld.cnf

# 初期化スクリプトをコピー
COPY init.sql /init.sql
COPY my.cnf /etc/mysql/conf.d/

RUN chown -R mysql:mysql /path/to/datadir

# MySQLのデータディレクトリを初期化し、初期化スクリプトを実行
RUN service mysql start && \
    mysql -u root -p${MYSQL_ROOT_PASSWORD} < /init.sql && \
    service mysql stop

# MySQLのポートを公開
EXPOSE 3306

# MySQLを起動
CMD ["mysqld"]
