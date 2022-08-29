CREATE TABLE IF NOT EXISTS todos
(
    id          INTEGER PRIMARY KEY NOT NULL,
    description TEXT                NOT NULL,
    done        BOOLEAN             NOT NULL DEFAULT 0
);


CREATE TABLE IF NOT EXISTS users
(
    id          INTEGER PRIMARY KEY NOT NULL,
    username TEXT                NOT NULL,
    done        BOOLEAN             NOT NULL DEFAULT 0
);