pub trait DispatchableCommand {
    fn execute(&self) -> Result<Option<String>, String>;
}
