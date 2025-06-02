use crate::result::Result;
use snafu::ResultExt;
use std::collections::HashMap;
use std::fs;
use std::process::Stdio;
use std::time;
use tokio::process;
use uuid;

pub struct Script {
    script_path: String,
    log_path: String,
}

unsafe impl Sync for Script {

}

impl Script {
    pub fn new(script_path: String, log_path: String) -> Script {
        Script {
            script_path,
            log_path,
        }
    }

    pub async fn git(
        &self,
        upstream_url: &str,
        working_dir: &str,
        timeout: time::Duration,
        proxy: &Option<String>,
    ) -> Result<u128> {
        let task_id = uuid::Uuid::new_v4().as_u128();

        let timeout_in_second = format!("{}", timeout.as_secs());
        let mut envs = HashMap::from([
            ("TUNASYNC_UPSTREAM_URL", upstream_url),
            ("TUNASYNC_WORKING_DIR", working_dir),
            ("TUNASYNC_TIMEOUT", &timeout_in_second),
        ]);

        if let Some(proxy) = proxy {
            envs.insert("TUNASYNC_PROXY", proxy);
        }

        let command_log_path = format!("{}/{}", self.log_path, task_id);
        let command_path = format!("{}/git.sh", self.script_path);

        let command_log_file =
            fs::File::create(command_log_path).context(crate::error::ScriptExecSnafu {
                message: format!("script create task_id {} log_file failed", task_id),
            })?;

        let mut child = process::Command::new(&command_path)
            .envs(envs)
            .stdout(command_log_file)
            .stdin(Stdio::null())
            .spawn()
            .context(crate::error::ScriptExecSnafu {
                message: format!("script start {} command failed", command_path),
            })?;

        let exit_status = child.wait().await.context(crate::error::ScriptExecSnafu {
            message: format!("script run {} command failed", command_path),
        })?;

        match exit_status.success() {
            true => Ok(task_id),
            false => match exit_status.code() {
                Some(code) => todo!(),
                None => todo!(),
            },
        }
    }
}
