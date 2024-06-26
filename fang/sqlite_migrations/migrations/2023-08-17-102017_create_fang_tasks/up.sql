-- Your SQL goes here

CREATE TABLE fang_tasks (
    -- uuid will be stored as a 16 byte BLOB
    id BLOB NOT NULL PRIMARY KEY, -- UUID generated inside the language
    metadata TEXT NOT NULL, 
    -- why metadata is text ? https://stackoverflow.com/questions/16603621/how-to-store-json-object-in-sqlite-database#16603687
    error_message TEXT,
    state TEXT CHECK ( state IN ('new', 'in_progress', 'failed', 'finished', 'retried') ) NOT NULL DEFAULT 'new',
    -- why state is a text ? https://stackoverflow.com/questions/5299267/how-to-create-enum-type-in-sqlite#17203007
    task_type TEXT NOT NULL DEFAULT 'common',
    uniq_hash CHAR(64),
    retries INTEGER NOT NULL DEFAULT 0,
    -- scheduled_at TEXT NOT NULL DEFAULT (CURRENT_TIMESTAMP || '.000000+00'),

    -- Timestamps are stored as the number of seconds since the Unix epoch ('1970-01-01 00:00:00 UTC').

    scheduled_at INTEGER NOT NULL DEFAULT (unixepoch('now')),
    created_at INTEGER NOT NULL DEFAULT (unixepoch('now')),
    updated_at INTEGER NOT NULL DEFAULT (unixepoch('now'))
);

CREATE INDEX fang_tasks_state_index ON fang_tasks(state);
CREATE INDEX fang_tasks_type_index ON fang_tasks(task_type);
CREATE INDEX fang_tasks_scheduled_at_index ON fang_tasks(scheduled_at);
CREATE INDEX fang_tasks_uniq_hash ON fang_tasks(uniq_hash);