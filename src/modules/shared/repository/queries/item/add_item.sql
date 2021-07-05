INSERT INTO human_item (item_id, human_id, amount)
VALUES (?, ?, ?)
ON DUPLICATE KEY UPDATE
    amount = amount + ?
