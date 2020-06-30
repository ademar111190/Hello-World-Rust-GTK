# Writing my firs Rust app and my first GTK at same time

This repo contains the code of my first rust app as well as my first GTK app, 

# How the app looks like

TODO add screenshots

# What I did, by steps

## 1º Install dependencies

You need to install both Rust and GTK

### Installing Rust

To install rust follow the steps [here at rustup](https://rustup.rs/), at the time I wrote this that is the command:

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Installing GTK

If you are on linux, probably you already have the GTK, if not check out the installations guides for your distribution, on macOS you have to install the following package:

```
brew install gtk+3
```

## 2º Start a project

```
cargo init hello-world ; cd hello-world ; cargo run
```

if everything works you should see it:

```
   Compiling hello-world v0.1.0 (/path/to/hello-world)
    Finished dev [unoptimized + debuginfo] target(s) in 0.35s
     Running `target/debug/hello-world`
Hello, world!
```

## 3º Add dependencies

Open the `Cargo.toml` file and add the following dependencies.

```
[package]
name = "hello-world"
version = "0.1.0"
authors = ["Your name <your@email>"]
edition = "2018"

[dependencies.gdk]
version = "0.12.1"

[dependencies.glib]
version = "0.9.3"

[dependencies.gtk]
version = "0.8.1"
features = ["v3_16"]

[dependencies.gio]
version = ""
features = ["v2_44"]
```

After that, run again with `cargo run` you should see your dependencies being download and after that the output should be the same as the previous step.

## 4º Add dependencies


