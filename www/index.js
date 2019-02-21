import { WebClient } from "ruzzle";

function loadAssets() {
    const audioPromises = Array.from(document.querySelectorAll('.audio')).map(element => {
        element.volume = element.id.startsWith('bgm') ? 0.4 : 1;
        return new Promise((resolve) => {
            if (element.readyState > 3) {
                resolve();
            } else {
                const cb = () => {
                    element.removeEventListener('canplaythrough', cb);
                    resolve();
                };
                element.addEventListener('canplaythrough', cb);
                element.load();
            }
        });
    });
    const spritePromises = Array.from(document.querySelectorAll('.sprite')).map(element => {
        return new Promise((resolve) => {
            if (element.complete) {
                resolve();
            } else {
                const cb = () => {
                    element.removeEventListener('load', cb);
                    resolve();
                };
                element.addEventListener('load', cb);
            }
        });
    });
    const assetPromises = [].concat(audioPromises).concat(spritePromises);
    return Promise.all(assetPromises);
};


function createBindControllerEventHandler() {
    const canvasElement = document.getElementById("canvas");
    let lastHandlers = null;
    return (handlers) => {
        if (lastHandlers) {
            canvasElement.removeEventListener("mousedown", lastHandlers.mouseDownHandler);
            canvasElement.removeEventListener("mouseup", lastHandlers.mouseUpHandler);
            canvasElement.removeEventListener("mousemove", lastHandlers.mouseMoveHandler);
            window.removeEventListener("keydown", lastHandlers.keyDownHandler);
            window.removeEventListener("keyup", lastHandlers.keyUpHandler);
        }
        canvasElement.addEventListener("mousedown", handlers.mouseDownHandler);
        canvasElement.addEventListener("mouseup", handlers.mouseUpHandler);
        canvasElement.addEventListener("mousemove", handlers.mouseMoveHandler);
        window.addEventListener("keydown", handlers.keyDownHandler);
        window.addEventListener("keyup", handlers.keyUpHandler);
        lastHandlers = handlers;
    };
}

function createHandlers(webClient) {
    // NOTE: Make sure mouse position are correct after canvas scaled
    const scaleRatio = getComputedStyle(document.documentElement).getPropertyValue('--scale-ratio');
    return {
        mouseDownHandler: e => webClient.handle_mouse_down(e.offsetX / scaleRatio, e.offsetY / scaleRatio),
        mouseUpHandler: e => webClient.handle_mouse_up(e.offsetX / scaleRatio, e.offsetY / scaleRatio),
        mouseMoveHandler: e => webClient.handle_mouse_move(e.offsetX / scaleRatio, e.offsetY / scaleRatio),
        keyDownHandler: e => webClient.handle_keydown(e.key, performance.now()),
        keyUpHandler: e => webClient.handle_keyup(e.key),
    };
}

loadAssets().then(() => {
    const webClient = WebClient.new("canvas", {
        "sprite": [
            "ruzzle_object",
            "ruzzle_ui",
            "ruzzle_environment",
            "ruzzle_characters",
        ],
        "bgm": [
            "bgm_0",
            "bgm_level_selection",
        ],
        "sfx": [
            "sfx_rock_fall",
            "sfx_rock_move",
            "sfx_dead",
            "sfx_fanfare",
            "sfx_projecting",
        ]
    });
    const bindControllerEvents = createBindControllerEventHandler();
    bindControllerEvents(createHandlers(webClient));
    window.addEventListener('resize', () => { bindControllerEvents(createHandlers(webClient)); });
    webClient.render();
    function render() {
        webClient.update();
        webClient.render();
        requestAnimationFrame(render);
    }
    render();
});
