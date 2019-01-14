import { WebClient } from "ruzzle";

const webClient = WebClient.new(
    "canvas",
    "environment",
    "object",
    "character",
    "world_0_bgm",
    "world_1_bgm",
);
webClient.render();

function render() {
    webClient.update();
    webClient.render();
    requestAnimationFrame(render);
}

render();