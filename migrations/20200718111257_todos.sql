CREATE TABLE IF NOT EXISTS todos
(
    id          serial8 PRIMARY KEY NOT NULL,
    description TEXT                NOT NULL,
    done        BOOLEAN             NOT NULL DEFAULT false
);


CREATE TABLE IF NOT EXISTS users
(
    id          serial8 PRIMARY KEY NOT NULL,
    username TEXT                NOT NULL,
    done        BOOLEAN             NOT NULL DEFAULT false
);