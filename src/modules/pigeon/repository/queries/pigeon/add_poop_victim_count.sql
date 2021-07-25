UPDATE
    `pigeon`
SET
    `pigeon`.`poop_victim_count` = `pigeon`.`poop_victim_count` + 1
WHERE
    `pigeon`.`human_id` = ? AND `pigeon`.`condition` = 'active'