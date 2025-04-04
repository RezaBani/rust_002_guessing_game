**Simple Guessing Game**

this is a simple guessing game inspired y the rust lang book with only 2 diffrence:

1. random number generated via zig that is hosted in this repo:

https://github.com/RezaBani/zig_004_random_number_generator

your folder structure should be something like this:

..Parent Folder

....Rust

......rust_002_guessing_game

....Zig

......zig_004_random_number_generator

in fact you can easily setup this:

`$ mkdir GuessingGame && cd GuessingGame`

`& mkdir Zig && cd Zig`

`$ git clone https://github.com/RezaBani/zig_004_random_number_generator`

`$ cd ../ && mkdir Rust && cd Rust`

`$ git clone https://github.com/RezaBani/rust_002_guessing_game`

change directory to the game and run it with cargo:

`& cd rust_002_guessing_game`

`$ cargo run`

Note: it will take some time to run for the first time since it's building it's dependency as well.

But source code is more beautiful than the game itself :)