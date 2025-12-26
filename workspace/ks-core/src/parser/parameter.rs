use super::data_type::DataType;

#[derive(PartialEq, Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub data_type: DataType,
}
