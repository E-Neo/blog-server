drop table if exists authors cascade;
create table authors (
        id serial primary key,
        email varchar(100) not null unique,
        name varchar(100) not null,
        hashed_password varchar(100) not null
        );

drop table if exists blogs;
create table blogs (
        slug varchar(100) not null unique primary key,
        author_id int not null references authors(id),
        title varchar(512) not null,
        markdown text not null,
        published timestamp with time zone not null
        );

drop table if exists tweets;
create table tweets (
        slug varchar(100) not null unique primary key,
        author_id int not null references authors(id),
        markdown text not null,
        published timestamp with time zone not null
        );

drop table if exists pages;
create table pages (
        slug varchar(100) not null unique primary key,
        html text not null
        );

drop table if exists proxy_white;
create table proxy_white (
        url text not null
        );
