import __wbg_init from "./circle_of_fifths.js";
import { CircleOfFifths } from "./circle_of_fifths.js";

let wasm = await __wbg_init();

const cof = CircleOfFifths.new();
const startTime = new Date();

const renderLoop = () => {
    const endTime = new Date();
    const delta = (endTime - startTime) / 1000.0;

    const sides = document.getElementById("polygon").value;
    const speed = document.getElementById("speed").value;

    cof.tick(delta, sides, speed);

    requestAnimationFrame(renderLoop);
};

requestAnimationFrame(renderLoop);