UPDATE `streak`
    SET
        `streak`.`current` = 1,
        `streak`.`last_set` = UTC_TIMESTAMP()
WHERE
    `streak`.`key`      = ?
AND
    `streak`.`human_id` = ?
