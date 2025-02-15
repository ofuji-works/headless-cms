INSERT INTO category (
  name,
  api_identifier,
  description
)
VALUES(
  'sample1',
  'sample1',
  'sample1 category mock data'
);

INSERT INTO contents (
  category_id,
  fields,
  status
)
VALUES(
  (SELECT id FROM category LIMIT 1),
  '[{"field_type": "ShortText", "value": "Blog Title"},{"field_type": "LongText","value": "Blog Content"}]',
  'Draft'
);
