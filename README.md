<div align="center">
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset="./docs/images/logos/eaqldb-logo-white.svg">
        <source media="(prefers-color-scheme: light)" srcset="./docs/images/logos/eaqldb-logo-black.svg">
        <img height="250px" src=""/>
    </picture>
</div>

<p align="center"><img href="https://github.com/matthambrecht/eaql-db/actions/workflows/tests.yaml" src="https://github.com/matthambrecht/eaql-db/actions/workflows/tests.yaml/badge.svg"/></p>

**EAQL (English Augmented Query Language)** is a simplified, English-like query language designed as a stepping stone to SQL. It's built for learners, educators, and simple projects where traditional SQL may feel intimidating or overly complex.

~~Bundled with **EAQL-DB**, a lightweight in-memory database, this project lowers the barrier to entry for understanding databases and query logic, making it perfect for prototyping, classrooms, or learning on your own.~~

---

## What’s Included
### EAQL (The Language)

EAQL makes data querying more human-readable. Inspired by SQL but based on natural English, it allows statements like:

```
Get everything from drinks wherever the price is 5 and the category is "coffee" then sort it in ascending order.
```

...which transpiles to:

```sql
SELECT * FROM drinks WHERE price = 5 and category = "coffee" ORDER BY ASC;
```

#### Features:

* Custom parser and compiler written in **Rust**
* Beginner-friendly error feedback
* Transpiler REPL for testing EAQL → SQL

#### Coming Soon:

* **Training REPL** – Interactive learning tool for beginners

<p align="center"><img src="docs/images/utils/under_construction.png"/></p>

---

### ~~EAQL-DB (The Database)~~

~~A lightweight, in-memory database that runs EAQL directly, no SQL backend required. Perfect for zero-setup experimentation and rapid prototyping.~~

<p align="center"><img src="docs/images/utils/under_construction.png"/></p>

---

## Why EAQL?

This project demonstrates:

* End-to-end system design
* Domain-specific language (DSL) creation
* Compiler and transpiler architecture
* Lightweight DB engine design

It’s both a **learning tool for others** and a **personal deep dive** into compiler theory, operating systems, and database internals. With that being said, a lot of this is "learn as we go" so not every initial design choice will likely be optimal or even correct. I hope to continue iterating on this to keep learning and hopefully help others interested in creating a computer language, or database as a lot of my thoughts and realizations will be transferrable (i.e. lessons learned from [conditional-parsing](docs/eaql/PARSER.md/#conditional-parsing) is transferrable to calculator design and operational heirarchies).

> ⚠️ *Note: EAQL and EAQL-DB are for educational purposes only. Not intended for production use.*