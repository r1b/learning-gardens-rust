fn is_question(query: &str) -> bool {
    query.trim().ends_with('?')
}

fn is_yelling(query: &str) -> bool {
    let alpha_str : String = query.chars().filter(|c: &char| (*c).is_alphabetic()).collect();
    alpha_str.len() > 0 && alpha_str.chars().all(|c: char| c.is_uppercase())
}

fn is_nothing(query: &str) -> bool {
    query.len() == 0 || query.chars().all(|c: char| c.is_whitespace())
}

pub fn reply(query: &str) -> &str {
    if is_yelling(query) {
        return "Whoa, chill out!"
    }
    else if is_question(query) {
        return "Sure."
    }
    else if is_nothing(query) {
        return "Fine. Be that way!"
    }
    else {
        return "Whatever."
    }
}
