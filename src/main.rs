use resolver::resolve;

mod dependency;
mod domain;
mod resolver;

fn main() {
    let result = resolve();
    println!("{}", result.message);
}
