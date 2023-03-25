# wanderways-api
Wanderways' API in rust

# Legacy

## Rejected ORM solution - Diesel

Rejection reason :

- Doesn't fit current needs with loose depencencies between entities
- Specifics SQL operators (group_concat et coalesce) not supported by defaults

In the end, way too much added work for specific needs on tables that are quite atomical.

### Linux

Linux installation is quite simple.

1. Install postgreSQL on your machine. 

```bash
sudo dnf install postgresql postgresql-server postgresql-devel # install development packages, i.e : lpq
sudo postgresql-setup --initdb --unit postgresql
```

2. Install diesel in the project 

```bash
cargo install diesel_cli --no-default-features --features postgres
```

3. Fix problems. If you are like me, you'll have a lot to do. Sources below.

Sources : 
- [PostgreSQL install Fedora](https://developer.fedoraproject.org/tech/database/postgresql/about.html)
- [`cannot find -lpq` error](https://github.com/rust-lang/rust/issues/25289)
- [Diesel doc](https://diesel.rs/guides/getting-started.html)
- [`FATAL:  Ident authentication failed for user "wanderways_user"`](https://serverfault.com/questions/406606/postgres-error-message-fatal-ident-authentication-failed-for-user)
- [Change authentication process from ident to password based (md5 <- vulnerability)](https://www.liquidweb.com/kb/change-postgresql-authentication-method-from-ident-to-md5/)

4. You may need to add cargo bin to path : `export PATH=home/$USER/.cargo/bin:$PATH`
5. Run setup `diesel setup`

### Windows

To install Diesel on windows *supposing you have rust and postgreSQL installed*, do the following :

`cargo install diesel_cli --no-default-features --features postgres`

Here, we specify a specific feature, postgres.

Once installed, you should be able to do everything showed in the [tutorial](https://diesel.rs/guides/getting-started.html).

But installing Diesel on windows can be troublesomes.

You may encounter the following problems : 

```
note: LINK : fatal error LNK1181: cannot open input file 'libpq.lib'

error: aborting due to previous error
error: failed to compile `diesel_cli v1.4.1`, intermediate artifacts can be found at `C:\Users\<user name here>\AppData\Local\Temp\cargo-installUU2DtT`

Caused by:
  could not compile `diesel_cli`.
```

To fix this issue : `setx PQ_LIB_DIR "C:\Program Files\PostgreSQL\YOUR_VERSION\lib"`

Then restart your computer.

Now you can run the command `cargo install diesel_cli --no-default-features --features postgres`

But, when you try to do `diesel setup`, you may have the error :
```
C:/Users/Utilisateur/.cargo/bin/diesel.exe: error while loading shared libraries: api-ms-win-crt-heap-l1-1-0.dll: cannot open shared object file: No such file or directory
```

To fix it, install the [C++ distribuable](https://www.smartftp.com/en-us/support/kb/2702) that will add the needed package to your computer.

You will then have to restart your computer once more.

But i did not succeed in doing so because of a dll that I cannot install (or more like I can install it, but then I would have to mess up with env path in windows and touching C: files, which i don't like for such simple tasks as to add a library).