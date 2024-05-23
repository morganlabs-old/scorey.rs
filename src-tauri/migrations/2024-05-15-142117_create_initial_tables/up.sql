CREATE TABLE team (
    id INTEGER PRIMARY KEY NOT NULL,
    name VARCHAR(255) UNIQUE NOT NULL,
    individual BOOLEAN NOT NULL,
    points INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE participant (
    id INTEGER PRIMARY KEY NOT NULL,
    team_id INTEGER NOT NULL,
    first_name VARCHAR(255) NOT NULL,
    last_name VARCHAR(255) NOT NULL,
    FOREIGN KEY (team_id) REFERENCES team(id)
);

CREATE TABLE event (
    id INTEGER PRIMARY KEY NOT NULL,
    name VARCHAR(255) UNIQUE NOT NULL,
    event_type VARCHAR(8) CHECK (event_type IN ('SPORT', 'ACADEMIC')) NOT NULL
);

CREATE TABLE event_entry (
    team_id INTEGER NOT NULL,
    event_id INTEGER NOT NULL,
    PRIMARY KEY (team_id, event_id)
    FOREIGN KEY (team_id) REFERENCES team(id)
    FOREIGN KEY (event_id) REFERENCES event(id)
);
