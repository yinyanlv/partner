CREATE TABLE `user` (
    `id` int(20) PRIMARY KEY AUTO_INCREMENT NOT NULL,
    `username` varchar(20) NOT NULL,
    `nickname` varchar(20), 
    `email` varchar(40) NOT NULL,
    `phone` varchar(20),
    `role` tinyint(2) DEFAULT 0,
    `password` varchar(40) NOT NULL,
    `salt` varchar(20) NOT NULL,
    `create_time` datetime NOT NULL,
    `update_time` datetime NOT NULL,
    UNIQUE KEY `username` (`username`),
    UNIQUE KEY `email` (`email`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8;

CREATE TABLE `work_record` (
    `id` int(32) PRIMARY KEY AUTO_INCREMENT NOT NULL,
    `username` varchar(20) NOT NULL,
    `day` date NOT NULL,
    `overtime` float(4,2) DEFAULT 0.0,
    `create_time` datetime NOT NULL,
    `update_time` datetime NOT NULL,
    KEY `username` (`username`),
    CONSTRAINT `work_record_ibfk_1` FOREIGN KEY (`username`) REFERENCES `user` (`username`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8;

CREATE TABLE `work_event` (
    `id` int(32) PRIMARY KEY AUTO_INCREMENT NOT NULL,
    `record_id` int(32) NOT NULL,
    `start_time` datetime NOT NULL,
    `end_time` datetime NOT NULL,
    `create_time` datetime NOT NULL,
    `update_time` datetime NOT NULL,
    KEY `record_id` (`record_id`),
    CONSTRAINT `work_event_ibfk_2` FOREIGN KEY (`record_id`) REFERENCES `work_record` (`id`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8;