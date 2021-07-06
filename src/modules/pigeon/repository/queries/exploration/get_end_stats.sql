SELECT
    CAST(SUM(gold) AS SIGNED) as gold,
    CAST(SUM(health) AS SIGNED) as health,
    CAST(SUM(happiness) AS SIGNED) as happiness,
    CAST(SUM(cleanliness) AS SIGNED) as cleanliness,
    CAST(SUM(experience) AS SIGNED) as experience,
    CAST(SUM(food) AS SIGNED) as food,
    TIME_TO_SEC(TIMEDIFF(UTC_TIMESTAMP(), `exploration`.`start_date`)) as total_seconds,
    GROUP_CONCAT(exploration_winnings.item_id) as item_ids
FROM
exploration_winnings
INNER JOIN exploration ON exploration.id = exploration_winnings.exploration_id
WHERE exploration_id = ?
