CREATE TYPE game_status AS ENUM ('lobby', 'playing', 'game_over');

CREATE TABLE IF NOT EXISTS colors (
    id  UUID PRIMARY KEY DEFAULT uuidv4(),
    name VARCHAR(12) UNIQUE NOT NULL
);

CREATE TABLE IF NOT EXISTS ships (
    id  UUID PRIMARY KEY DEFAULT uuidv4(),
    name VARCHAR(255) UNIQUE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
    character CHAR UNIQUE NOT NULL,
    max_speed INT NOT NULL DEFAULT 10,
    max_torpedo_count INT NOT NULL DEFAULT 10
);

CREATE TABLE IF NOT EXISTS players (
    id UUID PRIMARY KEY DEFAULT uuidv4(),
    name VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
    token UUID NOT NULL DEFAULT uuidv4(),
    ship_id UUID NOT NULL REFERENCES ships (id) DEFAULT '7959eef9-8e62-4cbe-a3da-8cb2abaa7d8c',
    color_id UUID NOT NULL DEFAULT 'c7bb5e85-1e66-4df3-95d2-f37fb5498d63',
    ready BOOL NOT NULL DEFAULT false,
    position_x INT,
    position_y INT,
    speed INT NOT NULL DEFAULT 0,
    torpedo_count INT NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS games (
    id  UUID PRIMARY KEY DEFAULT uuidv4(),
    status  game_status  NOT NULL DEFAULT 'lobby',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
    code INT NOT NULL UNIQUE DEFAULT random(1000, 9999),
    host_id UUID NOT NULL REFERENCES players (id) ON DELETE CASCADE,
    width INT NOT NULL DEFAULT 40,
    height INT NOT NULL DEFAULT 20
);

CREATE TABLE IF NOT EXISTS game_players (
    game_id UUID NOT NULL REFERENCES games (id) ON DELETE CASCADE,
    player_id UUID NOT NULL REFERENCES players (id) ON DELETE CASCADE,
    CONSTRAINT game_players_primary_key PRIMARY KEY (game_id, player_id)
);

CREATE TABLE IF NOT EXISTS game_turns (
    id UUID PRIMARY KEY DEFAULT uuidv4(),
    game_id UUID NOT NULL REFERENCES games (id),
    turn_number INT NOT NULL DEFAULT 1,
    active BOOL NOT NULL DEFAULT true
);

CREATE TABLE IF NOT EXISTS player_turns (
    id UUID PRIMARY KEY DEFAULT uuidv4(),
    player_id UUID NOT NULL REFERENCES players (id),
    game_turn_id UUID NOT NULL REFERENCES game_turns (id),
    speed_change INT NOT NULL,
    destination_x INT,
    destination_y INT
);

INSERT INTO ships (id, name, character, max_speed, max_torpedo_count ) VALUES ('7959eef9-8e62-4cbe-a3da-8cb2abaa7d8c', 'Reliable', '>', 10, 6);
INSERT INTO ships (name, character, max_speed, max_torpedo_count ) VALUES ('Serpent', 'S', 15, 3);
INSERT INTO ships (name, character, max_speed, max_torpedo_count ) VALUES ('Intrepid', '@', 5, 15);
INSERT INTO colors (id, name) VALUES ('c7bb5e85-1e66-4df3-95d2-f37fb5498d63', 'red');
INSERT INTO colors (name) VALUES ('green');
INSERT INTO colors (name) VALUES ('yellow');
INSERT INTO colors (name) VALUES ('blue');
INSERT INTO colors (name) VALUES ('magenta');
INSERT INTO colors (name) VALUES ('cyan');
