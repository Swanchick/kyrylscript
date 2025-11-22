use super::ks_path::KsPath;

pub enum PathType {
    Root(KsPath),
    Super(KsPath),
}
