CREATE TYPE game_status AS ENUM ('lobby', 'playing', 'game_over');

CREATE TABLE IF NOT EXISTS players (
    id UUID PRIMARY KEY DEFAULT uuidv4(),
    name VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS games (
    id  UUID PRIMARY KEY DEFAULT uuidv4(),
    status  game_status  NOT NULL DEFAULT 'lobby',
    created_by_id UUID REFERENCES players (id) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
    code INT NOT NULL UNIQUE DEFAULT random(1000, 9999)
);

CREATE TABLE IF NOT EXISTS game_players (
    game_id UUID NOT NULL REFERENCES games (id),
    player_id UUID NOT NULL REFERENCES players (id),
    CONSTRAINT game_players_primary_key PRIMARY KEY (game_id, player_id)
)
