CREATE TABLE `user` (
    `id` int(20) PRIMARY KEY AUTO_INCREMENT NOT NULL,
    `username` varchar(20) NOT NULL,
    `nickname` varchar(20) NOT NULL, 
    `email` varchar(40) NOT NULL,
    `phone` varchar(20),
    `role` tinyint(2) UNSIGNED DEFAULT 0,
    `password` varchar(40) NOT NULL,
    `salt` varchar(20) NOT NULL,
    `create_time` datetime NOT NULL,
    `update_time` datetime NOT NULL,
    UNIQUE KEY `username` (`username`),
    UNIQUE KEY `email` (`email`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8;