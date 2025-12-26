use super::data_type::DataType;

#[derive(Debug, Clone)]
pub enum Context {
    Function { return_data: DataType },
    None,
}
