-- Add migration script here
-- Crear la tabla de cursos
CREATE TABLE courses (
    id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
    instructor_id UUID NOT NULL,
    course_name VARCHAR(255) NOT NULL,
    course_slug VARCHAR(255) NOT NULL,
    course_description TEXT,
    course_order INT NOT NULL,
    course_status VARCHAR(255) NOT NULL,
    course_visibility VARCHAR(255) NOT NULL,
    course_created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    course_updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
