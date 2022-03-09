CREATE TABLE confirmations(
    id UUID NOT NULL PRIMARY KEY,
    email VARCHAR(50) NOT NULL UNIQUE,
    expires_at TIMESTAMP NOT NULL
);