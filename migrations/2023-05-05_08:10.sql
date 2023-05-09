-- we don't know how to generate root <with-no-name> (class Root) :(
create table user
(
    id         int auto_increment
        primary key,
    name       varchar(255) not null,
    created_at datetime(3)  not null,
    updated_at datetime(3)  not null
);

create table todo
(
    id           int auto_increment
        primary key,
    title        varchar(255)                             not null,
    contents     varchar(1000)                            null,
    status       enum ('Draft', 'Ready', 'Doing', 'Done') not null,
    started_date date                                     null,
    ended_date   date                                     null,
    `rank`       decimal(65, 30)                          not null,
    user_id      int                                      not null,
    updated_at   datetime(3)                              not null,
    created_at   datetime(3)                              not null,
    constraint todo_user_id_fk
        foreign key (user_id) references user (id)
);

