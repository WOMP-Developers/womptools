
ALTER DATABASE `wallet` DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE `characters` (
    `id` BIGINT UNSIGNED NOT NULL UNIQUE,
    `user_id` BIGINT UNSIGNED NOT NULL,
    `access_token` TEXT NOT NULL,
    `access_token_expire_at` DATETIME NOT NULL,

    PRIMARY KEY (`id`, `user_id`)
);

CREATE TABLE `wallets` (
    `character_id` BIGINT UNSIGNED UNIQUE,
    `balance` BIGINT NOT NULL DEFAULT 0,
    `updated_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,

    PRIMARY KEY(`character_id`),
    FOREIGN KEY (`character_id`) 
        REFERENCES characters(`id`)
        ON DELETE CASCADE
);
