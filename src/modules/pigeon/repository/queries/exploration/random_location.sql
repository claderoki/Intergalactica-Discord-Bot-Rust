SELECT
    `exploration_planet_location`.`id` as id,
    IFNULL(`exploration_planet_location`.`image_url`, `exploration_planet`.`image_url`) as image_url,
    90 as `travel_distance_in_minutes`
FROM
    `exploration_planet_location`
INNER JOIN `exploration_planet` ON `exploration_planet`.`id` = `exploration_planet_location`.`planet_id`
WHERE
    `exploration_planet`.`id` != 1
AND
    `exploration_planet_location`.`active` = 1
ORDER BY RAND()
LIMIT 1
