CREATE TABLE IF NOT EXISTS `balance_transfers` (
    `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
    `blocknumber` INT UNSIGNED NOT NULL,
    `sequence` INT UNSIGNED NOT NULL,
    `hash` BINARY(32) NOT NULL,
    `sender` BINARY(32) NOT NULL,
    `receiver` BINARY(32) NOT NULL,
    `amount` BINARY(16) NOT NULL,
    `fee` BINARY(16) NOT NULL,
    `unixtime` BIGINT NOT NULL,
    PRIMARY KEY (`id`),
    INDEX `SENDER` (`sender` ASC) INVISIBLE,
    INDEX `RECEIVER` (`receiver` ASC) VISIBLE,
    INDEX `BLOCKTIME` (`blocknumber` ASC) INVISIBLE,
    INDEX `UNIXTIME` (`unixtime` ASC) INVISIBLE,
    INDEX `BLOCKHASH` (`hash` ASC) INVISIBLE
);
