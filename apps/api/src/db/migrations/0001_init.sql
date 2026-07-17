CREATE TABLE games (
  id            INTEGER PRIMARY KEY AUTOINCREMENT,
  log_id        TEXT NOT NULL UNIQUE,
  added_at      INTEGER NOT NULL,
  ruleset_json  TEXT NOT NULL,
  detail_json   TEXT NOT NULL
);

CREATE TABLE game_players (
  game_id       INTEGER NOT NULL REFERENCES games(id) ON DELETE CASCADE,
  seat          INTEGER NOT NULL CHECK (seat BETWEEN 0 AND 3),
  player_name   TEXT NOT NULL,
  final_score   INTEGER,
  placement     INTEGER CHECK (placement BETWEEN 1 AND 4),
  wins          INTEGER NOT NULL DEFAULT 0,
  riichi        INTEGER NOT NULL DEFAULT 0,
  deal_ins      INTEGER NOT NULL DEFAULT 0,
  calls         INTEGER NOT NULL DEFAULT 0,
  hands         INTEGER NOT NULL DEFAULT 0,
  PRIMARY KEY (game_id, seat)
);

CREATE INDEX idx_game_players_name ON game_players(player_name COLLATE NOCASE);

CREATE TABLE users (
  id               INTEGER PRIMARY KEY AUTOINCREMENT,
  provider         TEXT NOT NULL,
  provider_subject TEXT NOT NULL,
  display_name     TEXT NOT NULL,
  created_at       INTEGER NOT NULL,
  UNIQUE (provider, provider_subject)
);

CREATE TABLE user_player_names (
  user_id       INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  player_name   TEXT NOT NULL,
  claimed_at    INTEGER NOT NULL,
  PRIMARY KEY (user_id, player_name)
);

CREATE TABLE user_saved_games (
  user_id       INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  game_id       INTEGER NOT NULL REFERENCES games(id) ON DELETE CASCADE,
  saved_at      INTEGER NOT NULL,
  PRIMARY KEY (user_id, game_id)
);
