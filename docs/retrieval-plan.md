# Retrieval Plan

## Purpose

This document covers how a query moves through the engine and becomes ranked results.

---

## V1 Retrieval Flow

The retrieval flow should follow this sequence:

1. validate the query
2. determine which retrieval paths should run
3. generate lexical candidates
4. generate vector candidates
5. apply namespace, collection, and metadata filters
6. merge candidate sets
7. compute final scores
8. return top-k results

V1 assumptions carried over from the model plan:

- queries target exactly one namespace
- queries target exactly one collection
- filters are a flat implicit-AND list
- final results expose component scores
- final results use a result-specific view rather than the full stored record

---

## Main Subsystems

### Retrieval Coordinator

Controls query execution and delegates work to the retrieval subsystems.

### Filter Engine

Applies namespace, collection, and metadata constraints.

In v1, filter evaluation should follow the metadata comparison contract from the model plan.
That means invalid type/operator combinations are validation errors, not runtime surprises.

### Candidate Merge

Combines results from lexical and vector retrieval into one candidate set.

### Rank Fusion

Produces final result scores from the available signals.

---

## V1 Ranking Signals

Version 1 should support combining:

- lexical score
- semantic score
- recency score
- importance score

The first scoring model should be simple and easy to inspect.

---

## Key Design Questions

- Which filters can be applied during candidate generation versus after candidate generation?
- How large should the lexical and vector candidate pools be before final ranking?
- How should missing scores be treated when a record appears in only one retrieval path?
- Should freshness and importance be normalized before final fusion?

---

## Recommended Implementation Order

1. filtering rules
2. lexical candidate generation
3. vector candidate generation
4. candidate merge
5. weighted rank fusion
6. end-to-end retrieval tests

---

## Rust Learning Notes

This layer will likely introduce:

- methods in `impl` blocks for coordinator logic
- iterators for transforming and filtering candidate lists
- sorting and comparison logic for ranking
- shared result types for passing data across modules

An `impl` block in Rust is where you define methods for a type.
It is how the retrieval engine will attach operations like `search` or `run_query` to a struct.
