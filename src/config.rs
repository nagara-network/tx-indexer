fn get_from_env<R: core::str::FromStr>(key: &str) -> R
where
    <R as core::str::FromStr>::Err: std::fmt::Debug, {
    let error_message = format!("Cannot find environment variable {key:?}");
    let string_value = std::env::var(key).expect(&error_message);

    R::from_str(&string_value).unwrap()
}

#[derive(Clone, Debug)]
pub(crate) struct RedisConfig {
    pub(crate) connection_url: url::Url,
}

impl RedisConfig {
    const ENV_CONNECTION_URL: &'static str = "REDIS_CONNECTION_URL";

    pub(crate) async fn into_connection(self) -> anyhow::Result<crate::wrapper::RedisClient> {
        let client = redis::Client::open(self.connection_url)?;
        let connection_manager = client.get_connection_manager().await?;
        let redis_client = crate::wrapper::RedisClient::new(connection_manager).await?;

        Ok(redis_client)
    }
}

impl Default for RedisConfig {
    fn default() -> Self {
        Self {
            connection_url: get_from_env(Self::ENV_CONNECTION_URL),
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct MySqlConfig {
    pub(crate) connection_url: url::Url,
}

impl MySqlConfig {
    const ENV_CONNECTION_URL: &'static str = "MYSQL_CONNECTION_URL";

    pub(crate) async fn into_connection(self) -> anyhow::Result<crate::wrapper::MysqlClient> {
        let pool = sqlx::MySqlPool::connect(self.connection_url.as_str()).await?;
        let mysql_client = crate::wrapper::MysqlClient::new(pool).await?;

        Ok(mysql_client)
    }
}

impl Default for MySqlConfig {
    fn default() -> Self {
        Self {
            connection_url: get_from_env(Self::ENV_CONNECTION_URL),
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct ChainConfig {
    pub(crate) connection_url: url::Url,
}

impl ChainConfig {
    const ENV_CONNECTION_URL: &'static str = "CHAIN_URL";

    pub(crate) async fn into_connection(self) -> anyhow::Result<crate::wrapper::ChainClient> {
        let use_insecure = match self.connection_url.scheme() {
            | "wss" | "https" => false,
            | "ws" | "http" => true,
            | bad_scheme => panic!("Unsupported scheme {bad_scheme:?}"),
        };
        let rpc_client = if use_insecure {
            subxt::backend::rpc::RpcClient::from_insecure_url(self.connection_url).await?
        } else {
            subxt::backend::rpc::RpcClient::from_url(self.connection_url).await?
        };
        let extrinsic_client = subxt::OnlineClient::from_rpc_client(rpc_client.clone()).await?;
        let legacy_rpc_client = subxt::backend::legacy::LegacyRpcMethods::new(rpc_client);
        let chain_client = crate::wrapper::ChainClient::new(extrinsic_client, legacy_rpc_client);

        Ok(chain_client)
    }
}

impl Default for ChainConfig {
    fn default() -> Self {
        Self {
            connection_url: get_from_env(Self::ENV_CONNECTION_URL),
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Config {
    worker_count: usize,
    redis: RedisConfig,
    mysql: MySqlConfig,
    chain: ChainConfig,
}

impl Config {
    const ENV_WORKER_COUNT: &'static str = "WORKER_COUNT";

    pub(crate) fn worker_count(&self) -> usize {
        self.worker_count
    }

    pub(crate) fn redis(&self) -> RedisConfig {
        self.redis.clone()
    }

    pub(crate) fn mysql(&self) -> MySqlConfig {
        self.mysql.clone()
    }

    pub(crate) fn chain(&self) -> ChainConfig {
        self.chain.clone()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            worker_count: get_from_env(Self::ENV_WORKER_COUNT),
            redis: Default::default(),
            mysql: Default::default(),
            chain: Default::default(),
        }
    }
}
