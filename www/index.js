import { WebClient } from "ruzzle";

function loadAssets() {
    const audioPromises = Array.from(document.querySelectorAll('.audio')).map(element => {
        return new Promise((resolve) => {
            if (element.readyState > 3) {
                console.log(`${element.id} loaded`);
                resolve();
            } else {
                const cb = () => {
                    console.log(`${element.id} loaded`);
                    element.removeEventListener('canplaythrough', cb);
                    resolve();
                };
                element.addEventListener('canplaythrough', cb);
            }
        });
    });
    const spritePromises = Array.from(document.querySelectorAll('.sprite')).map(element => {
        return new Promise((resolve) => {
            if (element.complete) {
                console.log(`${element.id} loaded`);
                resolve();
            } else {
                const cb = () => {
                    console.log(`${element.id} loaded`);
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

loadAssets().then(() => {
    console.log('All assets loaded, starting game!');
    const webClient = WebClient.new("canvas", {
        "sprite": [
            "environment",
            "object",
            "character",
        ],
        "bgm": [
            "bgm_0",
            "bgm_1",
        ],
        "sfx": [
            "sfx_rock_fall",
            "sfx_rock_move",
            "sfx_dead",
            "sfx_fanfare",
        ]
    });
    webClient.bind_events();
    webClient.render();
    function render() {
        webClient.update();
        webClient.render();
        requestAnimationFrame(render);
    }
    render();
});
