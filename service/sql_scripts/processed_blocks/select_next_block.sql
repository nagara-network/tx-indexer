SELECT
  `id` + 1 `next_block`
FROM
  `processed_blocks` `pb`
WHERE
  NOT EXISTS (
    SELECT
      *
    FROM
      `processed_blocks`
    WHERE
      `id` = `pb`.`id` + 1
  )
ORDER BY
  `id`
LIMIT
  1
