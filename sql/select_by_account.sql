SELECT * FROM `balance_transfers`
WHERE
    (`sender` = ? OR `receiver` = ?)
    AND `unixtime` >= ?
    AND `unixtime` <= ?
ORDER BY
    `blocknumber` DESC,
    `sequence` DESC
LIMIT ?;
