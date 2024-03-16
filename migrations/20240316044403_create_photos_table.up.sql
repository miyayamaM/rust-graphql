-- Add up migration script here

CREATE TABLE photos (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255),
    url VARCHAR(255),
    description text,
    posted_by_user_id int,
    CONSTRAINT fk_posted_by_user_id
        FOREIGN KEY (posted_by_user_id)
        REFERENCES users (id)
        ON DELETE RESTRICT ON UPDATE RESTRICT
    )
