use serde::{Deserialize, Serialize};
use snafu::ResultExt;
use std::{collections::HashMap, fs, sync::Arc};
use serde_yml::Value;
use crate::{error, result};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    script_path: String,
    script_log_path: String,
    tasks: Vec<TaskConfig>,
}

impl Config {
    pub fn get_script_path(&self) -> &String {
        return &self.script_path;
    }

    pub fn get_secript_log_path(&self) -> &String {
        return &self.script_log_path;
    }

    pub fn get_tasks(&self) -> &Vec<TaskConfig> {
        return &self.tasks;
    }
}

impl Config {
    pub async fn parse(config_path: String) -> result::Result<Arc<Config>> {
        let conf: Arc<Config> = Arc::new(
            serde_yml::from_reader(fs::File::open(&config_path).context(
                crate::error::ConfigFileReadSnafu {
                    message: format!("Config parse {config_path} failed"),
                },
            )?)
            .context(error::ConfigParseSnafu {
                message: format!(""),
            })?,
        );

        return Ok(conf);
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all_fields = "snake_case")]
pub enum TaskType {
    Git {
        upstream_url: String,
        timeout_in_second: u64,
        proxy: Option<String>,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskConfig {
    key: String,
    task_type: String,
    rest_params: HashMap<String, serde_yml::Value>,
}

impl TaskConfig {
    pub fn get_key(&self) -> &String {
        return &self.key;
    }
    pub fn get_task_type(&self) -> &String {
        return &self.task_type;
    }
}
