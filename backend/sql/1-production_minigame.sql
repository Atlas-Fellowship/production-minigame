CREATE DATABASE production_minigame;
\c production_minigame;

-- Table Structure
-- Primary Key
-- Creation Time
-- Creator User Id (if applicable)
-- Everything else

drop table if exists tournament cascade;
create table tournament(
  tournament_id bigserial primary key,
  creation_time bigint not null default extract(epoch from now()) * 1000,
  creator_user_id bigint not null,
  max_years bigint not null
);

-- invariant: tournament_id is valid
drop table if exists tournament_data cascade;
create table tournament_data(
  tournament_data_id bigserial primary key,
  creation_time bigint not null default extract(epoch from now()) * 1000,
  creator_user_id bigint not null,
  tournament_id bigint not null references tournament(tournament_id),
  -- tournament title
  title text not null,
  -- tournament description
  description text  not null,
  -- is the tournament still visible
  active bool not null
);

create view recent_tournament_data as
  select td.* from tournament_data td
  inner join (
   select max(tournament_data_id) id 
   from tournament_data 
   group by tournament_id
  ) maxids
  on maxids.id = td.tournament_data_id;

drop table if exists tournament_membership cascade;
create table tournament_membership(
  tournament_membership_id bigserial primary key,
  creation_time bigint not null default extract(epoch from now()) * 1000,
  creator_user_id bigint not null,
  tournament_id bigint not null references tournament(tournament_id),
  active bool not null
);

create view recent_tournament_membership as
  select td.* from tournament_membership td
  inner join (
   select max(tournament_membership_id) id 
   from tournament_membership 
   group by tournament_id, creator_user_id
  ) maxids
  on maxids.id = td.tournament_membership_id;


drop table if exists tournament_submission cascade;
create table tournament_submission(
  tournament_submission_id bigserial primary key,
  creation_time bigint not null default extract(epoch from now()) * 1000,
  creator_user_id bigint not null,
  tournament_id bigint not null references tournament(tournament_id),
  year bigint not null,
  amount bigint not null,
  autogenerated bool not null 
);
