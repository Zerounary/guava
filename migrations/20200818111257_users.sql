CREATE TABLE IF NOT EXISTS users
(
    id          INTEGER PRIMARY KEY NOT NULL,
    username TEXT                NOT NULL,
    done        BOOLEAN             NOT NULL DEFAULT 0
);