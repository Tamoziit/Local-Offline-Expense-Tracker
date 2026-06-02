CREATE TABLE
    IF NOT EXISTS recurring (
        id TEXT PRIMARY KEY,
        title TEXT NOT NULL,
        type TEXT NOT NULL CHECK (type IN ('expense', 'income', 'transfer')),
        amount REAL NOT NULL,
        frequency TEXT NOT NULL CHECK (
            frequency IN (
                'daily',
                'weekly',
                'monthly',
                'quarterly',
                'yearly'
            )
        ),
        start_date TEXT NOT NULL,
        end_date TEXT,
        active INTEGER NOT NULL DEFAULT 1,
        category_id TEXT NOT NULL REFERENCES categories (id) ON DELETE RESTRICT,
        from_account_id TEXT REFERENCES accounts (id),
        to_account_id TEXT REFERENCES accounts (id)
    );