# API Plan

## Purpose

This document defines the public interface clients will use to interact with the retrieval engine.

---

## V1 API Goals

The public API should make these actions simple:

- upsert a record
- upsert a batch of records
- run a retrieval query
- return ranked results

The API should hide internal index and ranking details.

---

## Public Surface

The engine should expose a small, clean entry point rather than many loosely related functions.

Likely capabilities:

- create engine
- upsert record
- upsert records
- search

---

## V1 Design Priorities

- simple to call from application code
- predictable result shapes
- explicit errors for invalid inputs
- minimal leakage of internal implementation details

The API should align with the model plan by:

- requiring namespace and collection on writes
- requiring namespace and collection on queries in v1
- returning a result-specific view instead of the full stored record
- exposing component scores for debugging and tuning

---

## Questions To Settle

- Should the API be library-only in v1, or should it also expose HTTP later?
- Should indexing be synchronous in v1?
- Which configuration should be passed in at engine creation time?
- Should the API expose an optional expansion mode later for returning more record fields?

---

## Recommended Implementation Order

1. define the engine entry point
2. define record insertion methods
3. define the query method
4. define result and error behavior
5. add ergonomic improvements after the basics work

---

## Rust Learning Notes

This layer will likely introduce:

- constructor-style methods
- public vs private visibility
- `Result<T, E>` return types for fallible operations

`Result<T, E>` is Rust's standard way to represent an operation that can either succeed with a value `T` or fail with an error `E`.
It is important here because indexing and search can fail for clear reasons, such as invalid input or missing data.
