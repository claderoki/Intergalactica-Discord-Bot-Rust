SELECT
    `item`.`id`,
    `item`.`name`,
    `item`.`image_url`,
    COUNT(*) as `amount`
FROM
`exploration_winnings`
INNER JOIN `item` ON `exploration_winnings`.`item_id` = `item`.`id`
WHERE `exploration_winnings`.`exploration_id` = ? AND `exploration_winnings`.`item_id` IS NOT NULL
GROUP BY `item`.`id`
