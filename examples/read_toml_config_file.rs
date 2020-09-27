/*!
由于dotenv库只是将配置文件以key-value的格式加载到环境变量里，用完之后又不能自动调用env::remove_var去清空环境变量来回收资源
而且toml格式支持枚举、DateTime、数组、配置项嵌套、Option等方便的数据格式(是否支持Option存疑)，所以用toml的可读性更高，
而且能直接反序列化为Rust结构体
但是DateTime类型这个反序列化暂时只能用toml::value::DateTime而非chrono
虽然toml配置文件里面有数据库链接，但是还是需要额外的.env文件好让sqlx编译时检查SQL语句
*/
use serde::Deserialize;
use std::io::Read;

const CONFIG_FILE: &str = "examples/read_toml_config_file.toml";

enum AppEnv {
    #[allow(dead_code)]
    Test,
    Production,
}

#[derive(Deserialize)]
struct Config {
    test: ServerConfig,
    production: ServerConfig,
}

impl Config {
    fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let mut config_file = std::fs::File::open(CONFIG_FILE)?;
        let mut buf = String::new();
        config_file.read_to_string(&mut buf)?;

        let config: Config = toml::de::from_str(&buf)?;
        Ok(config)
    }

    fn database_url(&self, app_env: AppEnv) -> String {
        let mysql_config = match app_env {
            AppEnv::Test => &self.test.mysql,
            AppEnv::Production => &self.production.mysql,
        };
        return format!(
            "mysql://{username}:{password}@{host}:{port}/{dbname}",
            username = mysql_config.username,
            password = mysql_config.password,
            host = mysql_config.host,
            port = mysql_config.port,
            dbname = mysql_config.dbname,
        );
    }
}

#[derive(Deserialize)]
struct ServerConfig {
    mysql: MySQLConfig,
    #[serde(rename = "redis_cluster")]
    redis_clusters: Vec<RedisClusterConfig>,
}

#[derive(Deserialize)]
struct MySQLConfig {
    host: String,
    port: String,
    username: String,
    password: String,
    dbname: String,
}

#[derive(Deserialize)]
struct RedisClusterConfig {
    #[allow(dead_code)]
    host: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("env::current_dir() = {:?}", std::env::current_dir()?);
    let config = Config::new()?;
    dbg!(config.database_url(AppEnv::Production));
    assert_eq!(config.test.redis_clusters.len(), 2);
    Ok(())
}