SELECT
a.id as id,
pigeon.status as pigeon_status,
(a.arrival_date <= UTC_TIMESTAMP()) as arrived,
actions_remaining,
total_actions,
ABS(TIME_TO_SEC(TIMEDIFF(UTC_TIMESTAMP(), arrival_date))) AS remaining_seconds,
CAST(ABS(((TIME_TO_SEC(TIMEDIFF(UTC_TIMESTAMP(), arrival_date)) / TIME_TO_SEC(TIMEDIFF(start_date, arrival_date)) * 100)-100)) AS INT) as percentage,
planet_location_id as location_id
FROM
pigeon
INNER JOIN exploration a ON a.pigeon_id = pigeon.id AND a.finished = 0
WHERE pigeon.human_id = ?