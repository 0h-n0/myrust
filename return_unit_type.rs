fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(());
    let e = Box::new(Err::<(), String>("Hello world".to_string()));
    Box::new(Err::<(), Box<dyn std::error::Error>>(e));
}
