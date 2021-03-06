SELECT
`pigeon`.`name`,
`pigeon`.`health`,
`human`.`gold`,
`pigeon`.`happiness`,
`pigeon`.`cleanliness`,
`pigeon`.`experience`,
`pigeon`.`food`,
`pigeon`.`status`
FROM
`pigeon`
INNER JOIN `human` ON `human`.`id` = `pigeon`.`human_id`
WHERE `pigeon`.`human_id` = ? AND `pigeon`.`condition` = 'active'
LIMIT 1