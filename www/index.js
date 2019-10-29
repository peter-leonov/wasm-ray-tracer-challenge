import { init } from "./canvas";
import { memory } from "ray-tracer/ray_tracer_bg";
import * as wasm from "ray-tracer";

init(memory);
wasm.greet("Peter");
