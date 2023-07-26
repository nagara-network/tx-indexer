INSERT INTO `transaction_histories` (
  `hash`, `sender`, `receiver`, `amount`,
  `fee`, `at_block_time`, `at_unix_time`
)
VALUES
  (?1, ?2, ?3, ?4, ?5, ?6, ?7) ON CONFLICT(`hash`) DO NOTHING;
