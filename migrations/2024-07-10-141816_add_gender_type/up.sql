-- Your SQL goes here
DROP TYPE IF EXISTS Gender;

CREATE TYPE Gender As ENUM(
    'male', -- 男性
    'female' -- 女性
);