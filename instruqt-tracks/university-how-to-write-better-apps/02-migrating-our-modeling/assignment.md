---
slug: migrating-our-modeling
id: ug7y3pozerrh
type: challenge
title: Migrating the Challenge Keyspace
teaser: Let's understand our data modeling and run our migrations.
notes:
- type: text
  contents: Loading challenge...
tabs:
- id: gay9z6k2pfys
  title: Terminal
  type: terminal
  hostname: scylladb
  cmd: bash
- id: bu5lbtiyrlv2
  title: Scylla Monitoring
  type: service
  hostname: scylladb
  path: /
  port: 3000
- id: korv9n0hnbxy
  title: Editor
  type: code
  hostname: scylladb
  path: carepet-rust/
difficulty: basic
timelimit: 600
---

Now that we have the clusters running, we can jump into some code! Switch to the [Code Editor](tab-2) and let's take a look around.

Project Overview
===

The project was built on top of the [Rust Driver](https://rust-driver.docs.scylladb.com/), which 4 major features:
* migrate - Setup your schema inside the Cluster
* serve - Serve a http server for you work with.
* sensor - A pet collar running and collecting data
* stressor - Stress test with the application.

At this lab, we're gonna be using only two of them, which is `migrate` and `stressor`.

The entire project runs inside a *keyspace* called **carepet**. This keyspace has a **replication factor of 3** and will hold all tables inside the project.

```sql,nocopy
CREATE KEYSPACE IF NOT EXISTS carepet
  WITH replication = {
    'class': 'NetworkTopologyStrategy',
    'replication_factor': '3'
  };
```

Since we want to keep it simple, we decided to work with a few tables which is:
- **owners** :  Pet owner entity
- **pets** :  Pet related information by owner
- **sensors** :  Sensors enabled by a specific pet (Temperature, Heartbeat, etc)
- **measurements** : Sensor data compacted with *TimeWindowCompactionStrategy*
- **sensor_avg** :  Average Sensor data by date and hour compacted with *TimeWindowCompactionStrategy*

> [!NOTE]
>  If you want to check the full migrations before coding something, open the *src/database/migrate/migrate.cql*.


Running the migrations
===

Let's run the migrate command and to deploy the project _keyspace_ inside our cluster:

```run
cargo run migrate
```

This should be the expected output:

```text,nocopy
[INFO  care_pet::database] Connecting to localhost:9042
[INFO  care_pet::database::migrate] Keyspace carepet created
[INFO  care_pet::database::migrate] Creating tables...
[INFO  care_pet::database::migrate] Migration completed!
```

> [!NOTE]
> At the moment, the only feature that fully works in the project is the `migrate` command and your main goal will be `populating the missing queries` inside the repositories folder.


Now, we're ready to code fix our codebase.