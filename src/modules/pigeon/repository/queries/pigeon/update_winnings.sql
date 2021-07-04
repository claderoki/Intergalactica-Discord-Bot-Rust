UPDATE `human`
INNER JOIN `pigeon` ON `pigeon`.`human_id` = `human`.`id` AND `pigeon`.`condition` = 'active'
SET
    `pigeon`.`health`      = LEAST(`pigeon`.`health` + ?, 100),
    `pigeon`.`happiness`   = LEAST(`pigeon`.`happiness` + ?, 100),
    `pigeon`.`cleanliness` = LEAST(`pigeon`.`cleanliness` + ?, 100),
    `pigeon`.`experience`  = `pigeon`.`experience` + ?,
    `pigeon`.`food`        = LEAST(`pigeon`.`food` + ?, 100),
    `human`.`gold`         = `human`.`gold` + ?
WHERE `human`.`id` = ?