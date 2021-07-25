UPDATE
    `pigeon`
SET
    `pigeon`.`status` = (CASE WHEN `pigeon`.`status` = 'jailed' THEN 'idle' ELSE `pigeon`.`status` END),
    `pigeon`.`jailed_until` = NULL
WHERE
    `pigeon`.`condition` = 'active'
AND
    `pigeon`.`jailed_until` IS NOT NULL
AND
    `pigeon`.`jailed_until` <= UTC_TIMESTAMP()