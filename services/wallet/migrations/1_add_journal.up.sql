
CREATE TABLE `journal` (
    `id` BIGINT NOT NULL,
    `character_id` BIGINT UNSIGNED,
    `date` DATETIME NOT NULL,
    `description` TEXT NOT NULL,
    `ref_type` VARCHAR(40) NOT NULL,
    `reason` VARCHAR(255),
    `amount` DECIMAL(16, 2),
    `balance` DECIMAL(16, 2),
    `context_id` BIGINT,
    `context_id_type` VARCHAR(24),
    `first_party_id` INT,
    `second_party_id` INT,
    `tax` DECIMAL(16, 2),
    `tax_receiver_id` INT,

    UNIQUE (`id`, `character_id`),

    PRIMARY KEY(`id`, `character_id`),
    FOREIGN KEY (`character_id`) 
        REFERENCES characters(`id`)
        ON DELETE CASCADE
);