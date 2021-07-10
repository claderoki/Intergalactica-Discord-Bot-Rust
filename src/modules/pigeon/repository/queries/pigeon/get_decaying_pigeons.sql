SELECT
`pigeon`.`name`,
`pigeon`.`health`,
`pigeon`.`happiness`,
`pigeon`.`cleanliness`,
`pigeon`.`food`,
`pigeon`.`human_id`
FROM
`pigeon`
WHERE `pigeon`.`condition` = 'active'
AND `pigeon`.`status` = 'idle'