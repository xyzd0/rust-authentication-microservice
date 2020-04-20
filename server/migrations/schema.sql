-- Create accounts table.
-- down: DROP TABLE accounts;
CREATE TABLE accounts (
    id integer PRIMARY KEY,
    given_name varchar NOT NULL,
    family_name varchar NOT NULL,
    email varchar NOT NULL,
    avatar_url varchar
);

-- Create IdentityProvider enum type.
-- down: DROP TYPE IdentityProvider;
CREATE TYPE IdentityProvider AS ENUM ('Password', 'Google');

-- Create Identities table.
-- down: DROP TABLE identities;
CREATE TABLE identities (
    id integer PRIMARY KEY,
    account_id integer REFERENCES accounts (id) NOT NULL,
    provider IdentityProvider NOT NULL,
    token varchar NOT NULL
);

-- Create refresh tokens table
-- down: DROP TABLE refresh_tokens;
CREATE TABLE refresh_tokens (
    id integer PRIMARY KEY,
    account_id integer REFERENCES accounts (id) NOT NULL,
    issued_at timestamp DEFAULT now() NOT NULL,
    expiry timestamp NOT NULL,
    revoked boolean DEFAULT FALSE NOT NULL,
    token varchar NOT NULL
)
