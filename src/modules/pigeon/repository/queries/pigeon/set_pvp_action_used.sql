UPDATE
    `pigeon`
SET
    `pigeon`.`last_used_pvp` = UTC_TIMESTAMP()
WHERE
    `pigeon`.`human_id` = ? AND `pigeon`.`condition` = 'active'