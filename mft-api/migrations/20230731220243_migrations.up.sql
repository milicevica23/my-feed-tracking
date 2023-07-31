-- Add up migration script here
CREATE SCHEMA mft_raw;

CREATE TABLE mft_raw.feeling (
    id INT,
    name VARCHAR(50)
);

