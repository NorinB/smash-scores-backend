/// Needed to convert Input Objects (Request Objects) to ready to be saved objects for the storage
pub trait InputToSimpleObjectConvertible<SimpleObject> {
    fn to_simple_object(&self) -> SimpleObject;
}
