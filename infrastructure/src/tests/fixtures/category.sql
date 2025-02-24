INSERT INTO category (
  name,
  api_identifier,
  description,
  created_by,
  updated_by
)
VALUES(
  'sample1',
  'sample1',
  'sample1 category mock data',
  (SELECT id FROM users LIMIT 1),
  (SELECT id FROM users LIMIT 1)
);

INSERT INTO category (
  name,
  api_identifier,
  description,
  created_by,
  updated_by
)
VALUES(
  'sample2',
  'sample2',
  'sample2 category mock data',
  (SELECT id FROM users LIMIT 1),
  (SELECT id FROM users LIMIT 1)
);

