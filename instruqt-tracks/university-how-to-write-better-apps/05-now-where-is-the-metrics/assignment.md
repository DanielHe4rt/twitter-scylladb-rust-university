---
slug: now-where-is-the-metrics
id: yu1agsjfxxrj
type: challenge
title: Fixed! Now where is the metrics?
teaser: Our app is finally running as expected, but how we can track how good is it?
notes:
- type: text
  contents: Loading challenge...
tabs:
- id: qfcxkgtwvffw
  title: Terminal
  type: terminal
  hostname: scylladb
  cmd: bash
- id: cagtn9mowozr
  title: Scylla Monitoring
  type: service
  hostname: scylladb
  path: /
  port: 3000
- id: ftcvwxjcxww7
  title: Editor
  type: code
  hostname: scylladb
  path: carepet-rust/
difficulty: ""
timelimit: 2400
---

Congratulations! You passed in all tests! Now let's run the stressor and see some metrics.

The command now will run 20 threads which will inserting data inside our ScyllaDB Cluster.
```run
cargo run stress -r -w 20
```

You can check the metrics at [Scylla Monitoring Tab](tab-1) and play with the stressor.

> [!NOTE]
> This is a development environment, where you learn the basics about ScyllaDB and Rust Driver.
> If you're looking for production metrics, please visit our [benchmark section](https://www.scylladb.com/product/benchmarks/).


Don't forget to follow us in socials and if you have any issue with this project, open a [issue](https://github.com/scylladb/care-pet).

See you in the next lab!