SELECT
    `streak`.`current`,
    (UTC_DATE() > DATE(`streak`.`last_set`)) as `is_available`
FROM `streak`
WHERE `streak`.`key` = ?
AND `streak`.`human_id` = ?
