INSERT INTO
  tags (name)
VALUES 
  ('tag1'),
  ('tag2'),
  ('tag3'),
  ('tag4');

INSERT INTO
  content_tags (content_id, tag_id)
SELECT 
    (SELECT id FROM contents LIMIT 1), id 
FROM
  tags 
WHERE
  name
IN
  ('tag1', 'tag2', 'tag3', 'tag4');
