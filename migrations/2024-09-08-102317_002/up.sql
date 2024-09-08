-- Your SQL goes here
CREATE TABLE IF NOT EXISTS stats (
    stat_id INTEGER PRIMARY KEY,
    total_swap FLOAT NOT NULL,
    free_swap FLOAT NOT NULL
);