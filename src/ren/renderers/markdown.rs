use crate::prelude::*;
use crate::utils::filesystem::copy_dir_recursive;
use crate::ren::manifest::TemplateManifest;
use crate::ren::RenderQueue;
use crate::tuack_lib::ren::base::Checker;
use crate::tuack_lib::ren::base::Compiler;
use tuack_ng_parser::printers::render_markdown;

pub struct MarkdownChecker {}

impl Checker for MarkdownChecker {
    fn new(_: PathBuf) -> Self {
        MarkdownChecker {}
    }

    fn check_compiler(&self) -> Result<()> {
        Ok(())
    }
}

pub struct MarkdownCompiler {
    pub day_config: ContestDayConfig,
    pub tmp_dir: PathBuf,
    pub renderqueue: Vec<RenderQueue>,
}

impl Compiler for MarkdownCompiler {
    fn new(
        _: ContestConfig,
        day_config: ContestDayConfig,
        tmp_dir: PathBuf,
        renderqueue: Vec<RenderQueue>,
        _manifest: TemplateManifest,
    ) -> Self {
        MarkdownCompiler {
            day_config,
            tmp_dir,
            renderqueue,
        }
    }

    fn compile(&self) -> Result<PathBuf> {
        let output_dir = &self.tmp_dir.join("output").join(&self.day_config.name);
        if !output_dir.exists() {
            fs::create_dir_all(output_dir)?;
        }
        for item in &self.renderqueue {
            if let RenderQueue::Problem(ast, problem_config) = item {
                let output = render_markdown(ast);
                let output_filename = format!("{}.md", problem_config.name);

                fs::write(output_dir.join(&output_filename), output)?;
                info!("生成 Markdown 文件：{}", output_filename);
            }
        }
        if self.tmp_dir.join("img").exists() {
            let target = output_dir.join("img");
            copy_dir_recursive(self.tmp_dir.join("img"), &target)?;
            info!("复制图片目录到：{}", target.display());
        }
        Ok(output_dir.parent().unwrap().to_path_buf())
    }
}
