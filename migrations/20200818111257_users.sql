CREATE TABLE IF NOT EXISTS users
(
    id          INTEGER PRIMARY KEY NOT NULL,
    username TEXT                NOT NULL,
    done        BOOLEAN             NOT NULL DEFAULT 0
);

INSERT INTO users
(id, username, done)
VALUES (1, 'first user', false);