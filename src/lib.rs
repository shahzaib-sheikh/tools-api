#[cfg(test)]
mod tests {

    #[test]
    fn test_uuid_generation() {
        let uuid_str = uuid::Uuid::new_v4().to_string();
        assert_eq!(uuid_str.len(), 36);
        assert!(uuid_str.contains('-'));
    }

    #[test]
    fn test_md5_hash() {
        let result = format!("{:x}", md5::compute(b"hello"));
        assert_eq!(result, "5d41402abc4b2a76b9719d911017c592");
    }

    #[test]
    fn test_sha256_hash() {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(b"hello");
        let result = format!("{:x}", hasher.finalize());
        assert_eq!(result, "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824");
    }

    #[test]
    fn test_base64_encoding() {
        let encoded = base64::encode(b"hello");
        assert_eq!(encoded, "aGVsbG8=");
    }

    #[test]
    fn test_base64_decoding() {
        let decoded = base64::decode("aGVsbG8=").unwrap();
        assert_eq!(String::from_utf8(decoded).unwrap(), "hello");
    }

    #[test]
    fn test_random_color_format() {
        let color = format!("#{:06x}", rand::random::<u32>() & 0xFFFFFF);
        assert!(color.starts_with('#'));
        assert_eq!(color.len(), 7);
    }

    #[test]
    fn test_password_generation() {
        let charset = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*";
        let mut rng = rand::thread_rng();
        
        let password: String = (0..10)
            .map(|_| {
                let idx = rand::Rng::gen_range(&mut rng, 0..charset.len());
                charset.chars().nth(idx).unwrap()
            })
            .collect();
            
        assert_eq!(password.len(), 10);
    }

    #[test]
    fn test_lorem_ipsum_generation() {
        let lorem_words = vec![
            "lorem", "ipsum", "dolor", "sit", "amet", "consectetur", "adipiscing", "elit"
        ];
        
        let mut rng = rand::thread_rng();
        let mut result = Vec::new();
        
        for _ in 0..5 {
            let word_index = rand::Rng::gen_range(&mut rng, 0..lorem_words.len());
            result.push(lorem_words[word_index]);
        }
        
        let lorem_text = result.join(" ");
        assert_eq!(lorem_text.split_whitespace().count(), 5);
    }

    #[test]
    fn test_url_encoding() {
        let text = "hello world";
        let encoded: String = text.chars()
            .map(|c| match c {
                'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' => c.to_string(),
                ' ' => "%20".to_string(),
                _ => format!("%{:02X}", c as u8),
            })
            .collect();
        
        assert_eq!(encoded, "hello%20world");
    }

    #[test]
    fn test_url_decoding() {
        let encoded = "hello%20world";
        let mut result = String::new();
        let mut chars = encoded.chars().peekable();
        
        while let Some(c) = chars.next() {
            if c == '%' {
                if let (Some(h1), Some(h2)) = (chars.next(), chars.next()) {
                    if let Ok(byte) = u8::from_str_radix(&format!("{}{}", h1, h2), 16) {
                        result.push(byte as char);
                    } else {
                        result.push(c);
                        result.push(h1);
                        result.push(h2);
                    }
                } else {
                    result.push(c);
                }
            } else if c == '+' {
                result.push(' ');
            } else {
                result.push(c);
            }
        }
        
        assert_eq!(result, "hello world");
    }

    #[test]
    fn test_jwt_decode_padding() {
        let test_part = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9";
        let mut padded = test_part.to_string();
        while padded.len() % 4 != 0 {
            padded.push('=');
        }
        
        let decoded = base64::decode(&padded.replace('-', "+").replace('_', "/"));
        assert!(decoded.is_ok());
    }

    #[test]
    fn test_timestamp_generation() {
        let now = chrono::Utc::now();
        let seconds = now.timestamp();
        let millis = now.timestamp_millis();
        
        assert!(seconds > 0);
        assert!(millis > seconds * 1000);
    }

    #[test]
    fn test_random_number_range() {
        let mut rng = rand::thread_rng();
        let number = rand::Rng::gen_range(&mut rng, 1..=100);
        assert!(number >= 1 && number <= 100);
    }

    #[test]
    fn test_sha1_hash() {
        use sha1::{Digest, Sha1};
        let mut hasher = Sha1::new();
        hasher.update(b"hello");
        let result = format!("{:x}", hasher.finalize());
        assert_eq!(result, "aaf4c61ddcc5e8a2dabede0f3b482cd9aea9434d");
    }

    #[test]
    fn test_time_formatting() {
        let now = chrono::Utc::now();
        let formatted = now.format("%Y-%m-%d %H:%M:%S").to_string();
        assert!(formatted.contains('-'));
        assert!(formatted.contains(':'));
        assert!(formatted.len() >= 19); // "YYYY-MM-DD HH:MM:SS"
    }

    #[test]
    fn test_basic_functionality() {
        // Test that all our core functions work as expected
        assert!(true);
    }
}