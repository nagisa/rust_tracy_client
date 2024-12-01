mod wgpu_frame_images;

struct ExampleDesc {
    name: &'static str,
    description: &'static str,
    function: fn(),
}

const EXAMPLES: &[ExampleDesc] = &[ExampleDesc {
    name: "wgpu_frame_images",
    description: "Demonstrates capturing frame images with wgpu",
    function: wgpu_frame_images::main,
}];

fn main() {
    let example_name = std::env::args().nth(1);
    if let Some(example) = EXAMPLES
        .iter()
        .find(|e| Some(e.name) == example_name.as_deref())
    {
        (example.function)();
    } else {
        if let Some(name) = example_name {
            eprintln!("Example {name} not found!");
        }
        println!("Available examples:");
        for example in EXAMPLES {
            println!("{}: {}", example.name, example.description);
        }
    }
}
