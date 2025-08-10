# Database Queries
Majority of database queries require two parts, an action and a target (barring a few outliers).

## Parts of Query
### Table of Contents
1. [Show](#show-keywords)
2. [Create](#create-keywords)
3. [Use](#use-keywords)
4. [Destroy](#delete-keywords)

### Listing All Databases
- **Format**: [[Show Keywords](#show-keywords)] [[Database Keywords](#database-keywords)]
- **Example**: `show databases.`

### Creating a Database
- **Format**: [[Create Keywords](#create-keywords)] [[Database Keywords](#database-keywords)] {Database Name}
- **Example**: `create database db_name.`

### Using a Database
- **Format**: [[Use Keywords](#create-keywords)] [[Database Keywords](#database-keywords)] {Database Name}
- **Example**: `use database db_name.`

### Deleting a Database
- **Format**: [[Delete Keywords](#delete-keywords)] [[Database Keywords](#database-keywords)] {Database Name(s)}
    - *Note: Databases(s) can be listed in standard english listing format or as a comma-separated list.*
- **Example**: `delete database db_1, db_2 and db_3.`

## Database Keyword Glossary
### Show Keywords
Indicates that the query action is to list something

- Show
- List

### Create Keywords
Indicates that the query action is to create something

- Create
- Make
- Add

### Use Keywords
Indicates that the query action is to use something (i.e. use a specific database for following queries)

- Use
- Enter

### Delete Keywords
Indicates that the query action is to delete something

- Delete
- Destroy
- Remove

### Database Keywords
Indicates that we will be targetting databases with our query

- Databases
- Database

