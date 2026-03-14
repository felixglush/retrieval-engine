# Implementation Roadmap

## Purpose

This document turns the architecture into a practical build sequence for version 1.

---

## Milestone 1: Foundations

Focus:

- create the Rust project structure
- define domain types
- define shared error and configuration types
- lock down validation rules, metadata comparison rules, and embedding dimension rules
- add first-class support types such as `RecordKey` and `EngineConfig`

Primary outcome:

- the project compiles with stable core types in place

---

## Milestone 2: Storage And Basic Indexing

Focus:

- implement in-memory record storage
- add lexical indexing
- add exact vector storage and similarity support
- enforce composite record keys and upsert semantics in storage/index maintenance

Primary outcome:

- records can be inserted and prepared for retrieval

---

## Milestone 3: Retrieval Pipeline

Focus:

- implement filtering
- add retrieval coordination
- merge candidate sets
- add weighted rank fusion
- return result-specific search views with component scores

Primary outcome:

- end-to-end hybrid retrieval works for simple datasets

---

## Milestone 4: Public API And Usability

Focus:

- expose the engine through a clean library API
- improve errors
- add logging and basic metrics

Primary outcome:

- another application can use the engine without needing to know its internals

---

## Milestone 5: Evaluation And Refinement

Focus:

- add test datasets
- measure latency
- inspect retrieval quality
- refine scoring and candidate sizes

Primary outcome:

- the engine is stable enough to call a real v1

---

## Suggested Working Rhythm

For each milestone:

1. define the types and interfaces first
2. implement the smallest working version
3. add tests
4. only then optimize or generalize

---

## Documentation Strategy

As implementation begins:

- keep architecture decisions in `docs/architecture.md` under a design decisions section
- keep component-specific planning in the matching plan docs
- add design updates when decisions become real, not just possible

---

## Rust Learning Strategy

The project should teach Rust in layers:

1. types and modules
2. error handling
3. collections and indexing structures
4. methods and traits
5. testing and refactoring

This keeps the language learning aligned with the system being built.
