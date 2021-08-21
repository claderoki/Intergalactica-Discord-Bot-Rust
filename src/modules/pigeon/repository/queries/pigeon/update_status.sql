UPDATE
    `pigeon`
SET
    `status` = ?
WHERE
    `human_id` = ?
AND
    `condition` = 'active'