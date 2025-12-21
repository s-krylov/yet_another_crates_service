-- Your SQL goes here
CREATE TABLE crates(
    id SERIAL PRIMARY KEY,
    rustaceans_id INTEGER NOT NULL REFERENCES rustaceans(id),
    code VARCHAR(64) NOT NULL,
    name VARCHAR(128) NOT NULL,
    version VARCHAR(64) NOT NULL,
    description TEXT,
    create_at TIMESTAMP DEFAULT NOW() NOT NULL
);