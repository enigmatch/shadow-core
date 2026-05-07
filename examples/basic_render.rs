use shadow_core::PromptTemplate;

fn main() {
    let template = PromptTemplate::new("Hello {user_name}, I am {shadow_name}.");
    let rendered = template.render(&[("user_name", "Yuki"), ("shadow_name", "Kage")]);

    println!("{rendered}");
}
