mod error;
mod result;

#[tokio::main]
async fn main() {
    let server = HttpExportServer::new();

    server.start().await;
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

use actix_files::NamedFile;
use actix_web::{HttpResponse, Responder};
struct HttpExportServer {}

impl HttpExportServer {
    fn new() -> HttpExportServer {
        HttpExportServer {}
    }

    async fn start(&self) {
        use actix_web::{App, HttpServer, web};

        HttpServer::new(|| {
            App::new()
                .route("/", web::to(Self::hello))
                .service(actix_files::Files::new("/git", "/home/code").show_files_listing())
        })
        .bind(("0.0.0.0", 8080))
        .expect("")
        .run()
        .await
        .expect("")
    }

    async fn handle_request(
        req: actix_web::HttpRequest,
    ) -> actix_web::Result<actix_files::NamedFile> {
        let path: std::path::PathBuf = req.match_info().query("filename").parse().unwrap();
        Ok(NamedFile::open(path)?)
    }

    async fn hello() -> impl Responder {
        HttpResponse::Ok().body("Hello world!")
    }
}
