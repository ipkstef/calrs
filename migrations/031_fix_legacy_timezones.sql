-- Fix legacy bare timezone names to full IANA identifiers
UPDATE users SET timezone = 'Europe/Paris' WHERE timezone = 'Paris';
UPDATE accounts SET timezone = 'Europe/Paris' WHERE timezone = 'Paris';
