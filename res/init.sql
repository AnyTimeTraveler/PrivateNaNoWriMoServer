DROP TABLE IF EXISTS User;
DROP TABLE IF EXISTS LoginCookies;
DROP TABLE IF EXISTS Story;
DROP TABLE IF EXISTS Wordcount;

CREATE TABLE User
(
    id       INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL,
    email    TEXT NOT NULL
);

CREATE TABLE LoginCookies
(
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id         INTEGER,
    expiration_date INTEGER NOT NULL,
    token_value     TEXT    NOT NULL,
    FOREIGN KEY (user_id) REFERENCES User (id)
);

CREATE TABLE Story
(
    id            INTEGER PRIMARY KEY AUTOINCREMENT,
    name          TEXT NOT NULL,
    user_id       INTEGER,
    creation_date TEXT NOT NULL,
    upload_url    TEXT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES User (id)
);

CREATE TABLE Wordcount
(
    story_id INTEGER,
    date     TEXT    NOT NULL,
    count    INTEGER NOT NULL,
    FOREIGN KEY (story_id) REFERENCES Story (id)
);

insert into User (username, email)
VALUES ('simon', 'simon@simonscode.org');
