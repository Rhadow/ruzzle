import { WebClient } from "ruzzle";

const webClient = WebClient.new("canvas", "environment", "object", "character");
webClient.render();

function render() {
    webClient.update();
    webClient.render();
    requestAnimationFrame(render);
}

render();
