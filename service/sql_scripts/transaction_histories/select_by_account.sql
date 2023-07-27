SELECT
  *
FROM
  `transaction_histories`
WHERE
  (
    `sender` = ?1
    OR `receiver` = ?1
  )
  AND `at_unix_time` >= ?2
  AND `at_unix_time` <= ?3
ORDER BY
  `at_unix_time` DESC,
  `id` DESC
LIMIT
  ?4;
