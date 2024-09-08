-- Your SQL goes here
CREATE TABLE IF NOT EXISTS expenses (
    id INTEGER SERIAL PRIMARY KEY,
    description TEXT NOT NULL,
    amount FLOAT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    update_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE if NOT EXISTS budget (
    id INTEGER PRIMARY KEY,
    january FLOAT DEFAULT 0.0,
    february FLOAT DEFAULT 0.0,
    march FLOAT DEFAULT 0.0,
    april FLOAT DEFAULT 0.0,
    may FLOAT DEFAULT 0.0,
    june FLOAT DEFAULT 0.0,
    july FLOAT DEFAULT 0.0,
    august FLOAT DEFAULT 0.0,
    september FLOAT DEFAULT 0.0,
    october FLOAT DEFAULT 0.0,
    november FLOAT DEFAULT 0.0,
    december FLOAT DEFAULT 0.0
);

CREATE TABLE IF NOT EXISTS stats (
    id INTEGER SERIAL PRIMARY KEY,
    host VARCHAR(255) NOT NULL,
    swap_total FLOAT NOT NULL,
    free_swap FLOAT NOT NULL,
    total_storage FLOAT NOT NULL,
    free_storage FLOAT NOT NULL
)