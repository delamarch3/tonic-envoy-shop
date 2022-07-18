CREATE TABLE IF NOT EXISTS users (
    id BIGSERIAL,
    firstname VARCHAR(100) NOT NULL,
    lastname VARCHAR(100) NOT NULL,
    credit_limit BIGINT NOT NULL,
    PRIMARY KEY (id)
);

INSERT INTO users (firstname, lastname, credit_limit) VALUES ('bob', 'smith', 2000);
