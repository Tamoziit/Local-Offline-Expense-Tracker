CREATE TABLE
    IF NOT EXISTS accounts (
        id TEXT PRIMARY KEY,
        name TEXT NOT NULL UNIQUE,
        account_type TEXT NOT NULL CHECK (account_type IN ('personal', 'external')),
        balance REAL NOT NULL DEFAULT 0.0,
        to_receive REAL DEFAULT 0.0,
        to_give REAL DEFAULT 0.0,
        created_at TEXT NOT NULL DEFAULT (strftime ('%Y-%m-%dT%H:%M:%fZ', 'now')),
        updated_at TEXT NOT NULL DEFAULT (strftime ('%Y-%m-%dT%H:%M:%fZ', 'now'))
    );

CREATE INDEX idx_accounts_type ON accounts (account_type);