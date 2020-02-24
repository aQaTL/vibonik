-- Your SQL goes here

create table users (
    id serial primary key,
    fb_id text unique not null,
    access_token text,

    login text,
    password_hash text,
    role text,

    name text not null,

    pesel bigint,
    email text,
    birthday date,
    gender char(1),
    food_preferences text,

    paid bigint NOT NULL DEFAULT 0
);