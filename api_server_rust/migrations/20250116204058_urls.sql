CREATE TABLE urls (
    hash VARCHAR(50) PRIMARY KEY NOT NULL,
    original_url TEXT NOT NULL,
	creator_id INT NOT NULL references users(id)
);
