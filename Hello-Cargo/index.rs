
// create a new project using Cargo and look at how it differs 
// from our original “Hello, world!” project.
// $ cargo new hello_cargo
// $ cd hello_cargo

// Git files won’t be generated if you run cargo new within an existing
//  Git repository; you can override this behavior by 
//  using cargo new --vcs=git

// Note: Git is a common version control system. 
// You can change cargo new to use a different version control system or no 
// version control system by using the --vcs flag. 
// Run cargo new --help to see the available options.


//Cargo.toml -> This file is in the TOML (Tom’s Obvious, Minimal Language) format,
// which is Cargo’s configuration format.


// [package]
// name = "hello_cargo"
// version = "0.1.0"
// edition = "2021"
// [dependencies]

// The first line, [package], is a section heading that indicates
//  that the following statements are configuring a package. 
// As we add more information to this file, we’ll add other sections.

// The last line, [dependencies], is the start of a section
//  for you to list any of your project’s dependencies.
//  In Rust, packages of code are referred to as crates

//Goto -> src open main.rs
// the differences between our project and the project 
// Cargo generated are that Cargo placed the code in 
// the src directory and we have a 
// Cargo.toml configuration file in the top directory.


// If you started a project that doesn’t use Cargo,
//  as we did with the “Hello, world!” project, 
//  you can convert it to a project that does use Cargo. 
// Move the project code into the
//  src directory and create an appropriate Cargo.toml file

// Building and Running a Cargo Project
// From your hello_cargo directory, 
// build your project by entering the following command:
// cargo build
// Goto target/debug/hello_cargo

// Cargo.Lock

// Running cargo build for the first time also causes 
// Cargo to create a new file at the top level: Cargo.lock.
// This file keeps track of the exact versions of dependencies in your project.
// This project doesn’t have dependencies, so the file is a bit sparse. 
// You won’t ever need to change this file manually; 
// Cargo manages its contents for you.


// We just built a project with cargo build and ran it with 
// ./target/debug/hello_cargo,
//  but we can also use cargo run to compile the code
//   and then run the resultant executable all in one

// Cargo also provides a command called cargo check.
//  This command quickly checks your code to make sure
//  it compiles but doesn’t produce an executable:

// cargo check

// cargo check is much faster than cargo build because it skips the step of producing an executable


// RECAP

// We can create a project using cargo new.
// We can build a project using cargo build.
// We can build and run a project in one step using cargo run.
// We can build a project without producing a binary to check for errors using cargo check.
// Instead of saving the result of the build in the same directory as our code, Cargo stores it in the target/debug directory.

// When your project is finally ready for release, 
// you can use cargo build --release to compile it with optimizations

// This command will create an executable in target/release 
// instead of target/debug. The optimizations make your 
// Rust code run faster,but turning them on lengthens the
// time it takes for your program to compile.

// This is why there are two different profiles: one for development,
// when you want to rebuild quickly and often, and another for building
// the final program you’ll give to a user that won’t be rebuilt
// repeatedly and that will run as fast as possible. 
// If you’re benchmarking your code’s running time, be sure to run cargo 
// build --release and benchmark with the executable in target/release.


// cargo-watch
// The cargo-watch crate is a third-party tool that allows you to
// automatically recompile your Rust project whenever the source 
// files change. It is particularly useful during development to 
// streamline the build and test process.
// it monitors your Rust project files, and when it detects changes, 
// it automatically triggers the specified build or test command. 
// If you're using it to continuously build your project, 
// it will recompile your Rust code, and if there are no errors,
//  it will update the executable file.

