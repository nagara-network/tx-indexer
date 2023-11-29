CREATE TABLE IF NOT EXISTS `transaction_histories` (
  `id` integer NOT NULL PRIMARY KEY AUTOINCREMENT,
  `hash` text NOT NULL,
  `sender` text NOT NULL,
  `receiver` text NOT NULL,
  `amount` binary(16) NOT NULL,
  `fee` binary(16) NOT NULL,
  `at_block_time` integer NOT NULL,
  `at_unix_time` integer NOT NULL
);
CREATE INDEX IF NOT EXISTS `idx-hash` ON `transaction_histories` (`hash`);
CREATE INDEX IF NOT EXISTS `idx-sender` ON `transaction_histories` (`sender`);
CREATE INDEX IF NOT EXISTS `idx-receiver` ON `transaction_histories` (`receiver`);
CREATE INDEX IF NOT EXISTS `idx-atblocktime` ON `transaction_histories` (`at_block_time`);
CREATE INDEX IF NOT EXISTS `idx-atunixtime` ON `transaction_histories` (`at_unix_time`);
