CREATE TABLE
    IF NOT EXISTS accounts (
        id TEXT PRIMARY KEY,
        name TEXT NOT NULL UNIQUE,
        balance REAL NOT NULL DEFAULT 0.0,
        created_at TEXT NOT NULL DEFAULT (datetime ('now')),
        updated_at TEXT NOT NULL DEFAULT (datetime ('now'))
    );