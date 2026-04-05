#[derive(Debug,Clone)]
pub struct Commit {
    hash: String,
    header: String,
    parents: Vec<String>,
    author: String,
    committer: String,
    message: String,
}

impl Commit {
    pub fn new(hash: String, raw: String) -> Commit {
        let mut lines = raw.lines();

        let mut header = String::new();
        let mut parents = Vec::new();
        let mut author = String::new();
        let mut committer = String::new();

        // читаем заголовки
        for line in &mut lines {
            if line.is_empty() {
                break; // конец заголовков
            }
            if line.starts_with("commit ") {
                header = line[5..].to_string();
            } else if line.starts_with("parent ") {
                parents.push(line[7..].to_string());
            } else if line.starts_with("author ") {
                author = line[7..].to_string();
            } else if line.starts_with("committer ") {
                committer = line[10..].to_string();
            }
        }

        // всё остальное — message
        let message = lines.collect::<Vec<_>>().join("\n");

        Commit {
            hash,
            header,
            parents,
            author,
            committer,
            message,
        }
    }
}