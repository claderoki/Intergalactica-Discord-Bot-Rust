SELECT
    `human`.`user_id` as `value`
FROM
`pigeon`
INNER JOIN `human` ON `human`.`id` = `pigeon`.`human_id`
INNER JOIN `earthling` ON `earthling`.`global_human_id` = `human`.`id` AND `earthling`.`guild_id` = ?
WHERE `pigeon`.`condition` = 'active'
AND `pigeon`.`status` = 'idle'
ORDER BY RAND()