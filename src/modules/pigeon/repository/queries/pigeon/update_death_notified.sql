UPDATE
`pigeon`
SET
`death_notified` = ?
WHERE `human_id` = ?
AND `condition` = 'dead'