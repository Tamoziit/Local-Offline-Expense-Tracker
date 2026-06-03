CREATE TABLE
    IF NOT EXISTS transactions (
        id TEXT PRIMARY KEY,
        title TEXT NOT NULL,
        description TEXT,
        category_id TEXT REFERENCES categories (id) ON DELETE RESTRICT,
        transaction_type TEXT NOT NULL CHECK (
            transaction_type IN ('expense', 'income', 'transfer')
        ),
        transaction_mode TEXT NOT NULL CHECK (transaction_mode IN ('online', 'cash')),
        is_recurring INTEGER NOT NULL DEFAULT 0 CHECK (is_recurring IN (0, 1)),
        from_account_id TEXT REFERENCES accounts (id) ON DELETE RESTRICT,
        to_account_id TEXT REFERENCES accounts (id) ON DELETE RESTRICT,
        amount REAL NOT NULL CHECK (amount > 0),
        transaction_date TEXT NOT NULL DEFAULT (date ('now')),
        created_at TEXT NOT NULL DEFAULT (strftime ('%Y-%m-%dT%H:%M:%fZ', 'now')),
        updated_at TEXT NOT NULL DEFAULT (strftime ('%Y-%m-%dT%H:%M:%fZ', 'now'))
    );

CREATE INDEX idx_transactions_date ON transactions (transaction_date);

CREATE INDEX idx_transactions_category ON transactions (category_id);

CREATE INDEX idx_transactions_type ON transactions (transaction_type);

CREATE INDEX idx_transactions_recurring ON transactions (is_recurring);

CREATE INDEX idx_transactions_mode ON transactions (transaction_mode);

CREATE INDEX idx_transactions_from_account ON transactions (from_account_id);

CREATE INDEX idx_transactions_to_account ON transactions (to_account_id);