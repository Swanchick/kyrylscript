use crate::utils::VMError;

pub type CaptureSize = usize;
pub type Offset = i32;
pub type VariableId = u32;
pub type Pointer = usize;

pub type Slot = u32;

pub type CollectionId = u32;
pub type StorageId = u32;
pub type Owners = u32;

pub type NativeId = u32;
pub type Arguments = u32;

pub type VMResult<T> = Result<T, VMError>;
