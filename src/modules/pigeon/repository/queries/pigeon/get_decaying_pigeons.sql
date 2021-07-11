SELECT
    `pigeon`.`name`,
    `pigeon`.`health`,
    `pigeon`.`happiness`,
    `pigeon`.`cleanliness`,
    `pigeon`.`food`,
    `pigeon`.`human_id`,
    `human`.`user_id`
FROM
`pigeon`
INNER JOIN `human` ON `human`.`id` = `pigeon`.`human_id`
WHERE `pigeon`.`condition` = 'active'
AND `pigeon`.`status` = 'idle'