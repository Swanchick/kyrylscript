use ks_global::utils::ks_result::KsResult;

type NativeFunction = fn() -> KsResult<()>;

pub struct NativeRegistry {
    pub functions: Vec<NativeFunction>,
}
