
DROP TABLE IF EXISTS `wallets`;

CREATE TABLE balance_history (
    `uid` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT UNIQUE,
    `date` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `character_id` BIGINT UNSIGNED NOT NULL,
    `balance` DECIMAL NOT NULL,

    UNIQUE (`date`, `character_id`),

    PRIMARY KEY(`uid`, `character_id`),

    FOREIGN KEY (`character_id`) 
        REFERENCES characters(`id`)
        ON DELETE CASCADE
);
