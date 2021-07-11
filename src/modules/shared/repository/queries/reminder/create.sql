INSERT INTO `reminder`
(
    `reminder`.`user_id`,
    `reminder`.`channel_id`,
    `reminder`.`message`,
    `reminder`.`due_date`
)
VALUES
(
    ?,
    ?,
    ?,
    ?
)