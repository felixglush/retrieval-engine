# Storage And Indexing Plan

## Purpose

This document covers how records are stored and how retrieval-specific indexes are built from them.

Storage is the source of truth for records.
Indexes are optimized views used to answer retrieval queries efficiently.

---

## Recommended V1 Storage Approach

For version 1, use a simple in-memory primary store with a clean interface.

That means:

- records live in memory while the engine is running
- the engine keeps a direct lookup by a record key made from namespace, collection, and id
- the engine maintains secondary groupings by namespace and collection
- lexical and vector indexes are built from the same stored records

This is the best first implementation because it is:

- easy to reason about
- fast to build
- easy to test
- a good fit for tens of thousands of records in a single-node system

It also avoids introducing database complexity before the retrieval logic is proven.

---

## What "Storage" Means In V1

The storage layer is responsible for:

- inserting records
- retrieving records by composite record key
- listing records by namespace and collection
- updating or replacing existing records
- providing the source data used to build indexes

In v1, storage does not need to be a full database.

---

## Likely Internal Shape

The storage component will probably keep:

- a primary map from `record_key -> record`
- a namespace-to-record-key grouping
- a collection-to-record-key grouping
- possibly a combined namespace-and-collection grouping for faster scans

This keeps lookup simple while supporting filter checks and index maintenance.

The record key should not be just `id`.
The model plan now assumes records are upserted by `id` within a namespace and collection, which means the storage layer must distinguish:

- `workspace_a / products / prod_123`
- `workspace_b / products / prod_123`

even though they share the same `id` value.

In the implementation, this composite identity should be represented by a first-class `RecordKey` type rather than ad hoc joined strings.

---

## Why Start In Memory

For this project, in-memory storage is a strong first step because:

- the architecture is single-node
- the scale target is tens of thousands of records, not millions
- the bigger learning challenge is retrieval logic, not persistence plumbing
- it keeps the Rust code easier to understand while you are learning

Once retrieval behavior works, persistence can be added with much less risk.

---

## Persistence Strategy After The First Milestone

After the in-memory version works, add one of these:

### Option 1: Snapshot Persistence

Periodically write records and indexes to local files.

Benefits:

- very simple
- easy to inspect during development
- low implementation overhead

Tradeoff:

- weaker update guarantees

### Option 2: Append-Only Log Plus Rebuild

Write record changes to a local log, then rebuild indexes on startup.

Benefits:

- simple mental model
- better durability story than pure memory

Tradeoff:

- slower startup as data grows

### Option 3: Embedded Database Later

Move the record store into an embedded database after the engine design settles.

Benefits:

- stronger persistence
- cleaner long-term storage foundation

Tradeoff:

- more implementation and operational complexity

For now, the recommendation is:

1. in-memory record store first
2. optional snapshot or append-only persistence second
3. embedded database only if v1 proves the need

---

## Indexing Responsibilities

Indexes should be derived from storage, not treated as the primary source of truth.

The indexing layer is responsible for:

- tokenizing record content for lexical search
- storing token-to-record mappings
- storing embedding vectors for semantic search
- updating indexes when records are inserted or changed

This separation matters because it keeps record correctness and retrieval performance concerns distinct.

---

## Lexical Index Shape

The lexical index will likely be an inverted index.

At a high level, that means:

- take each token from a record
- map that token to the record keys containing it

This makes keyword lookup much faster than scanning all records.

---

## Vector Index Shape

The vector index will likely store:

- record key
- embedding vector

In the first version, vector retrieval can compare the query embedding against all stored embeddings and compute similarity directly.

That is usually called exact vector search.

It is slower than ANN methods, but it is much easier to implement and validate.

The vector index should also enforce the embedding dimension rules defined in the model plan:

- embeddings are optional on records
- if present, they must match the configured dimension from shared engine configuration for the searchable scope
- queries with mismatched dimensions should fail before similarity scoring runs

---

## Interfaces To Stabilize Early

Before coding deeply, define stable operations for:

- upsert record
- get record by key
- list records for a namespace or collection
- update indexes from a record
- delete or replace a record

If these interfaces are clean, later persistence changes will be much easier.

The important alignment with the model layer is:

- upserts are keyed by namespace, collection, and id
- replacements must update both lexical and vector indexes
- delete semantics, if added in v1, should use the same composite record key

---

## Rust Learning Notes

This part of the project will likely introduce these Rust concepts:

- `HashMap<K, V>`: a key-value store, useful for record lookup and token indexes
- ownership of stored values: important because the store owns the records it keeps
- references: useful when reading records without copying them
- traits: a way to define shared behavior for different storage implementations

A `trait` in Rust is similar to an interface in other languages.
It lets us define operations like "insert" and "get" without tying the whole system to one storage implementation too early.
