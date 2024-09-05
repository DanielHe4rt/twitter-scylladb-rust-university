---
slug: create-multi-node-cluster
id: bymiwpe7q7lw
type: challenge
title: Setup Environment
teaser: Just a test.
notes:
- type: video
  url: https://www.youtube.com/embed/xWJSNCjilVQ
tabs:
- id: trzxmthygrr1
  title: Terminal
  type: terminal
  hostname: scylladb
  cmd: bash
- id: xsj0fcjnyrqj
  title: Scylla Monitoring
  type: service
  hostname: scylladb
  path: /
  port: 3000
- id: ztbnubl3bw76
  title: Editor
  type: code
  hostname: scylladb
  path: university/
difficulty: ""
---

Welcome to the IoT (CarePet) project! This lab is a hands-on project where you will implement the queries by yourself and make the app work properly.

For that scenario, we're gonna spin a three-node cluster with Docker and start by migrating the schema to our cluster.

Starting the Clusters
===

Run ScyllaDB in Docker.  Run this command in the [terminal](tab-0) tab:

```run
cd university

```

To check the status of your node, copy and run this command in the [terminal](tab-0) tab:
```run
docker exec -it carepet-node1 nodetool status
```

The response you get back should be UN (Up and Normal). If it is not then wait a few second and try again as the node is not ready yet.

If your nodes are "UN", you're ready to go to the next stage.
