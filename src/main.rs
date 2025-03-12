mod error;
mod result;

fn main() {
    println!("Hello, world!");
}

trait Executor {
    fn key(&self) -> String;

    // fn check_conf(&self) -> crate::result::Result<()>;

    async fn sync(&self) -> crate::result::Result<()>;

    // async fn service(&self) -> crate::result::Result<()>;
}

struct GitExecutorConfig {
    upstream_url: String,
    working_dir: String,
    timeout_in_second: i32,
}

struct GitExecutor {
    conf: GitExecutorConfig,
}

use std::collections::{HashMap, HashSet};
use tokio::process;

impl Executor for GitExecutor {
    fn key(&self) -> String {
        return String::from("git");
    }

    async fn sync(&self) -> crate::result::Result<()> {
        let mut child = process::Command::new("/scripts/git.sh")
            .envs(HashMap::from([
                ("TUNASYNC_UPSTREAM_URL", &self.conf.upstream_url),
                ("TUNASYNC_WORKING_DIR", &self.conf.working_dir),
                ("TUNASYNC_TIMEOUT", &self.conf.timeout_in_second.to_string()),
            ]))
            .spawn()
            .expect("");

        let exit_status = child.wait().await.expect("");

        if exit_status.success() {
            return Ok(());
        } else {
        }
        todo!()
    }
}

struct ExecutorContext {
    http_expoter: HttpExporter,
}

struct HttpExporter {
    namespaces: HashSet<String>,
}

impl HttpExporter {
    fn export_http(&self, namespace: String) -> result::Result<()> {
        if self.namespaces.contains(&namespace) {}
        todo!()
    }
}

struct TcpExport {}

struct HttpExportServer {}

impl HttpExportServer {
    async fn start(&self) {
        use actix_web::{App, HttpServer, web};

        HttpServer::new(|| App::new().route("/git/*", web::to(Self::handle_request)))
            .bind(("127.0.0.1", 8080))
            .expect("")
            .run()
            .await
            .expect("")
    }

    async fn handle_request(
        req: actix_web::HttpRequest,
    ) -> std::result::Result<actix_files::NamedFile, actix_web::Error> {
        todo!()
    }
}
