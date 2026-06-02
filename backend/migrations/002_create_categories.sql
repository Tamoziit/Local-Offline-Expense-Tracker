CREATE TABLE
    IF NOT EXISTS categories (
        id TEXT PRIMARY KEY,
        name TEXT NOT NULL UNIQUE,
        created_at TEXT NOT NULL DEFAULT (datetime ('now')),
        updated_at TEXT NOT NULL DEFAULT (datetime ('now'))
    );

INSERT
OR IGNORE INTO categories (id, name)
VALUES
    ('zomato', 'Zomato'),
    ('swiggy', 'Swiggy'),
    ('hit_canteen', 'HIT Canteen'),
    ('transport', 'Transport'),
    ('street_food', 'Street Food'),
    ('salary', 'Salary'),
    ('pocket_money', 'Pocket Money'),
    ('youtube', 'Youtube');