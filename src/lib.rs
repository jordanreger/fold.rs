pub fn fold(str: &'static str, len: Option<usize>) -> String {
    let length = match len {
        Some(length) => length,
        None => 79,
    };

    let mut strarr: Vec<String> = vec![];
    let mut strlen = 0;

    for word in str.split(" ") {
        let wlen = word.len() + 1;

        if strlen + wlen > length {
            strarr.push("\n".to_string());
            strarr.push(word.to_string());
            strarr.push(" ".to_string());
            strlen = wlen;
        } else if strlen + wlen == length {
            strarr.push(word.to_string());
            strarr.push("\n".to_string());
            strarr.push(" ".to_string());
            strlen = wlen;
        } else {
            strarr.push(word.to_string());
            strarr.push(" ".to_string());
            strlen += wlen;
        }
    }

    let res: String = strarr.join("");
    return res;
}
