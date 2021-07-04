SELECT
id, text, scenario_winnings_id as winnings_id
FROM
exploration_action_scenario
WHERE action_id = ?
ORDER BY RAND()
LIMIT 1