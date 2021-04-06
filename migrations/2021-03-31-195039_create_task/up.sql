-- Your SQL goes here
create table tasks (
  id serial PRIMARY KEY,
  title TEXT not null,
  created_at date not null default Now(), 
  until_at date default null, 
  in_progress boolean  default 'f'
);
