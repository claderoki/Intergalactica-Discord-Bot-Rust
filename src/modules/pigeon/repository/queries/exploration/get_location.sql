SELECT
`exploration_planet_location`.`id` as id,
`exploration_planet`.`name` as planet_name,
`exploration_planet_location`.`name` as location_name,
(IFNULL(`exploration_planet_location`.`image_url`, `exploration_planet`.`image_url`)) as image_url
FROM
`exploration_planet_location`
INNER JOIN `exploration_planet` ON `exploration_planet`.`id` = `exploration_planet_location`.`planet_id`
WHERE `exploration_planet_location`.`id` = ?
LIMIT 1