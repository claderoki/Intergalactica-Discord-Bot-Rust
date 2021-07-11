INSERT INTO exploration
(planet_location_id, start_date, arrival_date, finished, pigeon_id)
VALUES
(
    ?,
    UTC_TIMESTAMP(),
    ?,
    0,
    (SELECT id FROM pigeon WHERE human_id = ? AND `pigeon`.`condition` = 'active' LIMIT 1)
)