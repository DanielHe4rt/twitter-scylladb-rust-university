---
slug: fixing-pet-insertion
id: cmkmqpfza71p
type: challenge
title: '2nd task: Adding New Pet Query'
teaser: Our app is missing a couple of queries and we're gonna implement it!
notes:
- type: text
  contents: Loading challenge...
tabs:
- id: ugczhek9u8ni
  title: Terminal
  type: terminal
  hostname: scylladb
  cmd: bash
- id: bclqktulvn59
  title: Scylla Monitoring
  type: service
  hostname: scylladb
  path: /
  port: 3000
- id: uflumo3mih1p
  title: Editor
  type: code
  hostname: scylladb
  path: carepet-rust/
difficulty: ""
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

Creating a new Pet
===

The current data model for our "Pet" table is as follows:

```sql,nocopy
CREATE TABLE IF NOT EXISTS carepet.pets
(
    owner_id UUID,
    pet_id   UUID,
    chip_id  TEXT,
    species  TEXT,
    breed    TEXT,
    color    TEXT,
    gender   TEXT,
    age      INT,
    weight   FLOAT,
    address  TEXT,
    name     TEXT,
    PRIMARY KEY (owner_id, pet_id)
);
```

Your task is to navigate to the [Editor Tab](tab-2) and create a `INSERT` query inside the `src/repositories/pet_repository.rs` at the line 9.

> [!NOTE]
> Look where the `INSERT_PET_QUERY` is being called and try to match the amount of arguments to build your query. Since it's a sample app, it was built with minimal resources. So feel free to do your implementations if you want.

You can test your solution by clicking in **"Check"** or running the feature test in [terminal](tab-0):

```run
cargo test pet_can_be_created
```

Good luck! If you encounter any issues, feel free to open the resolution tab for assistance.

Resolution
===

To resolve the issue, you need to construct your query based on the parameters in the order they are executed around line `~51`. The parameters should be:
- owner_id
- pet_id

The `INSERT` query to add is:

```sql
INSERT INTO pets (owner_id, pet_id) VALUES (?, ?)
```

In the repository, it should be defined as:

```rust
const INSERT_PET_QUERY: &str =
  "INSERT INTO pets (owner_id, pet_id) VALUES (?, ?)";
```

Let's move on to the next stage!
