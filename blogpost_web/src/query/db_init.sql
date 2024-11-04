CREATE TABLE IF NOT EXISTS posts
(
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    username VARCHAR(255) NOT NULL,
    creation_date VARCHAR(255) NOT NULL,
    main_text VARCHAR(255) NOT NULL,
    blog_img_src VARCHAR(255),
    avatar_img_src VARCHAR(255)
);