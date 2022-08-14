-- Create table records if it doesn't exist yet
CREATE TABLE IF NOT EXISTS records (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    temperature REAL NOT NULL,
    pressure REAL NOT NULL,
    humidity REAL NOT NULL
);