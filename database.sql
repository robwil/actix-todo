DROP TABLE IF EXISTS todo_list;
DROP TABLE IF EXISTS todo_item;

CREATE TABLE todo_list (
    id serial primary key,
    title varchar(150) not null
);

CREATE TABLE todo_item (
    id serial primary key,
    title varchar(150) not null,
    checked boolean not null default false,
    list_id integer not null,
    foreign key (list_id) references todo_list(id)
);

INSERT INTO todo_list (title) VALUES ('List 123'), ('List ABC');
INSERT INTO todo_item (title, list_id)
    VALUES ('item 1', 1), ('Item 2', 1), ('Item A', 2);