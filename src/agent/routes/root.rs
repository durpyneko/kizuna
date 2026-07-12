pub async fn root() -> String {
    format!(
        "heyy, you've found me, hello!\n\n {}",
        include_str!("../../../misc/image.txt")
    )
}
