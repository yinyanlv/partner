CREATE TABLE `user` (
    `id` INT(20) PRIMARY KEY AUTO_INCREMENT NOT NULL,
    `username` VARCHAR(20) NOT NULL,
    `nickname` VARCHAR(20) NOT NULL, 
    `email` VARCHAR(40) NOT NULL,
    `phone` VARCHAR(20),
    `role` TINYINT(2) UNSIGNED DEFAULT 0,
    `password` VARCHAR(40) NOT NULL,
    `salt` VARCHAR(20) NOT NULL,
    `create_time` DATETIME NOT NULL,
    `update_time` DATETIME NOT NULL,
    UNIQUE KEY `username` (`username`),
    UNIQUE KEY `email` (`email`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8;