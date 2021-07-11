SELECT
    `reminder`.`id`,
    `reminder`.`message`,
    `reminder`.`user_id`,
    `reminder`.`channel_id`
FROM `reminder`
WHERE `reminder`.`due_date` <= UTC_TIMESTAMP()
AND `reminder`.`sent` = '0'