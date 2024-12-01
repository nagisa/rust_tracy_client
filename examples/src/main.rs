mod secondary_frames;
mod wgpu_frame_images;
mod zones;

struct ExampleDesc {
    name: &'static str,
    description: &'static str,
    function: fn(),
}

const EXAMPLES: &[ExampleDesc] = &[
    ExampleDesc {
        name: "wgpu_frame_images",
        description: "Demonstrates capturing frame images with wgpu",
        function: wgpu_frame_images::main,
    },
    ExampleDesc {
        name: "secondary_frames",
        description: "Demonstrates secondary frames, both continuous and discontinuous",
        function: secondary_frames::main,
    },
    ExampleDesc {
        name: "zones",
        description: "Demonstrates the use of zones to measure work",
        function: zones::main,
    },
];

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
