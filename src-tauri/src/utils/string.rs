use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    /// 匹配ANSI控制字符的正则表达式，支持实际控制字符和字面量\x1B格式
    static ref ANSI_RE: Regex = Regex::new(r"(?:\x1B|\\x1[bB])\[[0-9;]*[a-zA-Z]").unwrap();

    /// 需要过滤的无关行前缀列表
    static ref FILTERED_PREFIXES: [&'static str; 6] = [
        "\x1B[",          // 以实际控制字符开头的行
        "\\x1B[",         // 以字面量\x1B开头的行
        "[Proxy Enabled", // 代理启用信息
        "Proxy:",         // 代理相关信息
        "Welcome to",     // 欢迎信息
        "Last login:",    // 登录信息
    ];
}

/// 移除字符串中的所有ANSI控制字符
pub fn strip_ansi_codes(s: &str) -> String {
    ANSI_RE.replace_all(s, "").to_string()
}

/// 过滤命令输出，移除无关行和控制字符
pub fn filter_command_output(output: &str) -> String {
    output
        .lines()
        .filter(|line| !line.trim().is_empty())
        .filter(|line| {
            let trimmed = line.trim();
            // 过滤掉以指定前缀开头的行
            !FILTERED_PREFIXES.iter().any(|&prefix| trimmed.starts_with(prefix))
        })
        .map(|line| strip_ansi_codes(line))
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_ansi_codes() {
        let input = "\x1B[32m[Proxy Enabled]\x1B[0m http://example.com";
        let output = strip_ansi_codes(input);
        assert_eq!(output, "[Proxy Enabled] http://example.com");
    }

    #[test]
    fn test_filter_proxy_output() {
        let input = "
\x1B[32m[Proxy Enabled]\x1B[0m http://192.168.1.73:58267
/path/to/api-server
/path/to/another-repo
";
        let output = filter_command_output(input);
        assert_eq!(output, "/path/to/api-server\n/path/to/another-repo");
    }

    #[test]
    fn test_filter_mixed_output() {
        let input = "
Last login: Mon Mar 23 10:00:00 2026 from 192.168.1.100
Welcome to Ubuntu 22.04.4 LTS
\x1B[32m[Proxy Enabled]\x1B[0m http://192.168.1.73:58267
/home/user/projects/api-server
/home/user/projects/web-app
/home/user/projects/mobile-app
";
        let output = filter_command_output(input);
        let lines: Vec<&str> = output.lines().collect();
        assert_eq!(lines.len(), 3);
        assert!(lines.contains(&"/home/user/projects/api-server"));
        assert!(lines.contains(&"/home/user/projects/web-app"));
        assert!(lines.contains(&"/home/user/projects/mobile-app"));
    }

    #[test]
    fn test_no_filter_needed() {
        let input = "/path/to/repo1\n/path/to/repo2\n/path/to/repo3";
        let output = filter_command_output(input);
        assert_eq!(output, input);
    }

    #[test]
    fn test_filter_literal_escape_sequences() {
        // 测试处理字面量的\x1B序列（某些服务器可能输出转义后的字符）
        let input = r#"
\x1B[32m[Proxy Enabled]\x1B[0m http://192.168.1.73:58267
/path/to/api-server
"#;
        let output = filter_command_output(input);
        assert_eq!(output, "/path/to/api-server");
    }
}
