-- 22/03/2024

ALTER DATABASE `users` DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

DROP TABLE IF EXISTS `users`;

CREATE TABLE `users` (
    `id` BIGINT UNSIGNED AUTO_INCREMENT UNIQUE,

    PRIMARY KEY (`id`)
);

DROP TABLE IF EXISTS `esi_tokens`;

CREATE TABLE `authorized_toons` (
    `user_id` BIGINT UNSIGNED NOT NULL,
    `character_id` BIGINT UNSIGNED NOT NULL UNIQUE,
    `refresh_token` TEXT NOT NULL,
    `access_token` TEXT NOT NULL,

    PRIMARY KEY (`character_id`),
    FOREIGN KEY (`user_id`) 
        REFERENCES users(`id`)
        ON DELETE CASCADE
);
