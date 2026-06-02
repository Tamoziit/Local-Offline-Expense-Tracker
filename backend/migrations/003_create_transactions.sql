CREATE TABLE
    IF NOT EXISTS transactions (
        id TEXT PRIMARY KEY,
        title TEXT NOT NULL,
        description TEXT,
        category_id TEXT NOT NULL REFERENCES categories (id) ON DELETE RESTRICT,
        type TEXT NOT NULL CHECK (type IN ('expense', 'income', 'transfer')),
        from_account_id TEXT REFERENCES accounts (id) ON DELETE RESTRICT,
        to_account_id TEXT REFERENCES accounts (id) ON DELETE RESTRICT,
        amount REAL NOT NULL CHECK (amount > 0),
        transaction_date TEXT NOT NULL DEFAULT (date ('now')),
        created_at TEXT NOT NULL DEFAULT (datetime ('now')),
        updated_at TEXT NOT NULL DEFAULT (datetime ('now'))
    );

CREATE INDEX idx_transactions_date ON transactions (transaction_date);

CREATE INDEX idx_transactions_category ON transactions (category_id);

CREATE INDEX idx_transactions_type ON transactions (type);

CREATE INDEX idx_transactions_from_account ON transactions (from_account_id);

CREATE INDEX idx_transactions_to_account ON transactions (to_account_id);