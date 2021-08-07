SELECT
    `earthling`.`user_id` as `value`
FROM
`pigeon`
INNER JOIN `earthling` ON `earthling`.`global_human_id` = `pigeon`.`human_id` AND `earthling`.`guild_id` = ?
WHERE `pigeon`.`condition` = 'active'
AND `pigeon`.`status` = 'idle'
ORDER BY RAND()