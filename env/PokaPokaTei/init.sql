-- init.sql
CREATE DATABASE IF NOT EXISTS my_database;
USE my_database;
CREATE TABLE IF NOT EXISTS test_table (
    id INT AUTO_INCREMENT PRIMARY KEY,
    value VARCHAR(255) NOT NULL
);
INSERT INTO test_table (value) VALUES ('Hello, world!');