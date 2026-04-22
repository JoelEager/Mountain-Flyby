# Mountain Flyby
A procedurally generated graphics demo implemented in Rust. Targets [Vulkan](https://docs.vulkan.org/guide/latest/what_is_vulkan.html)
and tested on an AMD RX 6750, though it should compatible with most GPUs. 
Developed using [Google Jules](https://jules.google.com/).

## Setup
The `ash` crate requires that the Vulkan API be present on the path. For 
installation instructions see the crate's read me [here](https://docs.vulkan.org/guide/latest/what_is_vulkan.html).

Once that is done you can compile and run the application via `cargo run`.
