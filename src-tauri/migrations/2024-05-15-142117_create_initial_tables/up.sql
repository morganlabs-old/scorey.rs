CREATE TABLE team (
    id INTEGER PRIMARY KEY NOT NULL,
    name VARCHAR(32) UNIQUE NOT NULL,
    individual BOOLEAN NOT NULL,
    points INTEGER NOT NULL DEFAULT 0,

    CHECK(length(name) >= 4),
    CHECK(length(name) <= 32)
);

CREATE TABLE participant (
    id INTEGER PRIMARY KEY NOT NULL,
    team_id INTEGER NOT NULL,
    first_name VARCHAR(32) NOT NULL,
    last_name VARCHAR(322) NOT NULL,

    CHECK(length(last_name) >= 2),
    CHECK(length(first_name) >= 2),
    CHECK(length(first_name) <= 32),
    CHECK(length(last_name) <= 32),
    FOREIGN KEY (team_id) REFERENCES team(id)
);

CREATE TABLE event (
    id INTEGER PRIMARY KEY NOT NULL,
    name VARCHAR(32) UNIQUE NOT NULL,
    event_type VARCHAR(8) NOT NULL,

    CHECK(event_type IN ('SPORT', 'ACADEMIC')),
    CHECK(length(name) >= 4),
    CHECK(length(name) <= 32)
);

CREATE TABLE event_entry (
    team_id INTEGER NOT NULL,
    event_id INTEGER NOT NULL,
    PRIMARY KEY (team_id, event_id),
    FOREIGN KEY (team_id) REFERENCES team(id),
    FOREIGN KEY (event_id) REFERENCES event(id)
);
