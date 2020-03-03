CREATE TABLE api_user (
    PRIMARY KEY (username),
    username VARCHAR(16)  NOT NULL,
    salt     VARCHAR(128) NOT NULL,
    hash     VARCHAR(128) NOT NULL
);

CREATE TABLE tweet (
    PRIMARY KEY (id),
    id           UUID      NOT NULL,
    markdown     TEXT      NOT NULL,
    created_time TIMESTAMP NOT NULL
);
CREATE INDEX tweet_created_time_idx ON tweet (created_time);

CREATE TABLE blog (
    PRIMARY KEY (id),
    id           UUID      NOT NULL,
    title        TEXT      NOT NULL,
    markdown     TEXT      NOT NULL,
    created_time TIMESTAMP NOT NULL
);
CREATE INDEX blog_created_time_idx ON blog (created_time);

CREATE TABLE page (
    PRIMARY KEY (pagename),
    pagename VARCHAR(16) NOT NULL,
    markdown TEXT        NOT NULL
);
