-- 22/03/2024

DROP TABLE IF EXISTS `sessions`;

CREATE TABLE `sessions` (
    `session_id` VARCHAR(36) NOT NULL,
    `user_id` BIGINT UNSIGNED NOT NULL,
    `created_at` DATETIME DEFAULT CURRENT_TIMESTAMP,

    PRIMARY KEY (`session_id`),

    FOREIGN KEY (`user_id`) 
        REFERENCES users(`id`)
        ON DELETE CASCADE
);
