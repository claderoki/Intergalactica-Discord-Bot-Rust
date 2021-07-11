SELECT
`pigeon`.`name` as `value`
FROM
`pigeon`
INNER JOIN `human` ON `human`.`id` = `pigeon`.`human_id`
WHERE `pigeon`.`human_id` = ? AND `pigeon`.`condition` = 'active'
LIMIT 1