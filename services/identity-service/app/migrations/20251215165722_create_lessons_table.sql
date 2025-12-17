-- Add migration script here
-- Crear la tabla de lecciones
CREATE TABLE lessons (
    id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
    module_id UUID NOT NULL REFERENCES modules(id) ON DELETE CASCADE,
    lesson_name VARCHAR(255) NOT NULL,
    lesson_slug VARCHAR(255) NOT NULL,
    lesson_description TEXT,
    lesson_order INT NOT NULL,
    lesson_status VARCHAR(255) NOT NULL,
    lesson_visibility VARCHAR(255) NOT NULL,
    lesson_created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    lesson_updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
