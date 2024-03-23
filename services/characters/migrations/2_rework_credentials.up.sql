
ALTER TABLE `credentials`
    DROP COLUMN `refresh_token`,
    DROP COLUMN `created_at`;

ALTER TABLE `characters`
    ADD `requires_authorization` BOOL NOT NULL DEFAULT FALSE
    AFTER `is_main`;
