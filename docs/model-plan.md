# Model Plan

## Purpose

The model layer defines the shared data types used across the retrieval engine.

This is the best place to start implementation because every other component depends on these shapes.

---

## V1 Goals

Version 1 should define clear, stable types for:

- records
- queries
- metadata filters
- search results
- scoring inputs

The goal is not to model every future feature up front.
The goal is to create a small set of types that make indexing and retrieval possible.

---

## Working V1 Decisions

For planning purposes, the model layer should assume:

- `namespace` is required on stored records
- `collection` is required on stored records
- a query may include text, an embedding, or both
- a query should target one namespace at a time in v1
- metadata values in v1 should stay simple: strings, numbers, booleans, and string lists
- filters are a flat implicit-AND list in v1
- search results should expose component scores for debugging and tuning
- search results should return a result-specific view, not the full stored record
- embeddings must use a consistent dimension within a searchable scope

These are working decisions, not permanent rules.
They are meant to keep v1 focused and easier to implement.

The current implementation also includes:

- a first-class `RecordKey` type for namespace + collection + id
- an `EngineConfig` type for shared defaults such as embedding dimension and default `top_k`

---

## Main Types

### Record

Represents a stored item in the engine.

Suggested v1 fields:

- id
- namespace
- collection
- content
- embedding
- metadata
- created_at
- importance

Why these fields matter:

- `id` gives every record a stable identity
- `namespace` isolates one workspace or tenant from another
- `collection` groups related records such as `products` or `documents`
- `content` is the text used by lexical search and often the source text for embeddings
- `embedding` supports semantic search
- `metadata` supports structured filtering
- `created_at` supports freshness-aware ranking
- `importance` supports manual or application-level weighting

Example:

```json
{
  "id": "prod_123",
  "namespace": "workspace_abc",
  "collection": "products",
  "content": "Black linen midi dress with a relaxed summer fit.",
  "embedding": [0.12, -0.44, 0.08, 0.91],
  "metadata": {
    "category": "dress",
    "price": 149.0,
    "color": "black",
    "in_stock": true,
    "tags": ["linen", "summer", "midi"]
  },
  "created_at": "2026-03-12T10:30:00Z",
  "importance": 0.8
}
```

Planning note:

In Rust, this will almost certainly be a `struct`.
A `struct` is a named type that groups related fields together, which makes it the natural way to represent a stored record.

### Query

Represents a retrieval request.

Suggested v1 fields:

- query text
- query embedding
- namespace
- collection
- filters
- top-k

Recommended additions:

- retrieval mode or flags
- candidate pool sizes for lexical and vector retrieval

Why these fields matter:

- `query_text` drives lexical retrieval
- `query_embedding` drives vector retrieval
- `namespace` limits the search scope
- `collection` narrows the search to one logical dataset
- `filters` adds structured constraints
- `top_k` controls how many results are returned

Planning note:

For v1, `collection` should stay required.
That keeps indexing, retrieval, and ranking simpler because we do not need to fan a single query out across multiple collections yet.

Example:

```json
{
  "query_text": "black linen dresses selling well",
  "query_embedding": [0.11, -0.39, 0.12, 0.88],
  "namespace": "workspace_abc",
  "collection": "products",
  "filters": [
    {
      "field": "category",
      "operator": "eq",
      "value": "dress"
    },
    {
      "field": "price",
      "operator": "lt",
      "value": 200
    }
  ],
  "top_k": 10
}
```

Planning note:

The model should probably require that a query has at least one of:

- `query_text`
- `query_embedding`

Otherwise there is nothing to retrieve against.

### Filter

Represents structured constraints applied to records.

Suggested v1 fields:

- field
- operator
- value

Likely operators:

- equals
- less than
- greater than
- in-list

Example:

```json
{
  "field": "price",
  "operator": "lt",
  "value": 200
}
```

Additional examples:

```json
{
  "field": "category",
  "operator": "eq",
  "value": "dress"
}
```

```json
{
  "field": "tags",
  "operator": "contains",
  "value": "linen"
}
```

V1 filter behavior should be deliberately narrow:

- filters are evaluated as an implicit AND
- `eq` works on strings, numbers, and booleans
- `lt` and `gt` work only on numbers
- `contains` uses a string filter value and checks membership against a string-list field on the record
- invalid type/operator combinations should return validation errors rather than silently failing

Planning note:

In Rust, the operator will likely be an `enum`.
An `enum` is a type whose value can be one of several named variants.
That is useful here because a filter operator should only be one of a fixed set of valid choices, such as `Eq`, `Lt`, `Gt`, or `Contains`.

### Search Result

Represents a returned item plus its scores.

Suggested v1 fields:

- record id
- final score
- lexical score
- semantic score
- matched metadata

Recommended additions:

- namespace
- collection
- content preview
- record metadata
- freshness score
- importance score

Why these fields matter:

- `record_id` identifies the matching record
- `final_score` shows the fused ranking score
- component scores help debug ranking behavior
- metadata and content preview make results easier to use without an extra lookup

The result should not automatically include:

- the full embedding vector
- the entire raw stored record
- internal indexing fields

That keeps the API cleaner and avoids coupling result shapes to storage internals.

Example:

```json
{
  "record_id": "prod_123",
  "namespace": "workspace_abc",
  "collection": "products",
  "content_preview": "Black linen midi dress with a relaxed summer fit.",
  "metadata": {
    "category": "dress",
    "price": 149.0,
    "color": "black"
  },
  "final_score": 0.87,
  "lexical_score": 0.74,
  "semantic_score": 0.91,
  "freshness_score": 0.62,
  "importance_score": 0.8
}
```

Planning note:

Some scores may be missing for some results.
For example, a record found only by lexical search may not have a semantic score.
That is a good use case for `Option<T>` in Rust.
`Option<T>` means a value may either exist as `Some(value)` or be absent as `None`.

This same idea applies to optional query fields like `query_text` and `query_embedding`.

---

## Relationships Between The Main Types

These types connect in a straightforward pipeline:

1. a `Record` is stored and indexed
2. a `Query` asks for matching records
3. each `Filter` narrows the valid candidates
4. a `SearchResult` returns the best matches with scores

That relationship is worth keeping simple because it becomes the backbone of the whole engine.

One supporting type now sits between `Record` and storage:

- `RecordKey`, which is the composite identity made from namespace, collection, and id

This is important because storage keys should not assume `id` is globally unique across namespaces or collections.

---

## Suggested Validation Rules

These rules are reasonable to adopt early:

- `Record.id` must not be empty
- `Record.namespace` must not be empty
- `Record.collection` must not be empty
- `Record.content` must not be empty in v1
- `Record.importance`, if present, should be constrained to `0.0..=1.0`
- `Record.embedding`, if present, must be non-empty
- `Query.top_k` must be greater than zero
- `Query` must include `query_text`, `query_embedding`, or both
- `Query.collection` must not be empty
- `Query.query_embedding`, if present, must match the configured embedding dimension
- `Filter.field` must not be empty
- each `Filter` operator must be valid for the provided value type

Validation matters because it keeps bad data from leaking into storage and indexing logic.

In the Rust implementation, these checks should live on the model types themselves:

```rust
record.validate(expected_embedding_dimension)
query.validate(expected_embedding_dimension)
filter.validate()
```

That keeps the rules close to the data they protect.

The configuration layer should also validate its own inputs, especially:

- `default_top_k` must be greater than zero
- `embedding_dimension`, if present, must be greater than zero

---

## Metadata Comparison Contract

To keep filtering predictable in v1, the model should define clear comparison rules.

V1 decision:

Internally, the engine should use a narrow custom metadata value model rather than raw JSON values directly.

That means the Rust model should define a metadata value type with only the supported variants for v1.

Supported internal metadata value kinds:

- string
- number
- boolean
- string list

At the system boundary, metadata may still arrive in a JSON-like form.
If that happens, the engine should convert it into the internal metadata value type during validation and reject unsupported shapes early.

Supported operator behavior:

- `eq`: valid for string, number, and boolean
- `lt`: valid for number only
- `gt`: valid for number only
- `contains`: valid when the filter value is a string and the record field being filtered is a string list

Invalid comparisons should be rejected during validation.

Examples of invalid comparisons:

- `price contains "cheap"`
- `tags < 3`
- `in_stock > true`

This is important because a flexible metadata shape is only safe if the comparison behavior is explicit.

---

## Proposed Rust-Facing Shapes For V1

The goal of this section is not to finalize syntax perfectly before coding.
The goal is to define shapes that are stable enough to guide implementation.

### `MetadataValue`

Recommended internal value type:

```rust
enum MetadataValue {
    String(String),
    Number(f64),
    Boolean(bool),
    StringList(Vec<String>),
}
```

Why this shape fits v1:

- it supports the metadata value kinds the engine actually needs
- it avoids the ambiguity of arbitrary JSON inside the core engine
- it matches the intended filter operators cleanly

How to think about it:

An `enum` in Rust lets one value be exactly one of several allowed variants.
Here, a metadata value can be a string, a number, a boolean, or a list of strings, but not an arbitrary nested object.

Example values:

```text
MetadataValue::String("dress")
MetadataValue::Number(149.0)
MetadataValue::Boolean(true)
MetadataValue::StringList(vec!["linen", "summer"])
```

Why `f64` for numbers:

- it is simple for v1
- it works well for filtering and scoring
- it avoids premature complexity around numeric subtypes

This means numeric metadata like price, rating, or quantity can all use the same internal representation.

### `FilterOperator`

Recommended operator type:

```rust
enum FilterOperator {
    Eq,
    Lt,
    Gt,
    Contains,
}
```

Why this shape fits v1:

- it keeps the operator set intentionally small
- it matches the comparison contract already defined above
- it prevents invalid free-form operator strings from leaking into the engine core

Example meaning:

- `Eq` means exact equality
- `Lt` means less than
- `Gt` means greater than
- `Contains` means membership inside a string list

### `Filter`

Recommended filter type:

```rust
struct Filter {
    field: String,
    operator: FilterOperator,
    value: MetadataValue,
}
```

Why this shape fits v1:

- it is easy to validate
- it is easy to serialize and deserialize
- it supports the flat implicit-AND filter model without introducing an expression tree

Example:

```text
Filter {
    field: "price".to_string(),
    operator: FilterOperator::Lt,
    value: MetadataValue::Number(200.0),
}
```

Validation examples:

- valid: `price < 200`
- valid: `category == "dress"`
- valid: `tags contains "linen"`
- invalid: `price contains "cheap"`
- invalid: `in_stock > true`

### `Record.metadata`

Recommended internal metadata field shape:

```rust
type MetadataMap = HashMap<String, MetadataValue>;
```

And inside the record:

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

Why this shape fits v1:

- metadata fields remain flexible by name
- metadata values stay constrained by type
- filtering can look up a field by name and then compare against a known set of value kinds

One Rust concept here is `HashMap<K, V>`.
`HashMap` is Rust's key-value map type.
Here, the key is the metadata field name like `"price"` or `"category"`, and the value is the corresponding `MetadataValue`.

Example internal metadata map:

```text
"category" -> MetadataValue::String("dress")
"price" -> MetadataValue::Number(149.0)
"in_stock" -> MetadataValue::Boolean(true)
"tags" -> MetadataValue::StringList(vec!["linen", "summer"])
```

### Boundary Conversion Shape

At the system boundary, a client may still submit metadata in a JSON-like format.
The engine should convert that input into `MetadataMap` during validation.

Boundary example:

```json
{
  "category": "dress",
  "price": 149.0,
  "in_stock": true,
  "tags": ["linen", "summer"]
}
```

Internal representation after validation:

```text
{
  "category": MetadataValue::String("dress"),
  "price": MetadataValue::Number(149.0),
  "in_stock": MetadataValue::Boolean(true),
  "tags": MetadataValue::StringList(vec!["linen", "summer"]),
}
```

Unsupported boundary values should be rejected.

Examples:

- nested objects
- mixed-type arrays
- null values
- arrays of numbers or booleans

### Validation And Error Shape

The v1 implementation should give the model layer a shared validation error type.

Examples of validation failures:

- empty required fields
- missing query input
- embedding dimension mismatches
- out-of-range importance values
- invalid filter/operator/value combinations

The validation API should stay simple and local to the model:

```rust
record.validate(expected_embedding_dimension)
query.validate(expected_embedding_dimension)
filter.validate()
```

### `RecordKey`

The storage plan assumes records are identified by a composite key rather than `id` alone.

Recommended shape:

```rust
struct RecordKey {
    namespace: String,
    collection: String,
    id: String,
}
```

And the record should be able to produce its own key:

```rust
record.key()
```

Why this shape fits v1:

- it makes storage semantics explicit
- it avoids accidental key collisions across namespaces
- it gives the storage layer a stable identity type to use in maps and indexes

### `EngineConfig`

The model and storage layers need a shared place for global defaults and validation settings.

Recommended v1 shape:

```rust
struct EngineConfig {
    embedding_dimension: Option<usize>,
    default_top_k: usize,
}
```

Why this shape fits v1:

- it gives query and record validation a consistent source for embedding dimension
- it avoids passing loose configuration values throughout the codebase
- it gives the engine a natural place to grow later as ranking and indexing settings are added

### Why This Is Better Than Storing Raw JSON Internally

If the engine stored raw JSON values everywhere, every filter comparison would need to re-check the value shape at runtime.
By converting once at the boundary, the internal engine code becomes simpler and safer.

This is a common systems design pattern:

- flexible at the boundary
- strict in the core

That pattern is especially useful in Rust because Rust's type system rewards making supported states explicit.

---

## Metadata Representation Recommendation

The recommended v1 approach is:

- accept JSON-like metadata at the API boundary if needed
- convert it into a narrower internal metadata value type
- run filtering only against the internal metadata value type

Why this is the preferred design:

- it keeps the engine core predictable
- it makes filter logic easier to implement and test
- it prevents unsupported shapes from leaking into storage and retrieval
- it uses Rust's type system to make invalid states harder to represent

This is a deliberate tradeoff.
The boundary stays flexible, but the engine internals stay strict.

---

## `serde_json::Value` Versus Custom Metadata Type

Two realistic implementation approaches exist for metadata.

### Option 1: `serde_json::Value`

`serde_json::Value` is a general JSON value type from Rust's `serde_json` crate.
It can represent strings, numbers, booleans, arrays, objects, and null.

Benefits:

- very flexible
- easy to parse from incoming JSON
- fast to adopt at the boundary

Risks:

- allows more shapes than v1 intends to support
- requires repeated runtime checks during filtering
- makes comparison semantics less obvious
- increases the chance of subtle mismatches such as numbers stored as strings

### Option 2: Custom internal metadata value type

This means defining a Rust type that only allows the supported metadata variants for v1.

Benefits:

- explicit supported value kinds
- simpler and safer filter behavior
- fewer edge cases at runtime
- better fit for a focused v1 implementation

Tradeoff:

- requires a conversion step from incoming JSON-like data
- slightly more design work up front

### Recommendation

Use a custom internal metadata value type for the core engine.
If external inputs are JSON-like, convert them at the boundary and reject unsupported shapes during validation.

This gives the project flexibility where it interacts with the outside world and safety where the retrieval logic actually runs.

---

## Embedding Rules

The model should assume:

- embeddings are optional on records in v1
- when present, embeddings must have a consistent dimension within the same searchable scope
- queries with mismatched embedding dimensions should fail validation before retrieval starts

This prevents hard-to-debug failures during similarity scoring.

---

## Record Lifecycle Assumptions

The plan should assume one clear write behavior in v1:

- records are upserted by `id` within a namespace and collection

That means:

- inserting a new `id` creates a new record
- writing an existing `id` replaces the stored record and requires index updates

This matters because storage and indexing need consistent update semantics from day one.

---

## Design Questions

These questions should be answered before coding too far:

- Should `Query.collection` stay required in v1 and only become optional when cross-collection retrieval is explicitly designed?
- Which fields belong in the result view by default versus behind an optional expansion mechanism?
- Should `importance` be normalized to a strict range such as `0.0..=1.0`?
- Should `created_at` always be required, or can it be optional with freshness scoring disabled when missing?
- How much score detail should be public by default? All scores should be public.

---

## Recommended Implementation Order

1. Define `Record`
2. Define `Query`
3. Define `Filter`
4. Define `SearchResult`
5. Add validation rules

---

## Rust Learning Notes

This layer will likely introduce these Rust concepts early:

- `struct`: groups related fields into one named type
- `enum`: models a value that can be one of several variants, useful for filter operators
- `Option<T>`: represents a value that may or may not exist, useful for embeddings or optional scores
- `Vec<T>`: a growable list, useful for embeddings and filters

These are worth learning first because they match the core shapes of the retrieval engine.

One more concept will likely appear soon:

- `HashMap<K, V>`: a key-value map, useful if metadata is represented as dynamic fields rather than fixed struct fields

If we use dynamic metadata, a `HashMap` would let us store entries like `"category" -> "dress"` and `"price" -> 149.0`.
