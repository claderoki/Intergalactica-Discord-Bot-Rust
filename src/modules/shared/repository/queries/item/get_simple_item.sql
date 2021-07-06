SELECT
    `item`.`id`,
    `item`.`name`,
    `item`.`image_url`,
    CAST(1 AS INT) as `amount`
FROM item
WHERE `item`.`id` = ?