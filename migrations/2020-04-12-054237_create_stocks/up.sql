CREATE TABLE stocks (
       id SERIAL PRIMARY KEY,
       ticker VARCHAR(4) NOT NULL,
       name VARCHAR(255),
       description TEXT
)
