CREATE INDEX idx_games_added_page
ON games(added_at DESC, id DESC);

CREATE INDEX idx_saved_user_page
ON user_saved_games(user_id, saved_at DESC, game_id DESC);
