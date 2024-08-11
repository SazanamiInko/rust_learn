# MySQL公式イメージ使用
FROM mysql:8.0

COPY my.cnf /etc/mysql/conf.d/

#　環境変数のセット
ENV MYSQL_ROOT_PASSWORD=p0kap0ka
ENV MYSQL_DATABASE=uriage

# 初期データベーススクリプトをコンテナにコピー
COPY init.sql /docker-entrypoint-initdb.d/

# ポート3306を開放
EXPOSE 3306