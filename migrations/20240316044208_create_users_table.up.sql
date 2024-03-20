-- Add up migration script here
CREATE TYPE country AS ENUM ('japan', 'china', 'usa');

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    email VARCHAR(255),
    name VARCHAR(255),
    country country
);

