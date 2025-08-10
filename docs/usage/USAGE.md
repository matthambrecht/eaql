# Language Usage
The core aim of EAQL's syntax structure is to convert the core goals achieved by other database query languages like SQL into a more english form.

## Documentation Details
- Queries formats will be defined using this notation
    - `[]`: Required Keyword
    - `{}`: User-defined literal or identifier (i.e. table name or column value)
- All queries must end with an End-of-Query Token (`.`, `!`, or `;`)
- Chaining of queries is unsupported at this point in time but will be supported in the future.

## Query Formats
This is how specific queries are supposed to be formatted to be properly understood and parsed by the EAQL Parser. To get a better understanding of why they are this way and how queries are processed from tokens into a usable format see the [architecture](../architecture/ARCHITECTURE.md).

- [Table Access Queries](./TABLE_ACCESING.md)
- [Table Mutation Queries]
- [Database Queries](./DATABASE.md)