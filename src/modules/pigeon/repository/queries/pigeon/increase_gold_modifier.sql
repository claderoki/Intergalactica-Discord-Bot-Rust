UPDATE
`pigeon`
SET
`gold_modifier` = `gold_modifier` + ?
WHERE `human_id` = ?
AND `condition` = 'active'