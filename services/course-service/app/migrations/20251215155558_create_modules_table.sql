-- Add migration script here
-- Crear la tabla de módulos
CREATE TABLE modules (
    id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
    course_id UUID NOT NULL REFERENCES courses(id) ON DELETE CASCADE,
    module_name VARCHAR(255) NOT NULL,
    module_slug VARCHAR(255) NOT NULL,
    module_description TEXT,
    module_order INT NOT NULL,
    module_status VARCHAR(255) NOT NULL,
    module_visibility VARCHAR(255) NOT NULL,
    module_created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    module_updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
