pub mod pr1;
pub mod pr2;

fn main() {
    let mut pr = pr1::PullRequest::new("repo".to_string(), "topic".to_string(), "main".to_string());
    println!("PR: {:?}", pr);

    pr.open();
    println!("PR: {:?}", pr);

    pr.approve("Alice".to_string(), "Looks good to me".to_string());
    println!("PR: {:?}", pr);

    pr.close("Merged".to_string());
    println!("PR: {:?}", pr);
}
