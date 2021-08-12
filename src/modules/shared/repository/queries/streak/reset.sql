UPDATE `streak`
    SET `streak`.`current` = 1
WHERE
    `streak`.`key`      = ?
AND
    `streak`.`human_id` = ?
