SELECT
    CAST(SUM(`exploration_winnings`.`gold`) AS SIGNED) as gold,
    CAST(SUM(`exploration_winnings`.`health`) AS SIGNED) as health,
    CAST(SUM(`exploration_winnings`.`happiness`) AS SIGNED) as happiness,
    CAST(SUM(`exploration_winnings`.`cleanliness`) AS SIGNED) as cleanliness,
    CAST(SUM(`exploration_winnings`.`experience`) AS SIGNED) as experience,
    CAST(SUM(`exploration_winnings`.`food`) AS SIGNED) as food,
    TIME_TO_SEC(TIMEDIFF(UTC_TIMESTAMP(), `exploration`.`start_date`)) as total_seconds,
    GROUP_CONCAT(`exploration_winnings`.`item_id`) as item_ids
FROM
`exploration_winnings`
INNER JOIN `exploration` ON `exploration`.`id` = `exploration_winnings`.`exploration_id`
WHERE `exploration_winnings`.`exploration_id` = ?