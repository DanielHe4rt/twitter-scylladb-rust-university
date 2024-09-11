---
slug: wrapping-up
id: uawntv7w0uzu
type: challenge
title: 'Wrapping Up: How better is our app?'
teaser: Our app is missing a couple of queries and we're gonna implement it!
notes:
- type: text
  contents: Loading challenge...
tabs:
- id: jfzclryj8xx6
  title: Terminal
  type: terminal
  hostname: scylladb
  cmd: bash
- id: 8t6lo4s9y1kq
  title: Scylla Monitoring
  type: service
  hostname: scylladb
  path: /
  port: 3000
- id: x00mwdhpcslf
  title: Editor
  type: code
  hostname: scylladb
  path: /root/university/
difficulty: basic
timelimit: 2400
---

## Summary

To summarize, you saw what happens when your driver is not being properly used and how bad can be your app performance. 

Let's do a quick wrap up: 

1. **Reduced Latency with DC-Aware Routing**:
   - Queries will be efficiently routed to the nearest data center, minimizing cross-DC traffic and improving overall response time.

2. **Increased Query Efficiency via Prepared Statements**:
   - All queries will be utilizing Prepared Statements, reducing the overhead of query parsing and compilation, leading to faster execution times.

3. **Optimized Consistency Levels for Multi-DC**:
   - Consistency levels will be fine-tuned, improving performance by reducing the strictness of `QUORUM` where unnecessary, while still maintaining data integrity.

4. **Improved Cluster Performance with Paged Queries**:
   - Large datasets will be fetched in manageable chunks, preventing unpaged queries from overloading the cluster, which results in better resource utilization.

5. **Elimination of Reverse Queries**:
   - Reverse clustering order queries will be identified and removed, reducing the processing burden on the database and improving read performance.

6. **Removal of ALLOW FILTERING Queries**:
   - Queries relying on `ALLOW FILTERING` will be refactored, eliminating slow scans and enhancing cluster-wide performance.

7. **Clear and Optimized Monitoring Metrics**:
   - Grafana dashboards will show zero or minimal values in critical gauges, confirming that performance bottlenecks have been resolved and the cluster is running at optimal efficiency.

By achieving these optimizations, your ScyllaDB cluster will be better equipped to handle high query volumes, reduce latency, and maintain efficient resource management, all while ensuring a smooth user experience.

Don't forget to visit other courses at [Scylla University](https://university.scylladb.com) and rate this lab at the next page!