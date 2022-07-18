CREATE TABLE IF NOT EXISTS orders (
    id BIGSERIAL,
    userid BIGINT NOT NULL,
    product VARCHAR(100) NOT NULL,
    total REAL NOT NULL,
    PRIMARY KEY (id)
);

INSERT INTO orders (userid, product, total) VALUES (1, 'Laptop', 429.99);
INSERT INTO orders (userid, product, total) VALUES (1, 'Dryer', 680.00);
INSERT INTO orders (userid, product, total) VALUES (1, 'Camera', 180.00);
