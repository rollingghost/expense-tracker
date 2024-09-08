-- Your SQL goes here
CREATE TABLE IF NOT EXISTS pc_stats (
    stat_id VARCHAR(255) PRIMARY KEY,
    total_swap FLOAT NOT NULL
);