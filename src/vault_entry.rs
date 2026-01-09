pub struct VaultEntry {
    pub service: String,
    pub username: Option<String>,
    pub password: Vec<u8>,
}

impl VaultEntry {
    pub fn serialize(&self) -> Vec<u8> {
        let mut data = Vec::new();
        data.push(self.service.as_bytes().len() as u8);
        data.extend_from_slice(self.service.as_bytes());

        if let Some(username) = &self.username {
            data.push(username.as_bytes().len() as u8);
            data.extend_from_slice(username.as_bytes());
        } else {
            data.push(0u8);
        }

        data.push(self.password.len() as u8);
        data.extend_from_slice(&self.password);

        data
    }

    pub fn deserialize(data: &[u8]) -> Self {
        let mut index = 0;

        let service_len = data[index] as usize;
        index += 1;

        let service = String::from_utf8(data[index..index + service_len].to_vec()).unwrap();
        index += service_len;

        let username_len = data[index] as usize;
        index += 1;

        let username = if username_len > 0 {
            Some(String::from_utf8(data[index..index + username_len].to_vec()).unwrap())
        } else {
            None
        };
        index += username_len;

        let password_len = data[index] as usize;
        index += 1;

        let password = data[index..index + password_len].to_vec();

        VaultEntry {
            service,
            username,
            password,
        }
    }
}
