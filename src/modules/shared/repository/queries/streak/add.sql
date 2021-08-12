INSERT INTO `streak` (`key`, `human_id`, `current`)
VALUES (?, ?, 1)
ON DUPLICATE KEY UPDATE
    `current`  = `current` + 1,
    `last_set` = UTC_TIMESTAMP(),
    `greatest` = GREATEST(`greatest`, `current` + 1)
