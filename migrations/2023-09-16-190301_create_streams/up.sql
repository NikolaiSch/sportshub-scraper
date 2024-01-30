CREATE TABLE "stream" (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    home TEXT NOT NULL,
    away TEXT NOT NULL,
    start_time TEXT NOT NULL,
    league TEXT NOT NULL,
    country TEXT NOT NULL,
    url TEXT NOT NULL,
    stream_link TEXT NOT NULL,
    UNIQUE(url, home, away, start_time)
);