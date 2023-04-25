# termauth-rs
Terminal user interface with authentication over terminal applications. A lot of it 
is still in it's early phase. The application is best run on a terminal (full screen).

### Preview
Preview on the different parts of the application

- Landing UI (Welcome)

![image](https://user-images.githubusercontent.com/54986259/234174286-37447097-7053-4ab6-81bf-1c1a326a8a9c.png)

- Landing UI (Sign Up)

![image](https://user-images.githubusercontent.com/54986259/234174887-1e58e534-8286-4af0-bcac-539196a7ae6f.png)

- Logged UI (Section - fun user input)

![image](https://user-images.githubusercontent.com/54986259/234174547-b2df815e-af16-425b-891e-83568c64023b.png)


## Instruction
This application was build in `arch` linux os using `vim`. A lot of my test cases have come from within that environment so there's
no gurantee that it'll work on yours if our environments differ.

## Prerequisites
Ensure that you have `rust` installed on your operating system and you are able to run `cargo` commands. Cargo is a package manager for `rust`
that comes installed with the language. Windows has a nice setup of doing this although I did encounter some issues during the initial 
installation stages due to a library.

### Arch Linux Installation
Clone this repository to your machine.

```bash
cargo run
```

### Windows Installation (Not compatible.. yet)
As of this moment, windows is not compatible with the application as it requires a dependency on `libc` which I did not have time
to setup. At minimum my tests were only done on Windows 11. It may work for other versions / architecture of windows. Worth a try.

- Install through git
Clone the repo and run the following in the parent directory

```bash
cargo run
````

- Install through release tags
Download the latest release tag. Extract the folder and open a terminal from within the parent directory then run

```bash
cargo run
```

## References
The following links and references were used to help inspire and create this application:

- NIST Password Guidelines:
https://pages.nist.gov/800-63-3/sp800-63b.html#sec5
                    
- Normalization:
[crate normalization]        - https://docs.rs/unicode-normalization/latest/unicode_normalization/

- Normalization:
[unicode standard annex #15] - http://www.unicode.org/reports/tr15/

- Regex (swear words filtering)
[badwords filter repository] - https://github.com/mogade/badwords
                
- Restriction (swear words filtering)
[restriction]                - https://github.com/finnbear/rustrict/tree/master
                
- Rate Limiting:
[rate limiting]              - https://docs.rs/ratelimit/latest/ratelimit/struct.Ratelimiter.html


