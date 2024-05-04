use deno_core::error::AnyError;
use deno_core::op2;

#[op2]
#[serde]
fn op_listdir(#[string] path: Option<String>) -> Result<Vec<String>, AnyError> {
    let path = path.unwrap_or_else(|| ".".into());
    let dir = std::fs::read_dir(path)?;

    Ok(dir
        .into_iter()
        .flatten()
        .flat_map(|item| item.file_name().into_string())
        .collect())
}

#[op2(async)]
#[string]
async fn op_read_file(#[string] path: String) -> Result<String, AnyError> {
    let contents = tokio::fs::read_to_string(path).await?;
    Ok(contents)
}

#[op2(async)]
async fn op_write_file(#[string] path: String, #[string] contents: String) -> Result<(), AnyError> {
    tokio::fs::write(path, contents).await?;
    Ok(())
}

#[op2(fast)]
fn op_remove_file(#[string] path: String) -> Result<(), AnyError> {
    std::fs::remove_file(path)?;
    Ok(())
}

deno_core::extension!(
    fs_ext,
    ops = [op_read_file, op_write_file, op_remove_file, op_listdir]
);
