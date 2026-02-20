use anyhow::Result;

pub fn encrypt(
    input: String,
    output: String,
    user_password: String,
    owner_password: Option<String>,
) -> Result<()> {
    pdfx_lib::encrypt(&input, &output, &user_password, owner_password.as_deref())
}

pub fn decrypt(input: String, output: String, password: Option<String>) -> Result<()> {
    pdfx_lib::decrypt(&input, &output, password.as_deref())
}
