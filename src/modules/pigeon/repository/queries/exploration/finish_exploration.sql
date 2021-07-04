UPDATE `exploration`
SET
    `exploration`.`end_date` = UTC_TIMESTAMP(),
    `exploration`.`finished` = 1
WHERE `exploration`.`id` = ?