SELECT
`pigeon`.`name`,
`pigeon`.`health`,
`human`.`gold`,
`pigeon`.`happiness`,
`pigeon`.`cleanliness`,
`pigeon`.`experience`,
`pigeon`.`food`,
`pigeon`.`status`,
(CASE WHEN `pigeon`.`jailed_until` IS NULL THEN 0 ELSE ABS(TIME_TO_SEC(TIMEDIFF(UTC_TIMESTAMP(), `pigeon`.`jailed_until`))) END) AS `jail_time_left_in_seconds`
FROM
`pigeon`
INNER JOIN `human` ON `human`.`id` = `pigeon`.`human_id`
WHERE `pigeon`.`human_id` = ? AND `pigeon`.`condition` = 'active'
LIMIT 1