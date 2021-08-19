INSERT INTO `streak` (`key`, `human_id`, `current`, `greatest`)
VALUES (?, ?, 1, 1)
ON DUPLICATE KEY UPDATE
    `current`  = `current` + 1,
    `last_set` = UTC_TIMESTAMP(),
    `greatest` = GREATEST(`greatest`, `current`)
