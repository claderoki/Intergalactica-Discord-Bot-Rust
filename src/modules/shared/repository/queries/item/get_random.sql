SELECT results.* FROM (
    SELECT 
        `item`.`id`,
        `item`.`name`,
        `item`.`image_url`,
        @running_total AS previous_total,
        @running_total := @running_total + `item`.`chance` AS running_total,
        until.rand
FROM (
    SELECT round(rand() * init.max) AS rand FROM (
    SELECT sum(`item`.`chance`) - 1 AS max FROM `item` WHERE `item`.`category_id` IN (?)
    ) AS init
) AS until,
(SELECT * FROM `item` WHERE `item`.`category_id` IN (?)) AS `item`,
( SELECT @running_total := 0.00 ) AS vars
) AS results
WHERE results.rand >= results.previous_total AND results.rand < results.running_total;
