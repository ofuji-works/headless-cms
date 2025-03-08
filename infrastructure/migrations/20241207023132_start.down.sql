-- content_tags table
ALTER TABLE IF EXISTS content_tags DROP CONSTRAINT content_tags_content_id_fkey;
ALTER TABLE IF EXISTS content_tags DROP CONSTRAINT content_tags_tag_id_fkey;
DROP INDEX IF EXISTS idx_content_tags_tag_id;
DROP TABLE IF EXISTS content_tags;

-- tag table
DROP TRIGGER IF EXISTS tags ON tags_updated_at_trigger;
DROP TABLE IF EXISTS tags;

-- contents table
ALTER TABLE IF EXISTS contents DROP CONSTRAINT contents_category_id_fkey;  
ALTER TABLE IF EXISTS contents DROP CONSTRAINT contents_created_by_fkey;
ALTER TABLE IF EXISTS contents DROP CONSTRAINT contents_updated_by_fkey;
DROP TRIGGER IF EXISTS contents_updated_trigger ON contents;
DROP TABLE IF EXISTS contents;
DROP TYPE content_status;

-- category table
ALTER TABLE IF EXISTS category DROP CONSTRAINT category_created_by_fkey;
ALTER TABLE IF EXISTS category DROP CONSTRAINT category_updated_by_fkey;
DROP TRIGGER IF EXISTS category_updated_trigger ON content_model;
DROP TABLE IF EXISTS category;

-- role_authorities table
ALTER TABLE IF EXISTS role_authorities DROP CONSTRAINT role_authorities_role_id_fkey;
ALTER TABLE IF EXISTS role_authorities DROP CONSTRAINT role_authorities_authority_id_fkey;
DROP TABLE IF EXISTS role_authorities;

-- users table
ALTER TABLE IF EXISTS users DROP CONSTRAINT users_role_id_fkey;
DROP INDEX IF EXISTS idx_users_deleted_at;
DROP TABLE IF EXISTS users;

-- role table
DROP TRIGGER IF EXISTS role ON role_updated_at_trigger;
DROP TABLE IF EXISTS role;

-- authority table
DROP TRIGGER IF EXISTS authority ON authority_updated_at_trigger;
DROP TABLE IF EXISTS authority;

DROP FUNCTION set_updated_at();

