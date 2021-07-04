SELECT
gold, health, happiness, cleanliness, experience, food, item_id, item_category_id
FROM
exploration_action_scenario_winnings
WHERE id = ?
LIMIT 1