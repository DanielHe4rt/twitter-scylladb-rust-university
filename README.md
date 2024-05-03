````cassandraql
CREATE KEYSPACE mykeyspace WITH replication = 
    {'class': 'NetworkTopologyStrategy', 'DC1' : 3, 'DC2' : 3}
AND durable_writes = true;

CREATE KEYSPACE mykeyspace WITH replication =
    {'class': 'NetworkTopologyStrategy', 'replication_factor' : 3}
AND durable_writes = true;
````

````
ALTER KEYSPACE system_auth WITH replication = { 'class' : 'NetworkTopologyStrategy', 'DC1' : 3, 'DC2' : 3};
ALTER KEYSPACE system_distributed WITH replication = { 'class' : 'NetworkTopologyStrategy', 'DC1' : 3, 'DC2' : 3};
ALTER KEYSPACE system_traces WITH replication = { 'class' : 'NetworkTopologyStrategy', 'DC1' : 3, 'DC2' : 3};

````


````cassandraql
CREATE TABLE scylla_demo.tweets
(
    tweet_id uuid PRIMARY KEY,
    time     timeuuid,
    user     text,
    text     text
);
````

````cassandraql
create table scylla_demo.timeline
(
    tweet_id uuid,
    time     timeuuid,
    user     text,
    author   text,
    text     text,
    liked    boolean,
    PRIMARY KEY (user, time, tweet_id)
) WITH CLUSTERING ORDER BY (time DESC);
````

````cassandraql
INSERT INTO tweets
    (tweet_id, author, text, time, created_at)
VALUES 
    ({}, '{}', '{}', now(), {}); 
````

````cassandraql
SELECT 
    username,
    tweet_id,
    author,
    text,
    liked,
    created_at
FROM 
    scylla_demo.timeline
WHERE 
    username = '{}'
````

`````cassandraql
create materialized view timeline_liked as
select tweet_id, username, author, author, text, liked, bookmarked, retweeted, created_at
from timeline
where tweet_id is not null
  and username is not null
  and created_at is not null
  and liked is not null
primary key ((username, liked), created_at, tweet_id)
WITH CLUSTERING ORDER BY (created_at DESC);
`````