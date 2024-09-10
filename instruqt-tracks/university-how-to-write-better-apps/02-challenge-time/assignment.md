---
slug: challenge-time
id: ug7y3pozerrh
type: challenge
title: 'Challenge Accepted: Optimize all CQL Gauges'
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
  path: /root/university/
difficulty: basic
timelimit: 3600
---

Seems that we have our cluster running, now it's time to run our migrations:

```run
make migrate
```

This will create all `Tables and Materialized Views` needed using [Charybdis ORM](https://github.com/nodecosmos/charybdis) and you can check it all under `src/models` folder.


If you're an CLI person, use the `make test` to run the projectt ests or just click in **check** at Instruqt UI.

If you want to check the performance and see some gauges popping off, you can start the project by running: 

```run
cargo run
```

So, let's get started.

First Challenge: DC-Aware
===
To optimize the performance of your CQL queries, enable the DC-Aware options in the ScyllaDB Rust driver. This allows the driver to automatically route queries to the nearest data center, reducing latency and improving overall performance.

> [!INFO]
> - Configure DC-Aware Routing in your ScyllaDB driver.
> - Use Grafana's **ScyllaDB Metrics Dashboard** to monitor query latency and confirm that queries are routed to the closest data center.
> - Make sure to check if your **connection** is DC-Aware.



Second Challenge: Enforce Prepared Statements Usage
===
Prepared Statements reduce query compilation overhead, improving performance. Ensure that all queries are using Prepared Statements to avoid unnecessary strain on the cluster.

> [!INFO]
> - Use Grafana’s **Query Metrics Dashboard** to monitor the `scylla_query_processor_statements_prepared` metric.
> - Identify unprepared queries, like `get_timeline_by_username`, and ensure they are converted to Prepared Statements.



Third Challenge: Optimize Consistency Level for Multi-DC
===
In a multi-data center (Multi-DC) environment, adjusting the consistency level can improve performance. Using `QUORUM` might be too strict, so explore less restrictive consistency levels like `LOCAL_QUORUM`.

> [!INFO]
> - Track the **Consistency Level Metrics** in Grafana using the `scylla_query_processor_queries{consistency_level='QUORUM'}` metric.
> - Analyze your query patterns and adjust the consistency levels where necessary for Multi-DC performance optimization.



Fourth Challenge: Avoid Unpaged Queries
===
Fetching large datasets without pagination can overwhelm the cluster. Ensure all SELECT statements use proper pagination to handle large volumes of data.

> [!INFO]
> - Implement a page size in all SELECT queries and verify the improvements using the Grafana dashboard.


Fifth Challenge: Eliminate Reverse Queries
===
Reverse clustering order queries are inefficient and can cause performance degradation. Identify and remove these queries by denormalizing your data model or restructuring the queries.

> [!INFO]
> - If you're tracking reverse queries, use Grafana to identify performance issues related to reverse clustering order.
> - Restructure queries or modify your schema to eliminate the need for reverse ordering if any of them is present at the code.


Sixth Challenge: Remove ALLOW FILTERING Queries
===
Queries using `ALLOW FILTERING` can negatively affect the performance of the entire cluster by scanning too much data. Eliminate these queries by redesigning them or creating appropriate indexes.

> [!INFO]
> - Monitor the `scylla_cql_filtered_read_requests` metric in Grafana to identify queries using `ALLOW FILTERING`.
> - Refactor your queries or schema, leveraging denormalization or indexing to remove the need for `ALLOW FILTERING`.



Final Check: Monitor and Verify Gauge Values
===
After making the necessary optimizations, use the Grafana dashboards to verify that all related gauges reflect improvements. If everything is optimized, gauge values for problematic behaviors (e.g., non-prepared queries) should be close to zero.

> [!INFO]
> - Regularly check the **ScyllaDB Grafana Monitoring Stack** to ensure your changes are reflected in the gauges. Key metrics to monitor include prepared statement usage, consistency level, unpaged queries, and `ALLOW FILTERING` behaviors.
> - Fine-tune any remaining issues based on real-time Grafana metrics and feedback.

