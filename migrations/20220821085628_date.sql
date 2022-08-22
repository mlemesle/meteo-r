-- Add the column 'date' to records table, with default value of NOW
ALTER TABLE records ADD COLUMN date TIMESTAMPTZ NOT NULL DEFAULT now();