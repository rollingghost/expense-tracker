-- Your SQL goes here
ALTER TABLE pc_stats ADD COLUMN free_swap FLOAT NOT NULL;

ALTER TABLE pc_stats ADD COLUMN total_memory FLOAT NOT NULL;

ALTER TABLE pc_stats ADD COLUMN free_memory FLOAT NOT NULL;

ALTER TABLE pc_stats ADD COLUMN total_disk FLOAT NOT NULL;

ALTER TABLE pc_stats ADD COLUMN free_disk FLOAT NOT NULL;