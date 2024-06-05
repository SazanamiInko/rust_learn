-- init.sql
ALTER USER 'root'@'localhost' IDENTIFIED WITH 'mysql_native_password' BY 'p0kap0ka';

-- DBの設定
CREATE DATABASE IF NOT EXISTS uriage CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;
USE uriage;

-- 店マスタ
CREATE TABLE IF NOT EXISTS m_shop (
    id INT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    iscity boolean NOT NULL,
    deleteflg boolean NOT NULL
);
-- 店マスタデータ
INSERT INTO m_shop (name,iscity,deleteflg) VALUES ('大芦よしきり店',false,false);
INSERT INTO m_shop (name,iscity,deleteflg) VALUES ('富士見つる店',false,false);
INSERT INTO m_shop (name,iscity,deleteflg) VALUES ('加美きじ店',false,false);
INSERT INTO m_shop (name,iscity,deleteflg) VALUES ('笠原じゅうしまつ店',true,false);
INSERT INTO m_shop (name,iscity,deleteflg) VALUES ('御成橋めじろ店',true,false);

-- 商品マスタ
CREATE TABLE IF NOT EXISTS m_goods (
    id INT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    ishigh boolean NOT NULL,
    deleteflg boolean NOT NULL
);
-- 商品マスタデータ
INSERT INTO m_goods (name,ishigh,deleteflg) VALUES ('孫が食ってる給食弁当',false,false);
INSERT INTO m_goods (name,ishigh,deleteflg) VALUES ('幕の内弁当',false,false);
INSERT INTO m_goods (name,ishigh,deleteflg) VALUES ('しし丸カレー',false,false);
INSERT INTO m_goods (name,ishigh,deleteflg) VALUES ('唐揚げ弁当',false,false);
INSERT INTO m_goods (name,ishigh,deleteflg) VALUES ('ステーキ弁当',true,false);
INSERT INTO m_goods (name,ishigh,deleteflg) VALUES ('なまず弁当',false,false);
INSERT INTO m_goods (name,ishigh,deleteflg) VALUES ('河太郎重',true,false);
INSERT INTO m_goods (name,ishigh,deleteflg) VALUES ('キノコご飯',false,false);
INSERT INTO m_goods (name,ishigh,deleteflg) VALUES ('ウラワライス',false,false);


-- 発注
CREATE TABLE IF NOT EXISTS t_arrival (
    id INT AUTO_INCREMENT PRIMARY KEY,
    year INT NOT NULL,
    month INT NOT NULL,
    day  INT NOT NULL,
    shop INT NOT NULL,
    goods INT NOT NULL,
    arrival_num INT NOT NULL
);

-- 売り上げ
CREATE TABLE IF NOT EXISTS t_sale (
    id INT AUTO_INCREMENT PRIMARY KEY,
    year INT NOT NULL,
    month INT NOT NULL,
    day  INT NOT NULL,
    shop INT NOT NULL,
    goods INT NOT NULL,
    rate INT NOT NULL
);

-- 集計
CREATE TABLE IF NOT EXISTS t_summary (
    id INT AUTO_INCREMENT PRIMARY KEY,
    year INT NOT NULL,
    month INT NOT NULL,
    day  INT NOT NULL,
    shop INT NOT NULL,
    goods INT NOT NULL,
    num INT NOT NULL,
    avg_rate INT NOT NULL,
    disposal INT NOT NULL
);