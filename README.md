CREATE TABLE scylla_demo.tweets (
tweet_id uuid PRIMARY KEY,
time timeuuid,
user text,
text text
);

create table scylla_demo.timeline (
tweet_id uuid,
time timeuuid,
user text,
author text,
text text,
liked boolean,
PRIMARY KEY (user, time, tweet_id)
) WITH CLUSTERING ORDER BY (time DESC);

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