-- Create accounts table.
-- down: DROP TABLE accounts;
CREATE TABLE accounts (
    id integer PRIMARY KEY,
    uuid UUID NOT NULL,
    given_name varchar NOT NULL,
    email varchar NOT NULL UNIQUE,
    hash varchar,
    created_at timestamp DEFAULT now() NOT NULL,
    avatar_url varchar
);

-- Create IdentitySource enum type.
-- down: DROP TYPE IdentitySource;
CREATE TYPE IdentitySource AS ENUM ('password', 'google');

-- Create Identities table.
-- down: DROP TABLE identities;
CREATE TABLE identities (
    id serial NOT NULL,
    account_id integer REFERENCES accounts (id) NOT NULL,
    source IdentitySource NOT NULL,
    PRIMARY KEY(account_id, source)
);

-- Create refresh tokens table
-- down: DROP TABLE refresh_tokens;
CREATE TABLE refresh_tokens (
    id integer PRIMARY KEY,
    account_id integer REFERENCES accounts (id) NOT NULL,
    issued_at timestamp DEFAULT now() NOT NULL,
    expires timestamp NOT NULL,
    revoked boolean DEFAULT FALSE NOT NULL,
    revocation_time timestamp,
    token varchar NOT NULL
);
