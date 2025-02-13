use std::borrow::Cow;

use anyhow::bail;

#[derive(Clone, Debug)]
pub struct EnvironmentVariables {
    pub database_url: Cow<'static, str>,
    pub database: Cow<'static, str>,
    pub port: u16,
}

impl EnvironmentVariables {
    pub fn from_env() -> anyhow::Result<Self> {
        dotenv::dotenv().ok();

        Ok(Self {
            database_url: match dotenv::var("DATABASE_URL") {
                Ok(url) => url.into(),
                Err(err) => bail!("missing DATABASE_URL: {err}"),
            },
            port: match dotenv::var("PORT") {
                Ok(port) => port.parse()?,
                _ => 8000,
            },
            database: match dotenv::var("DATABASE") {
                Ok(name) => name.into(),
                Err(err) => bail!("missing DATABASE: {err}"),
            },
        })
    }
}
