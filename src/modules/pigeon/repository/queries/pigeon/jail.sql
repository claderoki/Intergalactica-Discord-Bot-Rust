UPDATE
    `pigeon`
SET
    `pigeon`.`jailed_until` = DATE_ADD(UTC_TIMESTAMP(), INTERVAL ? HOUR)
WHERE
    `pigeon`.`human_id` = ? AND `pigeon`.`condition` = 'active'