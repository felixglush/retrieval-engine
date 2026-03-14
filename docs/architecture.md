# System Architecture

## Overview

The system is a hybrid retrieval engine combining:

- vector similarity search
- lexical keyword search
- metadata filtering
- rank fusion

The goal is to provide a simple retrieval backend for AI systems.

---

## Related Planning Docs

Detailed planning docs for the main component groups live alongside this architecture document:

- `docs/model-plan.md`
- `docs/storage-and-indexing-plan.md`
- `docs/retrieval-plan.md`
- `docs/api-plan.md`
- `docs/implementation-roadmap.md`

---

## High-Level Architecture

```text
           Client
             |
             v
        API Layer
             |
             v
    Retrieval Planner
             |
  +----------+----------+
  |          |          |
  v          v          v
Vector    Lexical     Filter
Search     Search     Engine
  |          |          |
  +----------+----------+
             |
             v
        Rank Fusion
             |
             v
          Results
```

---

## Core Components

### Record Store

Stores records and metadata.

Each record contains:

```text
id
namespace
collection
content
embedding
metadata
created_at
importance
```

Example structure:

```rust
struct Record {
    id: String,
    namespace: String,
    collection: String,
    content: String,
    embedding: Option<Vec<f32>>,
    metadata: MetadataMap,
    created_at: String,
    importance: Option<f32>,
}
```

---

### Vector Search

Provides semantic similarity search.

Capabilities:

- embedding similarity
- cosine similarity scoring
- approximate nearest neighbor (future)

Vector search generates candidate records.

---

### Lexical Search

Provides keyword search.

Capabilities:

- tokenized text indexing
- keyword matching
- BM25 ranking (future)

This component enables exact term retrieval.

---

### Metadata Filtering

Filters candidate records using metadata fields.

Examples:

```text
workspace_id = "abc"
category = "dress"
price < 200
```

Filtering occurs before ranking.

---

### Rank Fusion

Combines signals from multiple sources.

Example scoring function:

```text
score =
  alpha * semantic_score
+ beta * lexical_score
+ gamma * freshness_score
+ delta * importance_score
```

Weights are configurable.

---

## V1 Component Diagram

This is a more concrete breakdown of the version 1 system.

```text
Client
  |
  v
API / Engine Facade
  |
  v
Query Parser + Retrieval Coordinator
  |
  +-------------------+-------------------+
  |                   |                   |
  v                   v                   v
Lexical Search     Vector Search      Filter Engine
  |                   |                   |
  +---------+---------+---------+---------+
            |                   |
            v                   v
         Candidate Merge    Namespace / Collection Checks
                    \         /
                     v       v
                      Rank Fusion
                           |
                           v
                        Top-K Results

Supporting layers used throughout:
- record storage
- index maintenance
- configuration
- logging / metrics
```

---

## V1 Component Responsibilities

### API / Engine Facade

The top-level entry point used by clients.

Responsibilities:

- accept indexing requests
- accept retrieval requests
- return normalized results
- hide internal subsystem details

This should feel like the public surface of the engine.

### Query Parser + Retrieval Coordinator

Turns a client query into work for the retrieval subsystems.

Responsibilities:

- validate query inputs
- decide whether lexical search should run
- decide whether vector search should run
- pass namespace and collection constraints through the pipeline
- merge candidate sets before final ranking

This is the control layer of the engine.

### Record Storage

Stores source records and exposes them for indexing and retrieval.

Responsibilities:

- persist records
- load records by id
- scan records by collection / namespace when needed
- provide a stable base for indexes

In v1, this can stay simple as long as the interface is clean.

### Index Maintenance

Keeps retrieval data structures in sync with stored records.

Responsibilities:

- build lexical indexes from content
- store vector-ready embedding data
- update indexes when records are inserted or changed

This is separate from query execution so indexing logic does not leak into retrieval logic.

### Lexical Search

Handles keyword-based retrieval.

Responsibilities:

- tokenize query text
- match terms against indexed content
- produce candidate ids with lexical scores

V1 should favor a simple implementation before more advanced ranking methods.

### Vector Search

Handles semantic retrieval over embeddings.

Responsibilities:

- compare query embeddings with record embeddings
- compute similarity scores
- return top semantic candidates

V1 can use exact similarity scoring before adding ANN structures later.

### Filter Engine

Evaluates structured constraints.

Responsibilities:

- apply metadata predicates
- enforce namespace restrictions
- enforce collection restrictions

This prevents retrieval components from each needing custom filter logic.

### Candidate Merge

Combines results from lexical and vector retrieval.

Responsibilities:

- union candidate ids
- preserve subsystem scores
- prepare records for final scoring

### Rank Fusion

Produces one final ranking across all candidates.

Responsibilities:

- combine lexical score
- combine semantic score
- incorporate freshness and importance when available
- sort and trim to top-k

### Configuration

Provides tunable settings without changing engine logic.

Examples:

- ranking weights
- candidate pool sizes
- collection-specific settings
- default top-k values

### Logging and Metrics

Makes the engine easier to debug and evaluate.

Examples:

- query latency
- number of lexical candidates
- number of vector candidates
- number of filtered results
- final result count

---

## Retrieval Pipeline

Query execution follows these steps:

1. parse query
2. generate vector candidates
3. generate lexical candidates
4. apply metadata filters
5. merge candidates
6. compute ranking score
7. return top-k results

---

## Suggested Rust Crate / Module Layout

For a first Rust version, keep everything in a single crate and organize by module.

In Rust, a `crate` is the buildable package or library, and a `module` is a named code section used to group related types and functions. This layout keeps the project easy to navigate without introducing multi-crate complexity too early.

Example layout:

```text
retrieval-engine/
  Cargo.toml
  src/
    lib.rs
    model/
      mod.rs
      metadata.rs
      record.rs
      record_key.rs
      query.rs
      filter.rs
      result.rs
    storage/
      mod.rs
      store.rs
    index/
      mod.rs
      lexical.rs
      vector.rs
    retrieval/
      mod.rs
      coordinator.rs
      merge.rs
      rank.rs
    api/
      mod.rs
      engine.rs
    config.rs
    error.rs
```

### Why this layout fits v1

- `model/` holds the shared domain types used everywhere
- `storage/` keeps persistence concerns isolated
- `index/` groups the retrieval-specific data structures
- `retrieval/` owns the query execution flow
- `api/` provides the public surface clients call into
- `config.rs` and `error.rs` stay top-level because nearly every module will use them

### Suggested module roles

#### `src/lib.rs`

The library entry point.

Use it to expose the public modules and the main engine type.

#### `src/model/`

Contains the core domain types.

Suggested contents:

- `metadata.rs`: metadata value types and shared metadata map alias
- `record.rs`: stored document / item representation
- `record_key.rs`: composite record identity for namespace + collection + id
- `query.rs`: query inputs such as text, embedding, top-k, namespace, and collection
- `filter.rs`: filter expressions and operators
- `result.rs`: retrieval result types and scoring fields

#### `src/storage/`

Contains record persistence interfaces and the first concrete storage implementation.

Suggested contents:

- `store.rs`: insert, get, list, and update-style storage operations

#### `src/index/`

Contains indexing and search data structures.

Suggested contents:

- `lexical.rs`: token storage and lexical scoring helpers
- `vector.rs`: embedding storage and similarity helpers

#### `src/retrieval/`

Contains the retrieval pipeline itself.

Suggested contents:

- `coordinator.rs`: orchestrates lexical search, vector search, and filtering
- `merge.rs`: combines candidate sets from multiple retrieval paths
- `rank.rs`: final score calculation and top-k sorting

#### `src/api/`

Contains the user-facing engine interface.

Suggested contents:

- `engine.rs`: methods like index record, index batch, and search

#### `src/config.rs`

Holds shared configuration types such as embedding dimension, ranking weights, and default limits.

#### `src/error.rs`

Defines shared engine errors so all modules return a consistent error type.

---

## Recommended V1 Build Sequence

To keep the implementation manageable, build the modules in this order:

1. `model`
2. `error` and `config`
3. `storage`
4. `filter` logic inside `model` or `retrieval`
5. `index/lexical`
6. `index/vector`
7. `retrieval/coordinator`, `merge`, and `rank`
8. `api/engine`
9. tests and evaluation fixtures

This order works well because each phase gives the next one stable types and interfaces to build on.

---

## Namespaces

Records are isolated by namespace.

Example namespaces:

```text
workspace_123
workspace_123/user_8
workspace_123/thread_88
```

Namespaces allow multiple applications to share the same engine.

---

## Storage Model

Records are stored in collections.

Example collections:

```text
products
documents
insights
memories
```

Each collection has its own index.

---

## Future Extensions

Future versions may add:

- ANN indexing (HNSW)
- distributed indexing
- memory consolidation
- agent-aware retrieval
- time-decay scoring
