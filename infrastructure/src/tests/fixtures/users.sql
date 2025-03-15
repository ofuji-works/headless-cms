INSERT INTO role (
  id,
  name,
  description,
  is_super_administrator
)
VALUES( 
  gen_random_uuid(), 
  'admin',
  'description',
  'true'
);

INSERT INTO users (
  id,
  name,
  icon_url,
  role_id
)
VALUES(
  gen_random_uuid(),
  'user1',
  'https://example/image.jpg',
  (SELECT id FROM role LIMIT 1)
);

