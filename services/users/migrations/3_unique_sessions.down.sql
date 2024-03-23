
ALTER TABLE `sessions`
   DROP COLUMN `ip`,
   DROP COLUMN `last_used_at`;

-- TODO: drop the unique index
