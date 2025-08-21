-- create the course type enum
CREATE TYPE course_type_enum AS ENUM (
    'FREE',
    'PAID'
);

-- create the Tutor table
CREATE TABLE tutor (
    id UUID PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE,
    course_type course_type_enum NOT NULL,
    courses BIGINT DEFAULT 0,
    rating NUMERIC(0,0)  -- e.g., 4.50
);


--CREATE THE COURSE TABLE

CREATE TABLE course{
	id UUID PRIMARY KEY NOT NULL,
	name TEXT NOT NULL,
    enro
	
}