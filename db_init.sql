CREATE TYPE game_status AS ENUM ('lobby', 'playing', 'game_over');

CREATE TABLE IF NOT EXISTS colors (
    id  UUID PRIMARY KEY DEFAULT uuidv4(),
    name VARCHAR(12) UNIQUE NOT NULL
);

CREATE TABLE IF NOT EXISTS ships (
    id  UUID PRIMARY KEY DEFAULT uuidv4(),
    name VARCHAR(255) UNIQUE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
    character CHAR UNIQUE NOT NULL
);

CREATE TABLE IF NOT EXISTS players (
    id UUID PRIMARY KEY DEFAULT uuidv4(),
    name VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
    token UUID NOT NULL DEFAULT uuidv4(),
    ship_id UUID NOT NULL REFERENCES ships (id) DEFAULT '7959eef9-8e62-4cbe-a3da-8cb2abaa7d8c',
    color_id UUID NOT NULL DEFAULT 'c7bb5e85-1e66-4df3-95d2-f37fb5498d63'
);

CREATE TABLE IF NOT EXISTS games (
    id  UUID PRIMARY KEY DEFAULT uuidv4(),
    status  game_status  NOT NULL DEFAULT 'lobby',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
    code INT NOT NULL UNIQUE DEFAULT random(1000, 9999),
    host_id UUID NOT NULL REFERENCES players (id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS game_players (
    game_id UUID NOT NULL REFERENCES games (id) ON DELETE CASCADE,
    player_id UUID NOT NULL REFERENCES players (id) ON DELETE CASCADE,
    CONSTRAINT game_players_primary_key PRIMARY KEY (game_id, player_id)
);

INSERT INTO ships (id, name, character) VALUES ('7959eef9-8e62-4cbe-a3da-8cb2abaa7d8c', 'Reliable', '>');
INSERT INTO ships (name, character) VALUES ('Serpent', 'S');
INSERT INTO ships (name, character) VALUES ('Intrepid', '@');
INSERT INTO colors (id, name) VALUES ('c7bb5e85-1e66-4df3-95d2-f37fb5498d63', 'red');
INSERT INTO colors (name) VALUES ('green');
INSERT INTO colors (name) VALUES ('yellow');
INSERT INTO colors (name) VALUES ('blue');
INSERT INTO colors (name) VALUES ('magenta');
INSERT INTO colors (name) VALUES ('cyan');
