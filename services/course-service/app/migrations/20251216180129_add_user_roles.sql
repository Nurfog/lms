-- Crear un tipo ENUM para los roles de usuario
CREATE TYPE user_role AS ENUM ('student', 'instructor', 'admin');

-- Añadir la columna 'role' a la tabla 'users'
-- Por defecto, todos los nuevos usuarios serán 'student'
ALTER TABLE users ADD COLUMN role user_role NOT NULL DEFAULT 'student';
