-- Your SQL goes here
create table tasks (
  id SERIAL PRIMARY KEY,
  title TEXT NOT NULL,
  created_at DATE DEFAULT SELECT CAST( GETDATE() AS Date ) ,
  until_at DATE 
);
