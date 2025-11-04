pub fn translate(input: &str) -> String {
    let mut words: Vec<String> = vec![];
    let mut word: String; // input.to_string();
    let mut offset: usize;
    let mut remains: String;
    // remains.push_str(&word);
    for w in input.split_whitespace() {
        word = w.to_string();
        println!("{w}");
        match w {
            // Rule 1 If a word begins with a vowel, or starts with `"xr"` or `"yt"`,
            // add an `"ay"` sound to the end of the word.
            w if w.starts_with(['a', 'e', 'i', 'o', 'u']) => word.push_str("ay"),
            w if w.starts_with("xr") => word.push_str("ay"),
            w if w.starts_with("yt") => word.push_str("ay"),
            w if !w.starts_with(['a', 'e', 'i', 'o', 'u']) => {
                // Rule 3. // If a word starts with zero or more consonants followed by
                // `"qu"`, first move those consonants (if any) and the `"qu"` part to
                //the end of the word, and then add an `"ay"` sound to the end of the word.
                if word.contains("qu") {
                    offset = word.find("qu").unwrap_or(word.len());
                    remains = word.drain(offset + 2..).collect();
                    remains.push_str(&word);
                    remains.push_str("ay");
                    word = remains;
                } else {
                    // Rule 4. If a word starts with one or more consonants followed
                    // by `"y"`, first move the consonants preceding the `"y"`to the
                    // end of the word, and then add an `"ay"` sound to the end of
                    // the word.
                    if !word.starts_with("y") {
                        offset = word
                            .find(['a', 'e', 'i', 'o', 'u', 'y'])
                            .unwrap_or(word.len());
                    } else {
                        // Rule 2. If a word begins with a one or more consonants,
                        // first move those consonants to the end of the word and
                        // then add an `"ay"` sound to the end of the word.

                        offset = word.find(['a', 'e', 'i', 'o', 'u']).unwrap_or(word.len());
                    }
                    remains = word.drain(..offset).collect();
                    word.push_str(&remains);
                    word.push_str("ay");
                }
            }
            _ => (),
        }
        words.push(word);
    }
    words.join(" ")
}
