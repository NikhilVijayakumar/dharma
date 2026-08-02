pub fn matches_glob(pattern: &str, path: &str) -> bool {
    let pat_segments: Vec<&str> = pattern.split('/').collect();
    let path_segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
    match_segments(&pat_segments, &path_segments, 0, 0)
}

fn match_segments(pat: &[&str], parts: &[&str], pi: usize, si: usize) -> bool {
    if pi >= pat.len() {
        return si >= parts.len();
    }
    if pat[pi] == "**" {
        if pi + 1 >= pat.len() {
            return true;
        }
        for i in si..=parts.len() {
            if match_segments(pat, parts, pi + 1, i) {
                return true;
            }
        }
        return false;
    }
    if si >= parts.len() {
        return false;
    }
    if segment_match(pat[pi], parts[si]) && match_segments(pat, parts, pi + 1, si + 1) {
        return true;
    }
    false
}

fn segment_match(pattern: &str, segment: &str) -> bool {
    if pattern == "*" {
        return true;
    }
    if !pattern.contains('*') && !pattern.contains('?') {
        return pattern == segment;
    }
    let pattern_bytes = pattern.as_bytes();
    let segment_bytes = segment.as_bytes();
    let mut pi = 0;
    let mut si = 0;
    let mut star_match_pos: Option<(usize, usize)> = None;

    loop {
        if pi >= pattern_bytes.len() {
            return si >= segment_bytes.len();
        }
        match pattern_bytes[pi] {
            b'*' => {
                pi += 1;
                star_match_pos = Some((pi, si));
                while pi < pattern_bytes.len() && pattern_bytes[pi] == b'*' {
                    pi += 1;
                }
            }
            b'?' => {
                if si >= segment_bytes.len() {
                    return false;
                }
                pi += 1;
                si += 1;
            }
            c => {
                if si < segment_bytes.len() && segment_bytes[si] == c {
                    pi += 1;
                    si += 1;
                } else if let Some((saved_pi, saved_si)) = star_match_pos {
                    if saved_si >= segment_bytes.len() {
                        return false;
                    }
                    pi = saved_pi;
                    si = saved_si + 1;
                    star_match_pos = Some((saved_pi, si));
                } else {
                    return false;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_literal_exact() {
        assert!(matches_glob("standards", "standards"));
    }

    #[test]
    fn test_dir_glob_does_not_match_file() {
        assert!(!matches_glob("**/standards/**", "standards.md"));
        assert!(!matches_glob("**/.dharma/**", ".dharma.md"));
    }

    #[test]
    fn test_dir_glob_matches_under_directory() {
        assert!(matches_glob("**/.dharma/**", ".dharma/assets/skill.py"));
        assert!(matches_glob("**/target/**", "crates/common/target/debug/x"));
    }

    #[test]
    fn test_file_glob_exact() {
        assert!(matches_glob("**/test.md", "test.md"));
        assert!(matches_glob("**/test.md", "subdir/test.md"));
        assert!(!matches_glob("**/test.md", "test.md2"));
    }

    #[test]
    fn test_wildcard_segment() {
        assert!(matches_glob("*.md", "test.md"));
        assert!(!matches_glob("*.md", "test.md.txt"));
    }

    #[test]
    fn test_mixed_pattern() {
        assert!(matches_glob("docs/**/*.md", "docs/raw/standards/arch.md"));
        assert!(!matches_glob("docs/**/*.md", "docs/raw/standards/arch.txt"));
    }

    #[test]
    fn test_star_in_middle() {
        assert!(matches_glob("docs/*/foo.md", "docs/raw/foo.md"));
        assert!(!matches_glob("docs/*/foo.md", "docs/raw/sub/foo.md"));
    }
}
