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

With our database set up, our next goal is to run the stressor with the following command:

```run
cargo run stress -r
```

After running this command, you may encounter a compilation error similar to:

```
thread 'main' panicked at src/repositories/pet_repository.rs:39:72:
called `Result::unwrap()` on an `Err` value: DbError(SyntaxError, "line 1:0 no viable alternative at input '<EOF>'")
```

Your objective is to implement the missing queries based on the provided data model.

Creating a new Owner
===

The current data model for our "Owner" table is as follows:

```sql,nocopy
CREATE TABLE IF NOT EXISTS carepet.owners
(
    owner_id UUID,
    address TEXT,
    name    TEXT,
    PRIMARY KEY (owner_id)
);
```

Your task is to navigate to the [Editor Tab](tab-2) and create a `INSERT` query inside the `src/repositories/owner_repository.rs` at the line 9.

> [!NOTE]
> Look where the `INSERT_OWNER_QUERY` is being called and try to match the amount of arguments to build your query.

You can test your solution by clicking in **"Check"** or running the feature test in [terminal](tab-0):

```run
cargo test owner_can_be_created
```

Good luck! If you encounter any issues, feel free to open the resolution tab for assistance.

Resolution
===

To resolve the issue, you need to construct your query based on the parameters in the order they are executed around line `~34`. The parameters should be:
- owner_id
- name
- address

The `INSERT` query to add is:

```sql
INSERT INTO owners (owner_id, name, address) VALUES (?, ?, ?)
```

In the repository, it should be defined as:

```rust
const INSERT_OWNER_QUERY: &str =
  r"INSERT INTO owners (owner_id, name, address) VALUES (?, ?, ?)";
```

Let's move on to the next stage!
