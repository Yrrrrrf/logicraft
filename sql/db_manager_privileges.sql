-- Type: PostgreSQL script
-- Description: This script grants privileges to manipulate an schema of a database to a new role.
-- Version: 1.0
-- Last modified date: Dec 19th, 2023


-- Instructions for Use:
-- 1. Open this script in a text editor.
-- 2. Replace:
    -- 'new_role' with the desired new role name.
    -- 'your_database_name' with the name of the database where you want to grant privileges.
    -- 'your_schema' with the name of the schema where you want to grant privileges.
-- 3. Optionally, replace 'secure_password' with a secure password for the new role.
-- 4. Save the changes to the script.
-- 5. Open your command line tool.
-- 6. Execute the script using psql or another PostgreSQL client. Replace 'username' with your PostgreSQL username. 
--    For example, execute:
--      psql -U username -d your_database_name -a -f path_to_grant_privileges.sql
--    Replace 'path_to_grant_privileges.sql' with the path to this script.
-- 7. Remember, this script should be used carefully, as it grants extensive permissions.


-- Create Role
CREATE ROLE new_role WITH LOGIN PASSWORD 'secure_password';

-- Grant Connect on Database
GRANT CONNECT ON DATABASE your_database_name TO new_role;

-- Change to the database
\c your_database_name;

-- Grant CRUD Operations
GRANT SELECT, INSERT, UPDATE, DELETE ON ALL TABLES IN SCHEMA your_schema TO new_role;

-- Grant DDL Operations
GRANT CREATE, ALTER, DROP ON SCHEMA your_schema TO new_role;

-- Grant All Privileges on Tables
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA your_schema TO new_role;

-- Grant Privileges on Sequences
GRANT USAGE, SELECT, UPDATE ON ALL SEQUENCES IN SCHEMA your_schema TO new_role;

-- Grant Execute on Functions
GRANT EXECUTE ON ALL FUNCTIONS IN SCHEMA your_schema TO new_role;

-- Grant Usage on Schema
GRANT USAGE ON SCHEMA your_schema TO new_role;

-- Optional: Grant Role and DB Management
-- ALTER ROLE new_role WITH CREATEROLE CREATEDB;

-- Add this new role to a user (Replace 'existing_user' with the actual user name)
ALTER USER existing_user WITH ROLE new_role;
