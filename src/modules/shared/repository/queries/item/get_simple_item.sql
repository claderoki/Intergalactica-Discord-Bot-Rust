SELECT
`item`.`id`,
`item`.`name`,
`item`.`image_url`
FROM item
WHERE `item`.`id` = ?