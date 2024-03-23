
CREATE TABLE `credentials` (
    `character_id` BIGINT UNSIGNED NOT NULL UNIQUE,
    `access_token` TEXT NOT NULL,
    `updated_at` DATETIME DEFAULT CURRENT_TIMESTAMP,
    `expires_at` DATETIME NOT NULL,

    PRIMARY KEY (`character_id`),

    FOREIGN KEY (`character_id`) 
        REFERENCES characters(`id`)
        ON DELETE CASCADE
)