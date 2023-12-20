
# PostgreSQL Command Guide

This guide provides an overview of commonly used PostgreSQL commands, including those for database management, user and role management, and more.

## General Database Operations

### List All Databases
```sql
\l
```
This command lists all databases in the PostgreSQL server.

### Connect to a Specific Database
```sql
\c [database_name]
```
Replace `[database_name]` with the name of the database you want to connect to.

### List All Tables in the Current Database
```sql
\dt
```
Displays a list of all tables in the currently connected database.

### Describe a Table
```sql
\d [table_name]
```
Replace `[table_name]` with the name of the table to show its structure, including columns and data types.

### List All Schemas
```sql
\dn
```
Lists all schemas available in the current database.

### List All Users/Roles
```sql
\du
```
Shows all users and roles along with their privileges in the PostgreSQL server.

### Quit psql
```sql
\q
```
Exits the PostgreSQL command-line interface.

## Query Execution

### Execute a SQL Query
```sql
-- Example: SELECT * FROM your_table;
```
Executes a specified SQL query. Replace `your_table` with the actual table name.

### Execute SQL Script from a File
```sql
\i [file_path]
```
Runs an SQL script from a specified file. Replace `[file_path]` with the path to the SQL file.

## System Information

### Show Current PostgreSQL Version
```sql
SELECT version();
```
Displays the current version of the PostgreSQL server.

### Show Current User
```sql
SELECT current_user;
```
Displays the name of the user currently connected to the PostgreSQL server.

### Show Current Database
```sql
SELECT current_database();
```
Displays the name of the currently connected database.

## Database Maintenance

### Vacuum a Database
```sql
VACUUM [table_name];
```
Cleans and optimizes the specified table or the entire database if no table is specified. Replace `[table_name]` with the name of the table.

### Analyze a Table
```sql
ANALYZE [table_name];
```
Collects statistics about the specified table to help optimize query performance. Replace `[table_name]` with the name of the table.

## User and Role Management

### Create a New User
```sql
CREATE USER [user_name] WITH PASSWORD 'password';
```
Creates a new user with the specified username and password. Replace `[user_name]` and `'password'` accordingly.

### Change a User's Password
```sql
ALTER USER [user_name] WITH PASSWORD 'new_password';
```
Updates the password for the specified user. Replace `[user_name]` and `'new_password'` accordingly.

### Grant Privileges
```sql
GRANT [privileges] ON [table_name] TO [user_name];
```
Grants specified privileges on a table to a user. Replace `[privileges]`, `[table_name]`, and `[user_name]` accordingly.

### Revoke Privileges
```sql
REVOKE [privileges] ON [table_name] FROM [user_name];
```
Revokes specified privileges on a table from a user. Replace `[privileges]`, `[table_name]`, and `[user_name]` accordingly.
