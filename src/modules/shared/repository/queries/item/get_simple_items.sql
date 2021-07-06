SELECT
    `item`.`id`,
    `item`.`name`,
    `item`.`image_url`,
    CAST(1 as INT) as `amount`
FROM `item`
WHERE `item`.`id` IN (?)
GROUP BY `item`.`id`