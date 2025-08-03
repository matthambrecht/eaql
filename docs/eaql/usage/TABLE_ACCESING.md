# Table Accessing Queries
Table accessing queries require 4 parts: target table, associated columns, filters (limiting queries based on table values), and post-processors (i.e. limiting number of results).

## Parts of Query
### Choosing a Table Name
- **Format**: [[Get Keyword](#get-keywords)] ([Column Selection](#choosing-target-columns)) [[From Keyword](#from-keywords)] {Table Name}
- **Example**: `get column_name from table_name`

### Choosing Target Columns
- **Format**: Column(s) can be listed in standard english listing format or as a comma-separated list.
- **Example**: `column_1, column_2 and column_3`

### Filters (Optional)
- **Format**: This is just listed as a mathematical boolean expression. Keep in mind, order of operations is determined by parentheses and ordring at this point in time, this *will* not be the end format and is a more simplified placeholder for furture versions. This means that `a and b or c and c` is treated as `(a and (b or (c and (c))))`. If you don't understand this concept please see this first: [logical expressions](https://runestone.academy/ns/books/published/thinkcspy/Selection/Logicaloperators.html).
- **Example**: `cost < 15 and (expiration_year > 2026 or best_by_date_exists = False)`

- [Logical Keywords](#logical-keywords)
- [Logical Operators](#logical-operators)

### Post-Processors (Optional)
- **Format**: To indicate that you will be using post-processors you will need to start with a [post-processor entrance keyword](#post-processor-entrances). Each post-processor will then have it's own format and if you wish to use multiple you just need to chain them together with "and".
- **Example**: `then limit it to 5 and sort in ascending order`

#### Limit Post-Processor
- **Format**: [[Limit Keyword](#limit-keywords)] {Amount}
- **Example:** `limit 5`

## Table Accessing Keyword Glossary
### Get Keywords
Indicates that the current query is for data retrieval from a table.

- Get
- Retrieve
- Find

### From Keywords
Indicates during retrieval which table we are getting data from.

- From
- In

### Logical Keywords
- And
- Or

### Logical Operators
- `> (Greater Than)`
- `>= (Greater Than or Equal to)`
- `< (Less Than)`
- `<= (Less Than or Eaql to)`
- `= (Equal to)`

### Post-Processor Entrances
Indicates that we would like to modify the results of our retrieval in a certain way.

- Then
- Afterwords
- After

### Limit Keywords
Indicates we would like to limit our results to a certain amount

- Limit
