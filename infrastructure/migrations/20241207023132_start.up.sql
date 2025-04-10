-- Add up migration script here
CREATE OR REPLACE FUNCTION set_updated_at() RETURNS trigger AS $$
  BEGIN
    NEW.updated_at := NOW();
    RETURN NEW;
  END;
$$ LANGUAGE plpgsql;

-- users table
CREATE TYPE roles AS ENUM('Admin', 'Member');
CREATE TABLE IF NOT EXISTS users (
  id UUID PRIMARY KEY NOT NULL,
  name VARCHAR(50) NOT NULL,
  icon_url VARCHAR(500) NOT NULL,
  role_id UUID NOT NULL,
  created_at TIMESTAMP(3) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP(3),
  updated_at TIMESTAMP(3) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP(3),
  deleted_at TIMESTAMP(3) WITH TIME ZONE DEFAULT NULL,
  FOREIGN KEY (role_id) REFERENCES role(id)
    ON DELETE RESTRICT
    ON UPDATE CASCADE
);
CREATE TRIGGER users_updated_at_trigger
  BEFORE UPDATE ON users FOR EACH ROW EXECUTE FUNCTION set_updated_at();
CREATE INDEX IF NOT EXISTS idx_users_deleted_at ON users(deleted_at);

-- categories table
CREATE TABLE IF NOT EXISTS categories (
  id UUID PRIMARY KEY NOT NULL,
  name VARCHAR(50) NOT NULL,
  api_identifier VARCHAR(64) NOT NULL UNIQUE,
  description VARCHAR(500) NOT NULL,
  created_at TIMESTAMP(3) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP(3),
  updated_at TIMESTAMP(3) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP(3)
);
CREATE TRIGGER category_updated_at_trigger
  BEFORE UPDATE ON category FOR EACH ROW EXECUTE FUNCTION set_updated_at();

-- contents table
CREATE TYPE content_status AS ENUM('Draft', 'Reserved', 'Published', 'Unpublished');
CREATE TABLE IF NOT EXISTS contents (
  id UUID PRIMARY KEY NOT NULL,
  title VARCHAR(50) NOT NULL,
  category_id UUID NOT NULL,
  fields JSONB NOT NULL,
  status content_status NOT NULL, 
  published_at TIMESTAMP(3) WITH TIME ZONE DEFAULT NULL,
  created_at TIMESTAMP(3) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP(3),
  updated_at TIMESTAMP(3) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP(3),
  created_by UUID NOT NULL,
  updated_by UUID NOT NULL,
  FOREIGN KEY (category_id) REFERENCES category(id)
    ON DELETE RESTRICT
    ON UPDATE CASCADE,
  FOREIGN KEY (created_by) REFERENCES users(id)
    ON DELETE RESTRICT
    ON UPDATE CASCADE,
  FOREIGN KEY (updated_by) REFERENCES users(id)
    ON DELETE RESTRICT
    ON UPDATE CASCADE
);
CREATE TRIGGER contents_updated_at_trigger
  BEFORE UPDATE ON contents FOR EACH ROW EXECUTE FUNCTION set_updated_at();
CREATE INDEX IF NOT EXISTS idx_contents_status ON contents(status);

-- tags table
CREATE TABLE IF NOT EXISTS tags (
  id UUID PRIMARY KEY NOT NULL,
  name VARCHAR(50) NOT NULL,
  description VARCHAR(500) NOT NULL, 
  created_at TIMESTAMP(3) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP(3),
  updated_at TIMESTAMP(3) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP(3)
);
CREATE TRIGGER tags_updated_at_trigger
  BEFORE UPDATE ON tags FOR EACH ROW EXECUTE FUNCTION set_updated_at();

-- content_tags table
CREATE TABLE IF NOT EXISTS content_tags (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
  content_id UUID NOT NULL,
  tag_id UUID NOT NULL,
  FOREIGN KEY (content_id) REFERENCES contents(id)
    ON DELETE CASCADE
    ON UPDATE CASCADE,
  FOREIGN KEY (tag_id) REFERENCES tags(id)
    ON DELETE CASCADE
    ON UPDATE CASCADE,
  UNIQUE (content_id, tag_id)
);
CREATE INDEX IF NOT EXISTS idx_content_tags_tag_id ON content_tags(tag_id);

