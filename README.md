This is a script written in pure Rust that solves for roots in a quadratic equation `x^2+px+q=0` using the Vieta's theorem (sum of the roots = `-p`, product of the roots = `q`) with focus on efficiency

# How to use it
It is a CLI app, so download the binary file for your platform from "Releases" and run in it in your terminal with `p` and `q` values for that quadratic equation as arguments.

For example, let's say you got the following equation: `x^2-28+96=0`, this is how you'd solve for roots using this script:
```bash
./vietta-solver -28 96
```

Change `vietta-solver` to name of the executable, so if you are on Windows for example, the command would be like this:
```shell
./vietta-solver.exe -28 96
```

# FAQs
- Why it is Viet**t**a and not Vieta solver?
    - I did not know that "Vieta" is written with one *t*
- Why did you build it?
    - In my 9th grade, Vieta's theorem was used very often, and it was too hard for me to guess numbers that produce a specific product and a specific sum as it is guesswork and not concise calculation. It turned out to be an amazing side project to practice my problem solving and Rust skills. This is quite a niche theorem, i could not find it on Wikipedia. You can find out more about it [here](https://www.miyklas.com.ua/p/algebra/8-klas/kvadratni-rivniannia-14001/teorema-viyeta-14020/re-dc539851-47e1-438e-a5ba-f2bcd52fd3df)
- Will this work if there is a coefficient for `x^2`?
    - No.

# Compiling
Firstly, you need to ensure that Rust toolchain is installed on your system, you can install it with [rustup](https://www.google.com/search?client=safari&rls=en&q=rustup&ie=UTF-8&oe=UTF-8)

Clone this repository and `cd` into it, run `cargo build --release` in the terminal from the repository folder. You will find the binary in `target/release` directory

# Limitations
This script has some technical limitations:
- Only integers are allowed, no floating point numbers
- Numbers flowing through the app can only be 32 bit integers (between -2147483648 and 2147483647)
    - If you need to process bigger numbers, you can do so by editing the source code, simply replace `i32` with `i64` or `i128` and compile the crate again