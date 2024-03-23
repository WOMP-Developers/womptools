
DROP TABLE IF EXISTS `balance_history`;

CREATE TABLE `wallets` (
    `character_id` BIGINT UNSIGNED UNIQUE,
    `balance` BIGINT NOT NULL DEFAULT 0,
    `updated_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,

    PRIMARY KEY(`character_id`),
    FOREIGN KEY (`character_id`) 
        REFERENCES characters(`id`)
        ON DELETE CASCADE
);
