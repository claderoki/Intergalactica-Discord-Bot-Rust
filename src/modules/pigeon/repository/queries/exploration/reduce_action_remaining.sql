UPDATE `exploration`
SET
    `exploration`.`actions_remaining` = `exploration`.`actions_remaining` - 1
WHERE `exploration`.`id` = ?