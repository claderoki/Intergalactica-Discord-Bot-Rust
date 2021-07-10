SELECT
    (CASE WHEN
        `exploration_action_scenario_winnings`.`gold` > 0
    THEN
        CAST((`exploration_action_scenario_winnings`.`gold` * ?) AS INT)
    ELSE `exploration_action_scenario_winnings`.`gold`
    END) as `gold`,
    `exploration_action_scenario_winnings`.`health`,
    `exploration_action_scenario_winnings`.`happiness`,
    `exploration_action_scenario_winnings`.`cleanliness`,
    `exploration_action_scenario_winnings`.`experience`,
    `exploration_action_scenario_winnings`.`food`,
    `exploration_action_scenario_winnings`.`item_id`,
    `exploration_action_scenario_winnings`.`item_category_id`
FROM
exploration_action_scenario_winnings
WHERE id = ?
LIMIT 1