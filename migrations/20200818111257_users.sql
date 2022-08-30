CREATE TABLE IF NOT EXISTS users
(
    id          serial8 PRIMARY KEY NOT NULL,
    username TEXT                NOT NULL,
    done        BOOLEAN             NOT NULL DEFAULT false
);

INSERT INTO users
(id, username, done)
VALUES (1, 'first user', false);