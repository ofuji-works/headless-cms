-- Add down migration script here
ALTER TABLE IF EXISTS contents DROP CONSTRAINT contents_category_id_fkey;  
DROP TRIGGER IF EXISTS contents_updated_trigger ON contents;
DROP TABLE IF EXISTS contents;
DROP TYPE content_status;

DROP TRIGGER IF EXISTS category_updated_trigger ON content_model;
DROP TABLE IF EXISTS category;

DROP FUNCTION set_updated_at();

