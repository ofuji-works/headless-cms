INSERT INTO content_model (
  name,
  api_identifier,
  description
)
VALUES(
  'sample1',
  'sample1',
  'sample1 content model mock data'
);

INSERT INTO contents (
  content_model_id,
  fields,
  status
)
VALUES(
  (SELECT content_model_id FROM content_model LIMIT 1),
  '[{"field_type": "ShortText", "value": "Blog Title"},{"field_type": "LongText","value": "Blog Content"}]',
  'Draft'
);
