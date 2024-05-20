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
    username   text,
    tweet_id   uuid,
    created_at timeuuid,
    author     text,
    text       text,
    liked      boolean,
    bookmarked boolean,
    retweeted  boolean,
    PRIMARY KEY (username, created_at, tweet_id)
) WITH CLUSTERING ORDER BY (created_at DESC);
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

SELECT username, tweet_id, author, text, liked, bookmarked, retweeted, created_at FROM timeline_liked  WHERE 
    username = 'danielhe4rt' ORDER BY created_at DESC ALLOW FILTERING ;

create materialized view timeline_liked as
select tweet_id, username, author, author, text, liked, bookmarked, retweeted, created_at
from timeline
where tweet_id is not null
  and username is not null
  and created_at is not null
  and liked is not null
primary key ((username, liked), created_at, tweet_id)
WITH CLUSTERING ORDER BY (created_at DESC);

CREATE MATERIALIZED VIEW first_timeline_tweets AS
SELECT tweet_id, username, author, author, text, created_at
FROM timeline
WHERE tweet_id IS NOT null
  AND username IS NOT null
  AND created_at IS NOT null
PRIMARY KEY (username, created_at, tweet_id)
WITH CLUSTERING ORDER BY (created_at ASC);
`````