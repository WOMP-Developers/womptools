-- 24/03/2024

DROP TABLE IF EXISTS `authorized_toons`;
RENAME TABLE `characters` TO `authorized_toons`;

ALTER TABLE `authorized_toons`
    ADD `access_token` TEXT NOT NULL,
    ADD `refresh_token` TEXT NOT NULL;
