SELECT
  *
FROM
  `transaction_histories`
WHERE
  `hash` = ?1
LIMIT
  1;
