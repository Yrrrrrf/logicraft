# Usefull SQL Commands (PostgreSQL)

**Note:** Adjust these commands based on the specific needs of your project. Replace `public` with your specific schema name if not using the default, and replace `your_project_db` with the actual name of your project database.

The difference between and schema and a database is that a database can contain multiple schemas, and a schema can contain multiple tables. A schema is a logical container for tables, views, and functions. A database is a logical container for schemas.


## Users
- Create a new user
```sql
CREATE USER username WITH PASSWORD 'password';
```
- Show the current user
```sql
SELECT current_user;
```

### Privileges

- Connect to the Database
```sql
GRANT CONNECT ON DATABASE your_project_db TO OutwardFire;
```

- CRUD Operations
```sql
GRANT SELECT, INSERT, UPDATE, DELETE ON ALL TABLES IN SCHEMA public TO OutwardFire;
```

- DDL Operations
```sql
GRANT CREATE, ALTER, DROP ON SCHEMA public TO OutwardFire;
```
- Manage Indexes and Constraints
```sql
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA public TO OutwardFire;
```
- Function Execution (to execute functions like `NOW()`)
```sql
GRANT EXECUTE ON ALL FUNCTIONS IN SCHEMA public TO OutwardFire;
```
- Additional Privileges
```sql
GRANT USAGE, SELECT, UPDATE ON ALL SEQUENCES IN SCHEMA public TO OutwardFire;
```
- If `OutwardFire` needs to create other roles or manage the database:
```sql
ALTER ROLE OutwardFire WITH CREATEROLE CREATEDB;
```
- Grant Usage on the Schema (if `OutwardFire` needs to create tables)
```sql
GRANT USAGE ON SCHEMA public TO OutwardFire;
```

----

## DB

- List all tables in the database
```sql
SELECT schemaname, tablename
FROM pg_tables
WHERE schemaname NOT IN ('pg_catalog', 'information_schema')
ORDER BY schemaname, tablename;
```

- List all tables in the database with their size
```sql
SELECT schemaname, tablename, pg_size_pretty(pg_total_relation_size(schemaname || '.' || tablename)) AS size
FROM pg_tables
WHERE schemaname NOT IN ('pg_catalog', 'information_schema')
ORDER BY schemaname, tablename;
```


** check if this is correct **
- List all tables in the database with their size and row count
```sql
SELECT schemaname, tablename, pg_size_pretty(pg_total_relation_size(schemaname || '.' || tablename)) AS size, pg_total_relation_size(schemaname || '.' || tablename) AS size_bytes, pg_total_relation_size(schemaname || '.' || tablename) / pg_relation_size(schemaname || '.' || tablename) AS row_count
FROM pg_tables
WHERE schemaname NOT IN ('pg_catalog', 'information_schema')
ORDER BY schemaname, tablename;
```
