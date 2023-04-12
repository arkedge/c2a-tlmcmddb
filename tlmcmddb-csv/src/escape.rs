pub fn escape(s: &str) -> String {
    s.replace('\n', "##").replace('\r', "%%").replace(',', "@@")
}

pub fn unescape(s: &str) -> String {
    s.replace("##", "\n").replace("%%", "\r").replace("@@", ",")
}
