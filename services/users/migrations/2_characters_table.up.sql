-- 24/03/2024

DROP TABLE IF EXISTS `characters`;
RENAME TABLE `authorized_toons` TO `characters`;

ALTER TABLE `characters`
    DROP COLUMN `access_token`,
    DROP COLUMN `refresh_token`;
