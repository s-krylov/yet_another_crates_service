-- Your SQL goes here
CREATE TABLE users_roles(
    id SERIAL PRIMARY KEY,
    users_id integer NOT NULL REFERENCES users(id),
    roles_id integer NOT NULL REFERENCES roles(id),
    create_at TIMESTAMP DEFAULT NOW() NOT NULL
);