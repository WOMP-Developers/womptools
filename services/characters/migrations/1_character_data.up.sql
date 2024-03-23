
CREATE TABLE `character_data` (
    `character_id` BIGINT UNSIGNED NOT NULL UNIQUE,
    `alliance_id` INT DEFAULT NULL,
    `bloodline_id` INT NOT NULL,
    `corporation_id` INT NOT NULL,
    `description` TEXT DEFAULT NULL,
    `faction_id` INT DEFAULT NULL,
    `gender` TEXT NOT NULL,
    `name` TEXT NOT NULL,
    `race_id` INT NOT NULL,
    `security_status` FLOAT DEFAULT NULL,
    `title` TEXT DEFAULT NULL,
    `updated_at` DATETIME DEFAULT CURRENT_TIMESTAMP,

    PRIMARY KEY (`character_id`),

    FOREIGN KEY (`character_id`) 
        REFERENCES characters(`id`)
        ON DELETE CASCADE
);
