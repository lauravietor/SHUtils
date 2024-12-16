CREATE TABLE hunts (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    target INTEGER NOT NULL,
    previous_encounters INTEGER NOT NULL,
    phase_encounters INTEGER NOT NULL,
    phase_count INTEGER NOT NULL,
    start_time TIMESTAMP,
    end_time TIMESTAMP,
    completed TINYINT NOT NULL DEFAULT FALSE,
    version TEXT,
    method TEXT,
    place TEXT,
    notes TEXT
);

CREATE TABLE shinies (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    species INTEGER NOT NULL,
    gender INTEGER,
    name TEXT,
    total_encounters INTEGER,
    phase_encounters INTEGER,
    phase_number INTEGER,
    found_time TIMESTAMP,
    version TEXT,
    method TEXT,
    place TEXT,
    notes TEXT,
    hunt_id INTEGER REFERENCES hunts(id)
);
