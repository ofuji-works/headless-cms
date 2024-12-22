-- Add down migration script here
DROP TRIGGER IF EXISTS content_model_updated_trigger ON content_model;
DROP TABLE IF EXISTS content_model;
DROP TRIGGER IF EXISTS contents_updated_trigger ON contents;
DROP TABLE IF EXISTS contents;

DROP FUNCTION set_updated_at();

