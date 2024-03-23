
ALTER TABLE `credentials`
    ADD `refresh_token` TEXT NOT NULL DEFAULT "",
    ADD `created_at` DATETIME DEFAULT CURRENT_TIMESTAMP,

ALTER TABLE `characters`
    DROP COLUMN `requires_authorization`;