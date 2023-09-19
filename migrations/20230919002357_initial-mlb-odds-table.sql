-- Add migration script here
CREATE TABLE [IF NOT EXISTS] mlb_today (
    home_team VARCHAR(50) NOT NULL,
    away_team VARCHAR(50) NOT NULL,
    -- over/under
    over_line FLOAT,
    over_odds INTEGER,
    under_line FLOAT,
    under_odds INTEGER,
    -- spread
    home_spread_line FLOAT,
    home_spread_odds INTEGER,
    away_spread_line FLOAT,
    away_team_odds INTEGER,
    -- moneyline
    home_moneyline_odds INTEGER,
    away_moneyline_odds INTEGER,
);
