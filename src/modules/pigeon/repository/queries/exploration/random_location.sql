SELECT
    `exploration_planet_location`.`id` as id,
    IFNULL(`exploration_planet_location`.`image_url`, `exploration_planet`.`image_url`) as image_url,
    30 as `travel_distance_in_minutes`
FROM
`exploration_planet_location`
INNER JOIN `exploration_planet` ON `exploration_planet`.`id` = `exploration_planet_location`.`planet_id`
WHERE `exploration_planet`.`id` != 1 AND
    (
        SELECT COUNT(*)
        FROM `exploration_action`
        WHERE ((`exploration_action`.`location_id` = `exploration_planet_location`.`id` OR `exploration_action`.`location_id` IS NULL)
            AND `exploration_action`.`planet_id` = `exploration_planet`.`id`)
    ) >= 2
ORDER BY RAND()
LIMIT 1