-- Create Login Schema
-- Type: PostgreSQL script
-- Description: This script creates a schema for login related tables.
-- Version: 1.0
-- Last modified date: Dec 19th, 2023

-- Create a table to store user login sessions
CREATE TABLE user_login_sessions (
    session_id SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    login_time TIMESTAMP WITHOUT TIME ZONE DEFAULT (NOW() AT TIME ZONE 'UTC'),
    logout_time TIMESTAMP WITHOUT TIME ZONE,
    ip_address TEXT,
    additional_info TEXT
);

-- Create a table to store user details
CREATE TABLE users (
    user_id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    email VARCHAR(100) UNIQUE NOT NULL,
    created_at TIMESTAMP WITHOUT TIME ZONE DEFAULT (NOW() AT TIME ZONE 'UTC'),
    last_login TIMESTAMP WITHOUT TIME ZONE,
    is_active BOOLEAN DEFAULT TRUE,
    additional_info JSONB
);

-- Add foreign key constraint to user_login_sessions table to reference users table
ALTER TABLE user_login_sessions
ADD CONSTRAINT fk_user
FOREIGN KEY (user_id)
REFERENCES users (user_id)
ON DELETE CASCADE
ON UPDATE CASCADE;

-- Optional: Create a table for additional user profile information
CREATE TABLE user_profiles (
    user_id INT PRIMARY KEY,
    date_of_birth DATE,
    address TEXT,
    phone_number VARCHAR(20),
    FOREIGN KEY (user_id) REFERENCES users (user_id) ON DELETE CASCADE
);

-- Create a new schema for login related tables
CREATE SCHEMA login_schema;

-- Move the user related tables to the new schema
ALTER TABLE public.users SET SCHEMA login_schema;
ALTER TABLE public.user_login_sessions SET SCHEMA login_schema;
ALTER TABLE public.user_profiles SET SCHEMA login_schema;
