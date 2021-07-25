UPDATE
    `pigeon`
SET
    `pigeon`.`pooped_on_count` = `pigeon`.`pooped_on_count` + 1
WHERE
    `pigeon`.`human_id` = ? AND `pigeon`.`condition` = 'active'