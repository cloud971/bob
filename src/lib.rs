

pub fn reply(message: &str) -> &str {

    let message = message.trim();

    if message.is_empty() || message.find(char::is_alphanumeric).is_none() {
        "Fine. Be that way!"
    }
    else if message.ends_with("!") && message.find(char::is_lowercase).is_none() {
        "Whoa, chill out!"
    }
    else if message.find(char::is_lowercase).is_none() && message.ends_with("?"){
        "Calm down, I know what I'm doing!"
    }
    else if message.find(char::is_lowercase).is_none(){
        "Whoa, chill out!"
    }
    else if message.ends_with("?"){
        "Sure."
    }
    else{
        "Whatever."
    }
}
