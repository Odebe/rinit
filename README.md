```
 ____   ___  _   _ _____   _____ ___  _   _  ____ _   _   __  ____   __   ____    _    ____  ____    _    ____ _____ 
|  _ \ / _ \| \ | |_   _| |_   _/ _ \| | | |/ ___| | | | |  \/  \ \ / /  / ___|  / \  |  _ \| __ )  / \  / ___| ____|
| | | | | | |  \| | | |     | || | | | | | | |   | |_| | | |\/| |\ V /  | |  _  / _ \ | |_) |  _ \ / _ \| |  _|  _|  
| |_| | |_| | |\  | | |     | || |_| | |_| | |___|  _  | | |  | | | |   | |_| |/ ___ \|  _ <| |_) / ___ \ |_| | |___ 
|____/ \___/|_| \_| |_|     |_| \___/ \___/ \____|_| |_| |_|  |_| |_|    \____/_/   \_\_| \_\____/_/   \_\____|_____|
```

# Rinit
A toy Git implementation. Started for learning purposes.

## Implemented commands:
* init
* hash-object
* update-index
* write-tree

## Installation
`cargo build`

## Usage
```shell
./target/debug/rinit init
./target/debug/rinit hash-object -w -- Cargo.lock
./target/debug/rinit update-index --add --cacheinfo 100644 <object-hash> Cargo.lock
./target/debug/rinit write-tree
```

## Contributors

- [Mihail Odebe](https://github.com/Odebe) - creator and maintainer