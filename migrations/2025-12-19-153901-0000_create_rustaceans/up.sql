-- Your SQL goes here
create table rustaceans(
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    email VARCHAR NOT NULL,
    create_at TIMESTAMP DEFAULT NOW() NOT NULL
);