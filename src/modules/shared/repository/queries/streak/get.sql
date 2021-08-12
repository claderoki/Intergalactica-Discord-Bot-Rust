SELECT
    `streak`.`current`,
    DATEDIFF(UTC_DATE(), DATE(`streak`.`last_set`)) as `days_missed`
FROM `streak`
WHERE `streak`.`key` = ?
AND `streak`.`human_id` = ?
