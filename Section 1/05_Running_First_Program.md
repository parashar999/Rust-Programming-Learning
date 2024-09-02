### 1. Compiling the file : when program is simple and contains only a single file.


![alt text](image.png)

**a. Enter ``` rustc file_name``` in the terminal.**
![alt text](image-1.png)
**b. It will create an executable file which can be executed,**

---

### 2. Complex Program using Multiple dependencies (Here Cargo is used)

Cargo is a Package Manager

**a. Create new rust Project using ``` cargo new Project_name ```**

![alt text](image-3.png)

Following Files and Folders will be created using this

![alt text](image-4.png)

1. Source folder will contain all the source files with ``.rs`` extension.
2. In src directory `main.rs` will be auto generated.
3. `Cargo.toml file` will conatin Information about Project regarding its `versions, edition, dependencies` it usings.

**b. To compile and run the code use `cargo run` int the terminal from root directory of project.**

1. It will compile and run the program in terminal.

Build
![alt text](image-5.png)

Execution
![alt text](image-6.png)

**b. To only Build the program use `cargo build` , it will only build the program will not execute it.**

![alt text](image-9.png)

**c. If you want to build only use `cargo build --release`**
![alt text](image-7.png)
![alt text](image-8.png)


**Note :-Optimization make code faster, While development phase `cargo build` is used, When handing over to the user `cargo build --release` is used.**