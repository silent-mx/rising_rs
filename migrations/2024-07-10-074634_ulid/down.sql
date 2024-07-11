-- This file should undo anything in `up.sql`
DROP FUNCTION if EXISTS generate_ulid ();

DROP EXTENSION if EXISTS pgcrypto;