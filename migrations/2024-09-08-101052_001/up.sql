-- Your SQL goes here

CREATE TABLE IF NOT EXISTS stats (
    id INTEGER SERIAL PRIMARY KEY,
    pc_host VARCHAR(255) NOT NULL,
    swap_total FLOAT NOT NULL,
    free_swap FLOAT NOT NULL,
    total_storage FLOAT NOT NULL,
    free_storage FLOAT NOT NULL
);