// When we call this macro as in the test, we get a syntax error because
// $msg:expr will match everything inside the parentheses, which is not a valid
// expr. Rust will not move on to the next rule in case of syntax error.
macro_rules! bad_complain {
    ($msg:expr) => {
        println!("Complaint filed: {}", $msg)
    };
    (user : $userid:tt, $msg:expr) => {
        println!("Complaint from user {}: {}", $userid, $msg);
    };
}

// One way is to avoid using confusable rules, for example by using different
// identifiers in each pattern.
macro_rules! complain1 {
    (msg : $msg:expr) => {
        println!("Complaint filed: {}", $msg);
    };
    (user : $userid:tt, $msg:expr) => {
        println!("Complaint from user {}: {}", $userid, $msg);
    };
}

// Another way is to put more specific rules first
macro_rules! complain2 {
    (user : $userid:tt, $msg:expr) => {
        println!("Complaint from user {}: {}", $userid, $msg);
    };
    ($msg:tt) => {
        println!("Complaint filed: {}", $msg);
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_complain1() {
        complain1!(user: "jimb", "the AI lab's chatbots keep picking on me");
        complain1!(msg: "Complaint");
    }

    #[test]
    fn test_complain2() {
        complain2!(user: "jimb", "the AI lab's chatbots keep picking on me");
        complain2!("Complaint");
    }
}
