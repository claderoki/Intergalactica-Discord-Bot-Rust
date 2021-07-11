UPDATE `reminder`
SET
    `reminder`.`sent` = '1'
WHERE `reminder`.`id` IN (?)