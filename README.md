# Circle Of Fifths
A simple interactive [demo](cof.demanda.pt/) of the circle of fifths, where you can explore how a regular polygon can play notes.


https://github.com/RuiVarela/CircleOfFifths/assets/11543973/f7c3c564-addc-4e93-9d5b-bdb3b82c5f63


This projects was inspired by the great [AlgoMotion](https://www.youtube.com/watch?v=V0YH8M6C-VM) video.
My son loved the video and I wanted to show him how it works, so I decided to create this project.

It is implemented in Rust targeting WebAssembly, using the [wasm-pack]

# Development
```
# build the project
./build.sh

# run a local server
npx light-server -s pkg -p 8080
```

# Credits
- [AlgoMotion](https://www.youtube.com/watch?v=V0YH8M6C-VM)
- [wasm-pack](https://github.com/rustwasm/wasm-pack)

