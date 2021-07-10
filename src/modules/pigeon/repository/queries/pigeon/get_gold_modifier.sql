SELECT
`pigeon`.`gold_modifier` as `value`
FROM
pigeon
WHERE `pigeon`.`human_id` = ? AND `pigeon`.`condition` = 'active'
LIMIT 1