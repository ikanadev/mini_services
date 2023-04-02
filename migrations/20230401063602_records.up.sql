create table if not exists records (
  id uuid primary key,
  game_level varchar,
  started_at bigint,
  name varchar,
  duration integer,
  game_status varchar,
  mines_count integer,
  flagged varchar,
  opened varchar,
  board varchar[]
);
