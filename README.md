# wanderways-api
Wanderways' API in rust

# How to install Diesel

## Linux

@TODO

## Windows

To install Diesel on windows *supposing you have rust and postgresql installed*, do the following :

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