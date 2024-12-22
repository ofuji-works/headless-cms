-- Add up migration script here
CREATE OR REPLACE FUNCTION set_updated_at() RETURNS trigger AS '
  BEGIN
    NEW.updated_at := ''now'';
    RETURN NEW;
  END;
' LANGUAGE 'plpgsql';

CREATE TABLE IF NOT EXISTS content_model (
  content_model_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  name VARCHAR(50) NOT NULL,
  api_identifier VARCHAR(64) NOT NULL,
  description VARCHAR(500) NOT NULL,
  fields JSONB NOT NULL,
  created_at TIMESTAMP(3) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP(3),
  updated_at TIMESTAMP(3) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP(3)
);
CREATE TRIGGER content_model_updated_at_trigger
  BEFORE UPDATE ON content_model FOR EACH ROW EXECUTE PROCEDURE set_updated_at();

CREATE TABLE IF NOT EXISTS contents (
  content_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  content_model_id UUID NOT NULL,
  field_values JSONB NOT NULL,
  is_draft BOOLEAN NOT NULL, 
  published_at TIMESTAMP(3) WITH TIME ZONE DEFAULT NULL,
  created_at TIMESTAMP(3) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP(3),
  updated_at TIMESTAMP(3) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP(3)
);
CREATE TRIGGER contents_updated_at_trigger
  BEFORE UPDATE ON contents FOR EACH ROW EXECUTE PROCEDURE set_updated_at();

