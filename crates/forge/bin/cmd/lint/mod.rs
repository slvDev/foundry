use std::{fs, io::Read, path::Path};

use clap::Parser;
use eyre::Result;
use foundry_cli::{
    opts::CoreBuildArgs,
    utils::LoadConfig,
};

use foundry_common::compile::ProjectCompiler;

mod detectors;

#[derive(Debug, Parser)]
pub struct LintArgs{
    #[clap(flatten)]
    pub args: CoreBuildArgs,

    // Implement args:

    // include sol
    // exclude sol
    // scope ? or create it from include and exclude
    // output -> console or file
    // only servenity
    // exclude servenity

}

impl LintArgs {
    pub fn run(self) -> Result<()> {
        let config = self.args.try_load_config_emit_warnings()?;
        let project = config.project()?;
        
        let sources_path = project.sources_path().clone();
        let src_dir_filter = Box::new(move |path: &Path| {path.starts_with(&sources_path)});

        let compiler = ProjectCompiler::new()
            .quiet(true)
            .filter(src_dir_filter);

        let output = compiler.compile(&project)?;

        // track file names -> probably should be better solution how to get ast from file
        let mut visited_sources = Vec::new();

        for (id, artifact) in output.artifact_ids() {
            let loc = id.source.to_string_lossy().to_string();
            if !visited_sources.contains(&loc) {
                // debug
                println!("File loc: {}", &loc);
                // read code from file to print the output.
                // can avoid this read?
                let mut file = fs::File::open(id.source)?;
                let mut code = String::new();
                file.read_to_string(&mut code)?;

                if let Some(ast) = &artifact.ast {
                    // Run all detectors
                    detectors::run_all_detectors(&code, ast);
                }

                visited_sources.push(loc);
            }
        }

        Ok(())
    }
}