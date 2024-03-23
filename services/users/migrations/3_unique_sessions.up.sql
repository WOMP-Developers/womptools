
ALTER TABLE `sessions`
    ADD `ip` VARCHAR(45) NOT NULL,
    ADD `last_used_at` DATETIME DEFAULT CURRENT_TIMESTAMP,
    ADD UNIQUE `unique_session` (`user_id`, `ip`);
