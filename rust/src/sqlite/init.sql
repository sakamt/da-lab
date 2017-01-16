CREATE TABLE IF NOT EXISTS enkf (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  K INTEGER NOT NULL,
  tau INTEGER NOT NULL,
  count INTEGER NOT NULL,
  r REAL NOT NULL,
  dt REAL NOT NULL,
  truth_id INTEGER NOT NULL,
  observation_id INTEGER NOT NULL,
  ensemble_id INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS truth (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  table_name TEXT NOT NULL,
  dt REAL NOT NULL
);

CREATE TABLE IF NOT EXISTS observation (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  table_name TEXT NOT NULL,
  dt REAL NOT NULL,
  truth_id INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS ensemble (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  table_name TEXT NOT NULL,
  dt REAL NOT NULL,
  K INTEGER NOT NULL,
  truth_id INTEGER NOT NULL,
  observation_id INTEGER NOT NULL
);