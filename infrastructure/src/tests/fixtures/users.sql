INSERT INTO role (
  name,
  description,
  is_super_administrator
)
VALUES(
  'admin',
  'description',
  'true'
);

INSERT INTO users (
  name,
  icon_url,
  role_id
)
VALUES(
  'user1',
  'https://example/image.jpg',
  (SELECT id FROM role LIMIT 1)
);

