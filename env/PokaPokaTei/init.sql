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
    code VARCHAR(4) NOT NULL,
    name VARCHAR(255) NOT NULL,
    prise INT NOT NULL, 
    ishigh boolean NOT NULL,
    deleteflg boolean NOT NULL
);
-- 商品マスタデータ
INSERT INTO m_goods (code,name,prise,ishigh,deleteflg) VALUES ('0001','孫が食ってる給食弁当',500,false,false);
INSERT INTO m_goods (code,name,prise,ishigh,deleteflg) VALUES ('0002','幕の内弁当',400,false,false);
INSERT INTO m_goods (code,name,prise,ishigh,deleteflg) VALUES ('0003','しし丸カレー',300,false,false);
INSERT INTO m_goods (code,name,prise,ishigh,deleteflg) VALUES ('0004','唐揚げ弁当',400,false,false);
INSERT INTO m_goods (code,name,prise,ishigh,deleteflg) VALUES ('0005','ステーキ弁当',1000,true,false);
INSERT INTO m_goods (code,name,prise,ishigh,deleteflg) VALUES ('0006','なまず弁当',800,false,false);
INSERT INTO m_goods (code,name,prise,ishigh,deleteflg) VALUES ('0007','河太郎重',2000,true,false);
INSERT INTO m_goods (code,name,prise,ishigh,deleteflg) VALUES ('0008','キノコご飯',100,false,false);
INSERT INTO m_goods (code,name,prise,ishigh,deleteflg) VALUES ('0009','ウラワライス',500,false,false);


-- 割引マスタ
CREATE TABLE IF NOT EXISTS m_discount (
    id INT AUTO_INCREMENT PRIMARY KEY,
    code  VARCHAR(3) NOT NULL,
    name VARCHAR(255) NOT NULL,
    is_time boolean NOT NULL,
    from_hour INT,
    to_hour INT,
    rate float NOT NULL,
    deleteflg boolean NOT NULL
);

INSERT INTO m_discount (code,name,is_time,from_hour,to_hour,rate,deleteflg) VALUES ('020','タイム割1',true,18,19,0.20,false);
INSERT INTO m_discount (code,name,is_time,from_hour,to_hour,rate,deleteflg) VALUES ('050','タイム割2',true,19,20,0.50,false);

-- マスターFericaカード
CREATE TABLE IF NOT EXISTS m_master_card (
    id INT AUTO_INCREMENT PRIMARY KEY,
    mID  VARCHAR(16) NOT NULL,
    add_mID VARCHAR(16) NOT NULL,
    confurm_mID VARCHAR(16) NOT NULL,
    confirm_auth boolean NOT NULL,
    deleteflg boolean NOT NULL
);

INSERT INTO m_master_card(mID,add_mID,confurm_mID,confirm_auth,deleteflg) VALUES ('2038270024106905','8640899906563105','0522871045567072',true,false);
INSERT INTO m_master_card(mID,add_mID,confurm_mID,confirm_auth,deleteflg) VALUES ('8640899906563105','2038270024106905','0522871045567072',true,false);
INSERT INTO m_master_card(mID,add_mID,confurm_mID,confirm_auth,deleteflg) VALUES ('0522871045567072','2038270024106905','8640899906563105',true,false);
INSERT INTO m_master_card(mID,add_mID,confurm_mID,confirm_auth,deleteflg) VALUES ('0361536805164383','2038270024106905','8640899906563105',false,false);
INSERT INTO m_master_card(mID,add_mID,confurm_mID,confirm_auth,deleteflg) VALUES ('1889364835489365','2038270024106905','8640899906563105',false,false);
INSERT INTO m_master_card(mID,add_mID,confurm_mID,confirm_auth,deleteflg) VALUES ('2233997016630948','2038270024106905','8640899906563105',false,false);
INSERT INTO m_master_card(mID,add_mID,confurm_mID,confirm_auth,deleteflg) VALUES ('7269928561756239','2038270024106905','8640899906563105',false,false);
INSERT INTO m_master_card(mID,add_mID,confurm_mID,confirm_auth,deleteflg) VALUES ('5029769354380365 ','2038270024106905','',false,false);


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
    hour  INT NOT NULL,
    minute  INT NOT NULL,
    shop INT NOT NULL,
    goods VARCHAR(4) NOT NULL,
    barcode VARCHAR(16),
    discount VARCHAR(3) NOT NULL,
    rate float NOT NULL
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

-- ロッカー開閉記録
CREATE TABLE IF NOT EXISTS t_open_log (
    id INT AUTO_INCREMENT PRIMARY KEY,
    shop int NOT NULL,
    open_time  VARCHAR(9),
    open_barcode  VARCHAR(16),
    close_time  VARCHAR(9),
    close_barcode  VARCHAR(16),
    mID  VARCHAR(16) NOT NULL
);