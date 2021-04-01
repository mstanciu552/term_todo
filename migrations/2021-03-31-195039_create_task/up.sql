-- Your SQL goes here
create table tasks (
  id serial PRIMARY KEY,
  title TEXT not null,
  created_at date , 
  until_at date, 
  in_progress boolean not null default 'f'
);
