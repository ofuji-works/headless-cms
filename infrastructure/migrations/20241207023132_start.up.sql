-- Add up migration script here
CREATE OR REPLACE FUNCTION set_updated_at() RETURNS trigger AS '
  BEGIN
    NEW.updated_at := ''now'';
    RETURN NEW;
  END;
' LANGUAGE 'plpgsql';

CREATE TABLE IF NOT EXISTS category (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  name VARCHAR(50) NOT NULL,
  api_identifier VARCHAR(64) NOT NULL,
  description VARCHAR(500) NOT NULL,
  created_at TIMESTAMP(3) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP(3),
  updated_at TIMESTAMP(3) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP(3)
);
CREATE TRIGGER category_updated_at_trigger
  BEFORE UPDATE ON category FOR EACH ROW EXECUTE PROCEDURE set_updated_at();

CREATE TYPE content_status AS ENUM('Draft', 'Reserved', 'Published', 'Unpublished');
CREATE TABLE IF NOT EXISTS contents (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  category_id UUID NOT NULL,
  fields JSONB NOT NULL,
  status content_status NOT NULL, 
  published_at TIMESTAMP(3) WITH TIME ZONE DEFAULT NULL,
  created_at TIMESTAMP(3) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP(3),
  updated_at TIMESTAMP(3) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP(3),
  FOREIGN KEY (category_id) REFERENCES category(id)
    ON DELETE CASCADE
    ON UPDATE CASCADE
);
CREATE TRIGGER contents_updated_at_trigger
  BEFORE UPDATE ON contents FOR EACH ROW EXECUTE PROCEDURE set_updated_at();

