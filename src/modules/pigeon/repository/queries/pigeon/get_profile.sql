SELECT
name, health, happiness, cleanliness, experience, food, status
FROM
pigeon
WHERE `pigeon`.`human_id` = ? AND `pigeon`.`condition` = 'active'
LIMIT 1