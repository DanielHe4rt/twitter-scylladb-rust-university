<p align="center">
  <p align="center">
  <img src=".github/images/scylla-university.png" alt="Logo" width="120">
  </p>
  <h2 align="center"> ScyllaDB University Twitter Lab </h2>

  <p align="center">
    <a href="https://university.scylladb.com/courses/data-modeling/lessons/how-to-write-better-apps/">
        <strong>« Explore the University Course »</strong>
    </a>
    <br />
    <a href="https://github.com/scylladb/scylla-cloud-getting-started/issues/new">Report Bug or Request Feature</a>
  </p>
</p>
<hr>

<center>
This project is designed to help you to go through the  "How to Write Better Apps" section by showing how you improve your app performance using <a href="https://github.com/scylladb/scylladb"> ScyllaDB </a> .
</center>

## Table of Contents

- [About the Project](#about-the-project)
- [Tooling](#tooling)
- [Getting Started](#getting-started)
    - [Step 1: Non-Prepared Statements and Non-Token Aware Requests](#step-1-non-optimized-app)
    - [Step 2: Prepared Statements and Token Aware Requests](#step-2-optimized-app)
    - [Step 3: Bad Usage of Allow Filtering](#step-3-bad-usage-of-allow-filtering)
    - [Step 4: Solving with Materialized Views](#step-4-solving-with-materialized-views)
    - [Step 5: Going Multi-DC](#step-5-going-multi-dc)
    - [Step 6: Going Multi-DC with Local Quorum](#step-6-going-multi-dc-with-local-quorum)
    - [Step 7: Reverse CQL Query](#step-7-reverse-cql-query)

## About the Project

This project demonstrates different optimization techniques and features of ScyllaDB through a series of incremental
steps, starting with a simple application and progressively optimizing it using various ScyllaDB features.

## Tooling

- **Rust**: The programming language used for the application.
- **Docker**: Used to run ScyllaDB in a containerized environment.
- **Scylla Monitoring Stack**: Used to monitor and visualize ScyllaDB metrics with Grafana and Prometheus.

## Getting Started

In this lab, we're going to use two different ScyllaDB clusters. Until the `step 5` the environment will be using
Single-DC, and after that we have to change to Multi-DC.

To make it easier, there's a `docker` folder where you can find all the related configurations, but you can also use the
`make` commands to start the environment.

```bash
# Start the Single DC environment
make start-single-dc

# Create and migrate the schema
make new-keyspace
make migrate
```

And now you're ready to start the lab.

> [!TIP]
> Check the `Makefile` to see all the available commands.

### Step 1: Non-Prepared Statements and Non-Token Aware Requests

In this initial step, the application performs basic operations without any optimizations. It does not use prepared
statements, and all requests are non-token aware, leading to inefficient database access patterns.

**Files to Watch:**

- `lessons/step1/src/repositories/timeline_service.rs`: Basic CRUD operations without Prepared Statements.
- `lessons/step1/src/repositories/tweet_service.rs`: Basic CRUD operations without Prepared Statements.
- `lessons/step1/src/connection.rs`: Non-token aware connection settings.

### Step 2: Prepared Statements and Token Aware Requests

In this step, we introduce prepared statements and token-aware requests, which improve performance by reducing the
overhead of query parsing and making efficient use of the cluster's topology.

**Files Changed:**

- `lessons/step2/src/repositories/timeline_service.rs`: Prepared Statements implemented.
- `lessons/step2/src/repositories/tweet_service.rs`: Prepared Statements implemented.
- `lessons/step2/src/connection.rs`: Token-Aware Requests enabled (by default).

### Step 3: Bad Usage of Allow Filtering

This step demonstrates the improper use of the `ALLOW FILTERING` clause, which can lead to inefficient queries and poor
performance. It highlights the risks associated with its misuse.

**Files to Watch:**

- `lessons/step3/src/repositories/timeline_service.rs`: Query `SELECT_LIKED_TIMELINE_QUERY` uses `ALLOW FILTERING`.

### Step 4: Solving with Materialized Views

To address the issues from Step 3, this step introduces materialized views. Materialized views allow for efficient
querying without the need for `ALLOW FILTERING`, improving query performance and consistency.

```sh
# Run the migration to create the materialized view
make timeline-liked-mv
```

**Files Changed:**

- `lessons/step4/src/repositories/timeline_service.rs`: Updated to use materialized views.

### Step 5: Going Multi-DC without proper setup

This step demonstrates how to configure the application for a multi-datacenter (Multi-DC) setup, ensuring data
availability and redundancy across multiple geographic locations.

With a Multi-DC setup, the application can be configured to read and write data from different datacenters, but without
the proper driver configurations, the application will not be able to take advantage of the Multi-DC setup.

```sh
# Changing the environment to Multi-DC 
make multi-dc
```

**Files to Watch:**

- `lessons/step5/src/connection.rs`: Still running in Single-DC mode.

### Step 6: Going Multi-DC with Local Quorum

Building on Step 5, this step introduces the concept of local quorum consistency. This approach ensures that read and
write operations are acknowledged by a majority of replicas within a local datacenter, improving performance and
availability. Also, you will be editing the `connection.rs` file to use the Multi-DC setup.

**Files Changed:**

- `lessons/step5/src/connection.rs`: Updated to use Multi-DC setup with LocalQuorum.

### Step 7: Reverse CQL Query

In the final step, we introduce reverse CQL queries to efficiently retrieve data in descending order based on clustering
keys. This optimization is useful for applications requiring reverse-ordered data retrieval.

The problem is related reverting the timeline query to get the latest tweets first, and it can be solved by creating a
new materialized view:

```sh 
# Run the migration to create the materialized view
make first-tweets-mv
```

**Files Changed:**

- `lessons/step4/src/repositories/timeline_service.rs`: Updated to use materialized views.

## License

Distributed under the MIT License. See `LICENSE` for more information.
