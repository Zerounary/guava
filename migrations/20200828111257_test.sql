CREATE TABLE IF NOT EXISTS tests
(
    id          INTEGER PRIMARY KEY NOT NULL,
    username TEXT                NOT NULL,
    done        BOOLEAN             NOT NULL DEFAULT 0
);
