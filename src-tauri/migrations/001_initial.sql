-- Top-level notebooks or nested ones
CREATE TABLE notebooks (
    id           TEXT PRIMARY KEY,
    parent_id    TEXT REFERENCES notebooks(id) ON DELETE CASCADE,
    title        TEXT NOT NULL,
    icon         TEXT,
    sort_order   INTEGER DEFAULT 0,
    created_at   DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE notes (
    id           TEXT PRIMARY KEY,
    notebook_id  TEXT NOT NULL REFERENCES notebooks(id) ON DELETE CASCADE,
    title        TEXT NOT NULL,
    markdown     TEXT NOT NULL DEFAULT '',
    priority     INTEGER DEFAULT 0,
    date         DATE,
    created_at   DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at   DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_notes_notebook ON notes(notebook_id);

CREATE TABLE tags (
    id   TEXT PRIMARY KEY,
    name TEXT UNIQUE NOT NULL
);

CREATE TABLE note_tags (
    note_id TEXT REFERENCES notes(id) ON DELETE CASCADE,
    tag_id  TEXT REFERENCES tags(id)   ON DELETE CASCADE,
    PRIMARY KEY (note_id, tag_id)
);

CREATE INDEX idx_note_tags_note ON note_tags(note_id);

-- Version history for each note
CREATE TABLE versions (
    id         TEXT PRIMARY KEY,
    note_id    TEXT NOT NULL REFERENCES notes(id) ON DELETE CASCADE,
    markdown   TEXT NOT NULL,
    saved_at   DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Special "Inbox" note
INSERT INTO notebooks (id, title, icon, sort_order)
VALUES ('inbox_nb', 'Inbox', '📥', 0);

INSERT INTO notes (id, notebook_id, title, markdown)
VALUES ('inbox', 'inbox_nb', 'Inbox', 'Quick-capture goes here…');
