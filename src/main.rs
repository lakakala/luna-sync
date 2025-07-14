mod config;
mod error;
mod result;
mod script;

use async_trait::async_trait;
use clap::Parser;
use log::info;
use script::Script;
use std::sync::Arc;
use std::time::{self, Duration};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long)]
    config: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let conf = match config::Config::parse(args.config).await {
        Ok(conf) => conf,
        Err(err) => {
            panic!("{}", err);
        }
    };

    let server = HttpExportServer::new(conf.clone());

    server.start().await;

    let exec_ctx = ExecutorContext::new(Script::new(
        conf.get_script_path().clone(),
        conf.get_secript_log_path().clone(),
    ));

    let task_server = Arc::new(TaskServer::new(conf.clone(), exec_ctx));

    task_server.start();
    println!("Hello, world!");
}

struct TaskServer {
    conf: Arc<config::Config>,
    tasks: HashMap<String, Arc<dyn Executor + Send + Sync>>,
    context: Arc<ExecutorContext>,
}

impl TaskServer {
    pub fn new(conf: Arc<config::Config>, context: Arc<ExecutorContext>) -> TaskServer {
        let mut tasks: HashMap<String, Arc<dyn Executor + Send + Sync>> = HashMap::new();
        for task_conf in conf.get_tasks() {
            match task_conf.get_task_type() {
                config::TaskType::Git {
                    upstream_url,
                    timeout_in_second,
                    proxy,
                } => tasks.insert(
                    task_conf.get_key().clone(),
                    Arc::new(GitExecutor::new(
                        upstream_url.clone(),
                        task_conf.get_key().clone(),
                        Duration::from_secs(timeout_in_second.clone()),
                        proxy.clone(),
                    )),
                ),
            };
        }
        return TaskServer {
            conf: conf,
            tasks: tasks,
            context: context,
        };
    }

    pub fn start(self: Arc<Self>) {
        self.start_task();
    }

    async fn start_task(self: Arc<Self>) {
        tokio::spawn(async {
            self.do_start_tasks().await;
        });
    }

    async fn do_start_tasks(self: Arc<Self>) {
        for (key, task) in self.tasks.clone() {
            if let Err(err) = task.sync(self.context.clone()).await {
                info!("[TaskServer] key {key} task.sync failed err {err}");
            }
        }
    }
}

struct TaskManager {
    tasks: HashMap<String, Box<dyn Executor>>
}

static TASK_MANAGER : TaskManager= TaskManager::new_task_manager();
impl TaskManager {
    fn new_task_manager() -> TaskManager {
        todo!()
    }
}

#[async_trait]
trait Executor {
    fn key(&self) -> String;

    // fn check_conf(&self) -> crate::result::Result<()>;

    async fn sync(&self, context: Arc<ExecutorContext>) -> crate::result::Result<()>;

    // async fn service(&self) -> crate::result::Result<()>;
}

struct ExecutorContext {
    script: script::Script,
}

impl ExecutorContext {
    pub fn new(script: script::Script) -> Arc<ExecutorContext> {
        return Arc::new(ExecutorContext { script: script });
    }
}

struct GitExecutor {
    upstream_url: String,
    working_dir: String,
    timeout: time::Duration,
    proxy: Option<String>,
}

impl GitExecutor {
    pub fn new(
        upstream_url: String,
        working_dir: String,
        timeout: time::Duration,
        proxy: Option<String>,
    ) -> GitExecutor {
        return GitExecutor {
            upstream_url,
            working_dir: working_dir,
            timeout: timeout,
            proxy,
        };
    }
}

use std::collections::{HashMap, HashSet};

#[async_trait]
impl Executor for GitExecutor {
    fn key(&self) -> String {
        return String::from("git");
    }

    async fn sync(&self, context: Arc<ExecutorContext>) -> crate::result::Result<()> {
        let task_id = context
            .script
            .git(
                &self.upstream_url,
                &self.working_dir,
                self.timeout,
                &self.proxy,
            )
            .await?;

        info!("git_executor sync task_id {}", task_id);

        return crate::result::Result::Ok(());
    }
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
struct HttpExportServer {
    config: Arc<config::Config>,
}

impl HttpExportServer {
    fn new(conf: Arc<config::Config>) -> HttpExportServer {
        HttpExportServer { config: conf }
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
