create table user_entity(
    id bigserial primary key,
    version bigint not null,
    identifier UUID not null,
    first_name varchar(255) not null,
    last_name varchar(255) not null,
    user_id varchar(255) not null,
    admin boolean not null
);

create unique index UK_UserEntity_Identifier on user_entity (identifier);

create unique index UK_UserEntity_UserId on user_entity (user_id);

/* uuid extension */
create extension if not exists "uuid-ossp";