---
slug: setup-your-environment
id: bymiwpe7q7lw
type: challenge
title: 'How to Write Better Apps: Introducing the Challenge and Setup the Environment'
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
  path: /root/university/
difficulty: basic
---

Welcome to the `How To Write Better Apps Challenge`  project! This lab is a hands-on project where you will apply all your new skills to improve your CQL health.

For that scenario, we're gonna spin a `Multi Datacenter Cluster` with `docker-compose` and start by migrating the schema to our cluster.

The Challenge
===

Your goal is to fix all `gauges` inside `Scylla Monitoring > CQL Dashboard`, which is related to:

- Enable the DC-Aware Queries
- Fix all Non-Prepared Statements in the code
- Turn all queries to work under Cross-DC
- Fix all Non-Paged Queries
- Validate if there's any Reverse CQL Queries running
- Denormalize any ALLOW FILTERING in the code

> [!TIP]
> Tip 1: For each change in the code, don't forget to save it.
> Tip 2: For each test run using the `instruqt`  or `make` it will take 30 seconds to collect the metrics for compare.

Now, let's setup our environment and begin our challenge.


Starting the Clusters
===

We prepared a `Makefile` with commands to help you setup your environment.

Run ScyllaDB in Docker.  Run this command in the [terminal](tab-0) tab:

```run
make setup
```

To check the status of your node, you can run the `nodetool` [terminal](tab-0) tab:

```run
make nodetool
```

The response you get back should be UN (Up and Normal). If it is not then wait a few second and try again as the node is not ready yet.

See if you can find something like this:

```log
Datacenter: EU-DC
=================
Status=Up/Down
|/ State=Normal/Leaving/Joining/Moving
-- Address  Load Tokens Owns Host ID                              Rack
UN 10.9.8.5 ?    256    ?    c6b99168-9cb7-466e-9633-2c58f8d839a3 RC2 
UN 10.9.8.7 ?    256    ?    c1934b87-5ee3-43a3-985e-83e626602ed9 RC2 
UN 10.9.8.9 ?    256    ?    bb27d363-7be4-4c8e-bf28-f991171130f1 RC2 
Datacenter: SA-DC
=================
Status=Up/Down
|/ State=Normal/Leaving/Joining/Moving
-- Address  Load      Tokens Owns Host ID                              Rack
UN 10.9.8.4 738.57 KB 256    ?    7f1e5622-94de-4357-8f8a-2e348616d32c RC1 
UN 10.9.8.6 ?         256    ?    625909f4-4715-4877-baca-820eb82afd79 RC1 
UN 10.9.8.8 724.30 KB 256    ?    d78353f8-4832-46a8-bd9f-3e4d290a437f RC1 
```

> [!NOTE]
> Remember that you're running Multi-DC named `EU-DC` and `SA-DC`. ;)

If your nodes are "UN", you're ready to go to the next stage.
