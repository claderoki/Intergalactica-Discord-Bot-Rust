UPDATE `human`
INNER JOIN `pigeon` ON `pigeon`.`human_id` = `human`.`id` AND `pigeon`.`condition` = 'active'
SET
    `pigeon`.`health`      = GREATEST(LEAST(`pigeon`.`health` + ?, 100), 0),
    `pigeon`.`condition`   = (CASE WHEN `pigeon`.`health` + ? <= 0 THEN 'dead' ELSE `pigeon`.`condition` END),
    `pigeon`.`happiness`   = GREATEST(LEAST(`pigeon`.`happiness` + ?, 100), 0),
    `pigeon`.`cleanliness` = GREATEST(LEAST(`pigeon`.`cleanliness` + ?, 100), 0),
    `pigeon`.`experience`  = `pigeon`.`experience` + ?,
    `pigeon`.`food`        = GREATEST(LEAST(`pigeon`.`food` + ?, 100), 0),
    `human`.`gold`         = GREATEST(`human`.`gold` + ?, 0)
WHERE `human`.`id` = ?