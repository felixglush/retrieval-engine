# Hybrid Retrieval Engine for AI Applications (Rust)

## Problem

AI systems increasingly depend on retrieval to ground model outputs in external data.
Common retrieval approaches rely on either:

- vector similarity search
- lexical / keyword search
- structured database queries

In practice, useful retrieval requires **combining all three**.

This introduces complexity, latency, and operational overhead.

This project aims to build a **lightweight hybrid retrieval engine in Rust** that supports:

- semantic similarity search
- keyword search
- metadata filtering
- hybrid ranking

The engine is designed as a retrieval substrate for AI systems such as:

- RAG pipelines
- analytics copilots
- agent memory systems

---

## Goals

The system should support:

1. **Vector search**

Similarity search over embedding vectors.

2. **Lexical search**

Keyword search over document text.

3. **Metadata filtering**

Structured filters on attributes.

Examples:

```
category = "dress"
price < 150
workspace_id = "abc"
```

4. **Hybrid ranking**

Combine signals from:

- semantic similarity
- lexical matching
- metadata
- recency
- importance

5. **Namespaces**

Allow isolation of records by workspace, user, or application.

---

## Non-Goals (v1)

Version 1 will **not** include:

- distributed indexing
- horizontal sharding
- full SQL query support
- transactional guarantees
- complex query planners

The focus is on **single-node hybrid retrieval**.

---

## Example Use Cases

### Product search

```
query: "boho summer dresses"

filters:
category = "dress"
price < 200
```

### Insight retrieval

```
query: "high performing linen products"
```

### RAG document retrieval

Retrieve relevant chunks for LLM context.

---

## Example Query

```
query_text: "black linen dresses selling well"

filters:
category = "dress"
```

---

## Success Criteria

Version 1 is successful if it can:

- index tens of thousands of records
- support hybrid retrieval queries
- combine semantic + lexical ranking
- return results within low latency
