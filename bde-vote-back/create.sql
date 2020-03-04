CREATE TABLE votes
(
    login              VARCHAR(8)  NOT NULL PRIMARY KEY,
    vote               VARCHAR(10) NOT NULL,
    confirmation_token VARCHAR(20) NOT NULL,
    created_at         TIMESTAMP DEFAULT current_timestamp()
);

DROP TABLE votes;