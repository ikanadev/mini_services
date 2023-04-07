-- Add up migration script here
create table if not exists counters (
  id uuid primary key,
  project varchar,
  entity varchar,
  count varchar
);
