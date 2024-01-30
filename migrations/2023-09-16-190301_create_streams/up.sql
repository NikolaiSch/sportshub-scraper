CREATE TABLE "stream" (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    home TEXT NOT NULL,
    away TEXT NOT NULL,
    start_time TEXT NOT NULL,
    league TEXT NOT NULL,
    country TEXT NOT NULL,
    UNIQUE(home, away, start_time)
);