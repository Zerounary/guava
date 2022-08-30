CREATE TABLE IF NOT EXISTS tests
(
    id          serial8 PRIMARY KEY NOT NULL,
    username TEXT                NOT NULL,
    done        BOOLEAN             NOT NULL DEFAULT false
);
