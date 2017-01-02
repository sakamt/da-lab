CREATE TABLE enkf (
  K INTEGER NOT NULL,
  T INTEGER NOT NULL,
  r REAL NOT NULL,
  dt REAL NOT NULL,
  tau REAL NOT NULL,
  ensemble TEXT,
  truth TEXT,
  observable TEXT
);

CREATE TABLE mpf (
  K INTEGER NOT NULL,
  T INTEGER NOT NULL,
  M INTEGER NOT NULL,
  r REAL NOT NULL,
  dt REAL NOT NULL,
  tau REAL NOT NULL,
  ensemble TEXT,
  truth TEXT,
  observable TEXT
);
