use log::info;
use rust_embed::Embed;

#[derive(Embed)]
#[folder = "src"]
pub struct OurSource;

#[derive(Embed)]
#[folder = "src/bin"]
pub struct OurSourceBin;

#[derive(Embed)]
#[folder = "src/extractor"]
pub struct OurSourceExtractor;

#[derive(Embed)]
#[folder = "src/extractor/components"]
pub struct OurSourceExtractorComponents;

#[derive(Embed)]
#[folder = "src/extractor/model"]
pub struct OurSourceExtractorModel;

#[derive(Embed)]
#[folder = "src/extractor/system"]
pub struct OurSourceExtractorSystem;

#[derive(Embed)]
#[folder = "src/model"]
pub struct OurSourceModel;

#[derive(Embed)]
#[folder = "src/model/git"]
pub struct OurSourceModelGit;

#[derive(Embed)]
#[folder = "src/model/lean"]
pub struct OurSourceModelLean;

#[derive(Embed)]
#[folder = "src/model/lean/types"]
pub struct OurSourceModelLeanTypes;

#[derive(Embed)]
#[folder = "src/model/math"]
pub struct OurSourceModeMath;

#[derive(Embed)]
#[folder = "src/playground"]
pub struct OurSourcePlayground;

#[derive(Embed)]
#[folder = "src/state"]
pub struct OurSourceState;

#[derive(Embed)]
#[folder = "src/views"]
pub struct OurSourceView;

#[derive(Embed)]
#[folder = "src/views/component_memes"]
pub struct OurSourceViewComponent;

#[derive(Embed)]
#[folder = "src/views/crypto_frontend"]
pub struct OurSourceViewCrypto;

#[derive(Embed)]
#[folder = "src/views/extras_views"]
pub struct OurSourceViewextra;

#[derive(Embed)]
#[folder = "src/views/wikidata_memes"]
pub struct OurSourceViewWikwidata;

#[derive(Embed)]
#[folder = "src/views/workflow_memes"]
pub struct OurSourceViewWorkflow;

pub fn printall() {
    info!("PRINT ALL");
    for file in OurSource::iter() {
        println!("{}", file.as_ref());
        info!("print {}", file.as_ref());
    }
}
