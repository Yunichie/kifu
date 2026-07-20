ALTER TABLE user_saved_games
ADD COLUMN is_public INTEGER NOT NULL DEFAULT 0 CHECK (is_public IN (0, 1));

CREATE INDEX idx_saved_public_page
ON user_saved_games(is_public, saved_at DESC, game_id DESC, user_id DESC);
