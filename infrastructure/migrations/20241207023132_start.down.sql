-- Add down migration script here
ALTER TABLE IF EXISTS contents DROP CONSTRAINT contents_content_model_id_fkey;  
DROP TRIGGER IF EXISTS contents_updated_trigger ON contents;
DROP TABLE IF EXISTS contents;

DROP TRIGGER IF EXISTS content_model_updated_trigger ON content_model;
DROP TABLE IF EXISTS content_model;

DROP FUNCTION set_updated_at();

