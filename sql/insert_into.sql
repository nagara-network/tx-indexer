INSERT INTO `balance_transfers`(
    `blocknumber`,
    `sequence`,
    `hash`,
    `sender`,
    `receiver`,
    `amount`,
    `fee`,
    `unixtime`
)
VALUES (
     ?,
     ?,
     ?,
     ?,
     ?,
     ?,
     ?,
     ?
);
