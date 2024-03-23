
ALTER DATABASE `sso` DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

DROP TABLE IF EXISTS `credentials`;

CREATE TABLE `credentials` (
    `character_id` BIGINT UNSIGNED NOT NULL UNIQUE,
    `refresh_token` TEXT NOT NULL,
    `access_token` TEXT NOT NULL,
    `updated_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `expires_at` DATETIME NOT NULL,

    PRIMARY KEY (`character_id`)
);
