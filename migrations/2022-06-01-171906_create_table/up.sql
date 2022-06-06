-- Your SQL goes here
CREATE TABLE stage(
  id SERIAL PRIMARY KEY,
  difficulty INTEGER NOT NULL
);

INSERT INTO stage (difficulty) VALUES (1);
INSERT INTO stage (difficulty) VALUES (2);
INSERT INTO stage (difficulty) VALUES (3);
INSERT INTO stage (difficulty) VALUES (5);
INSERT INTO stage (difficulty) VALUES (8);
INSERT INTO stage (difficulty) VALUES (13);
INSERT INTO stage (difficulty) VALUES (21);

CREATE TABLE score(
  uuid TEXT PRIMARY KEY,
  stage_id INTEGER NOT NULL,
  clear_time REAL NOT NULL, 
  user_name TEXT NOT NULL,
  FOREIGN KEY (stage_id) REFERENCES stage(id)
);

INSERT INTO score (stage_id,clear_time,user_name,uuid) VALUES (1,100,'tran0826','328d75a2-9603-fd16-7ea3-00a1f985784b');
