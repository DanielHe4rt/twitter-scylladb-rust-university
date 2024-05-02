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
INSERT INTO mykeyspace.tweets
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

create materialized view scylla_demo.timeline_liked as select tweet_id,user,time,author,text,liked
from scylla_demo.timeline
where tweet_id is not null and
user is not null and
time is not null and
author is not null and
text is not null and
liked is not null
primary key ((user,liked),time,tweet_id)
WITH CLUSTERING ORDER BY (time DESC);