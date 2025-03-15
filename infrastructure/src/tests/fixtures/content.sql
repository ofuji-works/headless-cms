INSERT INTO category (
  id,
  name,
  api_identifier,
  description
)
VALUES(
  gen_random_uuid(), 
  'sample1',
  'sample1',
  'sample1 category mock data'
);

INSERT INTO contents (
  id,
  title,
  category_id,
  fields,
  status,
  created_by,
  updated_by
)
VALUES(
  gen_random_uuid(), 
  'title',
  (SELECT id FROM category LIMIT 1),
  '[{"field_type": "ShortText", "value": "Blog Title"},{"field_type": "LongText","value": "Blog Content"}]',
  'Draft',
  (SELECT id FROM users LIMIT 1),
  (SELECT id FROM users LIMIT 1)
);
