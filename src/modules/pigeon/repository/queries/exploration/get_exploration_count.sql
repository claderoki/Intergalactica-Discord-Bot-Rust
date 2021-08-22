SELECT
    COUNT(*) as count
FROM
    `exploration`
WHERE `exploration`.`pigeon_id` = (SELECT `pigeon`.`id` FROM `pigeon` WHERE `pigeon`.`human_id` = ? AND `pigeon`.`condition` = 'active' LIMIT 1)
AND `exploration`.`finished` = 1